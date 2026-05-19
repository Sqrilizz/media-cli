use anyhow::{Context, Result};
use std::process::Command;

pub fn play(url: &str, player: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");

    println!("▶ Launching {} ...", player_cmd);

    let status = Command::new(player_cmd)
        .arg(url)
        .status()
        .context(format!("Failed to launch {}. Is it installed?", player_cmd))?;

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

pub fn play_youtube(url: &str, player: Option<&str>, quality: Option<&str>, terminal: bool, proxy: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");

    println!("▶ Launching {} ...", player_cmd);

    let mut cmd = Command::new(player_cmd);
    
    if let Some(proxy_url) = proxy {
        cmd.arg(format!("--ytdl-raw-options=proxy={}", proxy_url));
    }
    
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
            cmd.arg("--fps=10");
            if quality.is_none() {
                cmd.arg("--ytdl-format=bestvideo[height<=480]+bestaudio/best");
            }
        } else if quality.is_none() {
            cmd.arg("--ytdl-format=bestvideo[height<=720]+bestaudio/best");
        }
    }
    
    if let Some(q) = quality {
        cmd.arg(format!("--ytdl-format=bestvideo[height<={}]+bestaudio/best", q));
    }
    
    cmd.arg(url);

    let status = cmd.status()
        .context(format!("Failed to launch {}. Is it installed?", player_cmd))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn play_music(url: &str, title: &str, proxy: Option<&str>) -> Result<()> {
    println!("\x1B[1;32m♪ Playing:\x1B[0m {}", title);

    let mut cmd = Command::new("mpv");
    cmd.arg("--no-video")
        .arg("--term-osd-bar")
        .arg("--msg-level=all=status")
        .arg("--term-playing-msg=\x1B[1;36m▶ ${media-title}\x1B[0m");

    if let Some(proxy_url) = proxy {
        cmd.arg(format!("--ytdl-raw-options=proxy={}", proxy_url));
    }

    cmd.arg(url);

    let status = cmd.status()
        .context("Failed to launch mpv. Is it installed?")?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn play_stream(url: &str, player: Option<&str>, quality: Option<&str>, terminal: bool, proxy: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");
    let quality_str = quality.unwrap_or("best");

    if terminal {
        println!("▶ Getting stream URL...");

        let mut sl_cmd = Command::new("streamlink");
        sl_cmd.env_remove("HTTP_PROXY");
        sl_cmd.env_remove("HTTPS_PROXY");
        sl_cmd.env_remove("http_proxy");
        sl_cmd.env_remove("https_proxy");
        sl_cmd.arg("--stream-url");
        if let Some(proxy_url) = proxy {
            sl_cmd.arg("--http-proxy").arg(proxy_url);
        }
        sl_cmd.arg(url).arg(quality_str);

        let output = sl_cmd.output()
            .context("Failed to launch streamlink. Install: https://streamlink.github.io/install.html")?;

        if !output.status.success() {
            anyhow::bail!("Stream not found or offline");
        }

        let stream_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stream_url.is_empty() {
            anyhow::bail!("Could not get stream URL");
        }

        println!("▶ Launching {} in terminal...", player_cmd);

        let vo = detect_terminal_vo();
        let mut cmd = Command::new(player_cmd);
        cmd.arg(format!("--vo={}", vo));
        cmd.arg("--really-quiet");
        cmd.arg("--force-window=no");
        cmd.arg("--hwdec=auto");
        cmd.arg("--profile=fast");
        cmd.arg("--no-cache");
        cmd.arg("--force-seekable=no");
        cmd.arg(&stream_url);

        let status = cmd.status()
            .context(format!("Failed to launch {}. Is it installed?", player_cmd))?;

        if !status.success() {
            anyhow::bail!("Player exited with error");
        }
    } else {
        println!("▶ Launching streamlink...");

        let mut cmd = Command::new("streamlink");
        cmd.env_remove("HTTP_PROXY");
        cmd.env_remove("HTTPS_PROXY");
        cmd.env_remove("http_proxy");
        cmd.env_remove("https_proxy");
        cmd.arg("--player").arg(player_cmd);
        cmd.arg("--player-no-close");
        cmd.arg("--player-args");
        cmd.arg("--no-cache --force-seekable=no");

        if let Some(proxy_url) = proxy {
            cmd.arg("--http-proxy").arg(proxy_url);
        }

        cmd.arg(url).arg(quality_str);

        let status = cmd.status()
            .context("Failed to launch streamlink. Install: https://streamlink.github.io/install.html")?;

        if !status.success() {
            anyhow::bail!("Stream not found or exited with error");
        }
    }

    Ok(())
}

pub fn play_stream_terminal(url: &str, quality: Option<&str>, proxy: Option<&str>) -> Result<()> {
    play_stream(url, Some("mpv"), quality, true, proxy)
}
