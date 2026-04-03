use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::Command;

use super::MediaItem;

pub fn is_youtube_url(query: &str) -> bool {
    query.contains("youtube.com/") || query.contains("youtu.be/")
}

#[derive(Debug, Deserialize)]
struct YtDlpEntry {
    title: String,
    id: String,
    duration_string: Option<String>,
}

pub fn search(query: &str) -> Result<Vec<MediaItem>> {
    let output = Command::new("yt-dlp")
        .args([
            "--dump-json",
            "--flat-playlist",
            "--default-search", "ytsearch10",
            query,
        ])
        .output()
        .context("Failed to launch yt-dlp. Install: pip install yt-dlp")?;

    if !output.status.success() {
        anyhow::bail!("yt-dlp error: {}", String::from_utf8_lossy(&output.stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        if let Ok(entry) = serde_json::from_str::<YtDlpEntry>(line) {
            results.push(MediaItem {
                title: entry.title,
                url: format!("https://youtube.com/watch?v={}", entry.id),
                duration: entry.duration_string,
            });
        }
    }

    Ok(results)
}
