use anyhow::{Context, Result};
use std::process::Command;

pub fn is_twitch_url(query: &str) -> bool {
    query.contains("twitch.tv/")
}

pub fn extract_channel(url: &str) -> String {
    if let Some(channel) = url.split("twitch.tv/").nth(1) {
        channel.split('/').next().unwrap_or(url).to_string()
    } else {
        url.to_string()
    }
}

pub fn get_stream_url(channel: &str, quality: Option<&str>) -> Result<String> {
    let quality_str = quality.unwrap_or("best");
    
    let output = Command::new("streamlink")
        .args([
            "--stream-url",
            &format!("https://twitch.tv/{}", channel),
            quality_str,
        ])
        .output()
        .context("Failed to launch streamlink. Install: pip install streamlink")?;

    if !output.status.success() {
        anyhow::bail!("Stream not found or offline");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn search_streams(query: &str) -> Result<Vec<(String, String)>> {
    let output = Command::new("streamlink")
        .args([
            "--twitch-api-header",
            "Client-ID=kimne78kx3ncx6brgo4mv6wki5h1ko",
            "--json",
        ])
        .output();

    if output.is_err() {
        return Ok(vec![(query.to_string(), format!("https://twitch.tv/{}", query))]);
    }

    Ok(vec![(query.to_string(), format!("https://twitch.tv/{}", query))])
}
