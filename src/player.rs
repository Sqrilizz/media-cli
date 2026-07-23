use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum Player {
    #[default]
    Mpv,
    Vlc,
}

impl Player {
    pub fn command(self) -> &'static str {
        match self {
            Self::Mpv => "mpv",
            Self::Vlc => "vlc",
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.command())
    }
}

impl std::str::FromStr for Player {
    type Err = String;
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "mpv" => Ok(Self::Mpv),
            "vlc" => Ok(Self::Vlc),
            _ => Err("player must be either 'mpv' or 'vlc'".to_owned()),
        }
    }
}

impl TryFrom<String> for Player {
    type Error = String;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        value.parse()
    }
}

fn is_audio_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "mp3" | "flac" | "ogg" | "opus" | "m4a" | "wav" | "wma" | "aac"
            )
        })
        .unwrap_or(false)
}

fn ytdl_format(quality: &str) -> String {
    match quality {
        "best" => "bestvideo+bestaudio/best".to_owned(),
        "worst" => "worstvideo+worstaudio/worst".to_owned(),
        value => {
            let height = value
                .chars()
                .take_while(|character| character.is_ascii_digit())
                .collect::<String>();
            if height.is_empty() {
                "bestvideo+bestaudio/best".to_owned()
            } else {
                format!("bestvideo[height<={height}]+bestaudio/best[height<={height}]")
            }
        }
    }
}

pub fn play(url: &str, player: Player) -> Result<()> {
    if player == Player::Mpv && is_audio_file(url) {
        let music_config = crate::config::MusicConfig::default();
        return play_audio(
            url,
            Path::new(url)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(url),
            &music_config,
        );
    }

    println!("▶ Launching {} ...", player);

    let status = Command::new(player.command())
        .arg(url)
        .status()
        .context(format!("Failed to launch {}. Is it installed?", player))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn detect_terminal_vo() -> &'static str {
    if std::env::var("KITTY_PID").is_ok() {
        return "kitty";
    }
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("kitty") {
            return "kitty";
        }
    }
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        if term_program.contains("kitty") {
            return "kitty";
        }
        if term_program.contains("foot") || term_program.contains("mlterm") {
            return "sixel";
        }
    }
    "tct"
}

