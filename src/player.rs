use anyhow::{Context, Result};
use std::process::Command;

pub fn play(url: &str, player: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");

    println!("▶ Launching {} ...", player_cmd);

    let status = Command::new(player_cmd)
        .arg(url)
        .status()
        .context(format!("Failed to launch {}. Install: sudo apt install {}", player_cmd, player_cmd))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn play_youtube(url: &str, player: Option<&str>, quality: Option<&str>, terminal: bool, proxy: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");

    println!("▶ Launching {} ...", player_cmd);

    let mut cmd = Command::new(player_cmd);
    
    if let Some(proxy_url) = proxy {
        cmd.arg(format!("--ytdl-raw-options=proxy={}", proxy_url));
    }
    
    if terminal {
        cmd.arg("--vo=kitty");
        cmd.arg("--quiet");
        cmd.arg("--really-quiet");
        cmd.arg("--hwdec=auto");
        cmd.arg("--profile=fast");
        cmd.arg("--cache=yes");
        cmd.arg("--demuxer-max-bytes=50M");
        
        if quality.is_none() {
            cmd.arg("--ytdl-format=bestvideo[height<=720]+bestaudio/best");
        }
    }
    
    if let Some(q) = quality {
        cmd.arg(format!("--ytdl-format=bestvideo[height<={}]+bestaudio/best", q));
    }
    
    cmd.arg(url);

    let status = cmd.status()
        .context(format!("Failed to launch {}. Install: sudo apt install {}", player_cmd, player_cmd))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}

pub fn play_stream(url: &str, player: Option<&str>, quality: Option<&str>, terminal: bool, _proxy: Option<&str>) -> Result<()> {
    let player_cmd = player.unwrap_or("mpv");
    let quality_str = quality.unwrap_or("best");

    println!("▶ Launching streamlink...");

    let mut cmd = Command::new("streamlink");
    
    cmd.env_remove("HTTP_PROXY");
    cmd.env_remove("HTTPS_PROXY");
    cmd.env_remove("http_proxy");
    cmd.env_remove("https_proxy");
    
    cmd.arg("--player").arg(player_cmd);
    cmd.arg("--player-no-close");
    
    if terminal {
        cmd.arg("--player-args");
        cmd.arg("--vo=kitty --quiet --really-quiet --hwdec=auto --profile=fast --no-cache --force-seekable=no");
    } else {
        cmd.arg("--player-args");
        cmd.arg("--no-cache --force-seekable=no");
    }
    
    cmd.arg(url);
    cmd.arg(quality_str);

    let status = cmd.status()
        .context("Failed to launch streamlink. Install: pip install streamlink")?;

    if !status.success() {
        anyhow::bail!("Stream not found or exited with error");
    }

    Ok(())
}

pub fn play_stream_terminal(url: &str, quality: Option<&str>, _proxy: Option<&str>) -> Result<()> {
    let quality_str = quality.unwrap_or("best");

    println!("▶ Launching streamlink...");

    let mut cmd = Command::new("streamlink");
    
    cmd.env_remove("HTTP_PROXY");
    cmd.env_remove("HTTPS_PROXY");
    cmd.env_remove("http_proxy");
    cmd.env_remove("https_proxy");
    
    cmd.arg("--player").arg("mpv");
    cmd.arg("--player-args");
    cmd.arg("--vo=kitty");
    cmd.arg("--player-args");
    cmd.arg("--quiet");
    cmd.arg("--player-args");
    cmd.arg("--really-quiet");
    cmd.arg("--player-args");
    cmd.arg("--hwdec=auto");
    cmd.arg("--player-args");
    cmd.arg("--profile=fast");
    cmd.arg("--player-args");
    cmd.arg("--no-cache");
    cmd.arg("--player-args");
    cmd.arg("--force-seekable=no");
    cmd.arg(url);
    cmd.arg(quality_str);

    let status = cmd.status()
        .context("Failed to launch streamlink. Install: pip install streamlink")?;

    if !status.success() {
        anyhow::bail!("Stream not found or exited with error");
    }

    Ok(())
}