pub fn play_youtube(
    url: &str,
    player: Player,
    quality: Option<&str>,
    terminal: bool,
) -> Result<()> {
    ensure_terminal_supported(player, terminal)?;
    println!("▶ Launching {} ...", player);
    let source = resolve_source(url, player, quality)?;
    let mut cmd = Command::new(player.command());

    if terminal {
        let vo = detect_terminal_vo();
        cmd.arg(format!("--vo={}", vo));
        cmd.arg("--really-quiet");
        cmd.arg("--force-window=no");
        cmd.arg("--hwdec=auto");
        cmd.arg("--profile=fast");
        cmd.arg("--cache=yes");
        cmd.arg("--demuxer-max-bytes=50M");

        if vo == "tct" {
            cmd.arg("--vo-tct-algo=half-blocks");
            cmd.arg("--vo-tct-width=80");
            cmd.arg("--vo-tct-height=25");
            cmd.arg("--vf=fps=10");
            if quality.is_none() {
                cmd.arg("--ytdl-format=bestvideo[height<=480]+bestaudio/best");
            }
        } else if quality.is_none() {
            cmd.arg("--ytdl-format=bestvideo[height<=720]+bestaudio/best");
        }
    }

    if player == Player::Mpv {
        if let Some(q) = quality {
            cmd.arg(format!("--ytdl-format={}", ytdl_format(q)));
        }
    }

    cmd.arg(source);

    let status = cmd
        .status()
        .context(format!("Failed to launch {}. Is it installed?", player))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn play_music(url: &str, title: &str, music_config: &crate::config::MusicConfig) -> Result<()> {
    play_audio(url, title, music_config)
}

fn play_audio(url: &str, title: &str, music_config: &crate::config::MusicConfig) -> Result<()> {
    let ipc = crate::mpv_ipc::MpvIpc::new();
    let af = "@meter:lavfi=[astats=metadata=1:reset=1:measure_perchannel=none:measure_overall=RMS_level+Peak_level],@spectrum:lavfi=[aspectralstats=measure=all]";

    let mut cmd = Command::new("mpv");
    cmd.arg("--no-video")
        .arg("--really-quiet")
        .arg("--no-input-terminal")
        .arg(format!("--input-ipc-server={}", ipc.path().display()))
        .arg(format!("--af={}", af))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

    cmd.arg(url);

    let mut child = cmd
        .spawn()
        .context("Failed to launch mpv. Is it installed?")?;
    if let Err(error) = ipc.wait_until_ready(&mut child) {
        let mut stderr_buf = String::new();
        if let Some(mut stderr) = child.stderr.take() {
            let _ = stderr.read_to_string(&mut stderr_buf);
        }
        let _ = child.kill();
        let _ = child.wait();
        if stderr_buf.trim().is_empty() {
            return Err(error);
        }
        anyhow::bail!("{} — mpv: {}", error, stderr_buf.trim());
    }
    crate::tui::run_music_player(title, &mut child, &ipc, music_config)
}

pub fn play_stream(url: &str, player: Player, quality: Option<&str>, terminal: bool) -> Result<()> {
    play_youtube(url, player, quality, terminal)
}

pub fn play_stream_terminal(url: &str, quality: Option<&str>) -> Result<()> {
    play_stream(url, Player::Mpv, quality, true)
}

pub fn ensure_terminal_supported(player: Player, terminal: bool) -> Result<()> {
    if terminal && player == Player::Vlc {
        anyhow::bail!("Terminal rendering requires mpv.");
    }
    Ok(())
}

pub fn ensure_music_supported(player: Player) -> Result<()> {
    if player == Player::Vlc {
        anyhow::bail!("Music Deck requires mpv.");
    }
    Ok(())
}

pub fn resolve_source(url: &str, player: Player, quality: Option<&str>) -> Result<String> {
    if player == Player::Mpv {
        return Ok(url.to_owned());
    }
    let format = quality.map(vlc_format).unwrap_or_else(|| "best".to_owned());
    let output = Command::new("yt-dlp")
        .args(["--no-playlist", "--no-warnings", "-g", "-f", &format, url])
        .output()
        .context("Failed to run yt-dlp. Install it to play remote media with VLC.")?;
    if !output.status.success() {
        anyhow::bail!(
            "yt-dlp could not resolve a stream for VLC: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let source = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned();
    if source.is_empty() {
        anyhow::bail!("yt-dlp returned no playable stream for VLC");
    }
    Ok(source)
}

fn vlc_format(quality: &str) -> String {
    let height = quality
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    if height.is_empty() || quality.eq_ignore_ascii_case("best") {
        "best".to_owned()
    } else if quality.eq_ignore_ascii_case("worst") {
        "worst".to_owned()
    } else {
        format!("best[height<={height}]/best")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_local_audio_extensions_case_insensitively() {
        assert!(is_audio_file("/music/track.FLAC"));
        assert!(is_audio_file("song.mp3"));
        assert!(!is_audio_file("movie.mkv"));
    }

    #[test]
    fn normalizes_stream_quality_for_ytdl() {
        assert!(ytdl_format("720p60").contains("height<=720"));
        assert_eq!(ytdl_format("best"), "bestvideo+bestaudio/best");
        assert_eq!(ytdl_format("audio_only"), "bestvideo+bestaudio/best");
        assert_eq!(vlc_format("720p"), "best[height<=720]/best");
    }

    #[test]
    fn accepts_only_supported_players() {
        assert_eq!("mpv".parse::<Player>().unwrap(), Player::Mpv);
        assert_eq!("VLC".parse::<Player>().unwrap(), Player::Vlc);
        assert!("mplayer".parse::<Player>().is_err());
    }

    #[test]
    fn rejects_mpv_only_modes_for_vlc() {
        assert_eq!(
            ensure_terminal_supported(Player::Vlc, true)
                .unwrap_err()
                .to_string(),
            "Terminal rendering requires mpv."
        );
        assert_eq!(
            ensure_music_supported(Player::Vlc).unwrap_err().to_string(),
            "Music Deck requires mpv."
        );
    }
}
