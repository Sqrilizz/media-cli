use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: u64,
}

fn history_file() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let path = PathBuf::from(home).join(".cache/media-cli");
    fs::create_dir_all(&path)?;
    Ok(path.join("history.json"))
}

pub fn add(url: &str, title: &str) -> Result<()> {
    let mut entries = load()?;
    
    entries.retain(|e| e.url != url);
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    entries.insert(0, HistoryEntry {
        url: url.to_string(),
        title: title.to_string(),
        timestamp,
    });
    
    entries.truncate(50);
    
    let json = serde_json::to_string_pretty(&entries)?;
    fs::write(history_file()?, json)?;
    
    Ok(())
}

pub fn load() -> Result<Vec<HistoryEntry>> {
    let path = history_file()?;
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(path)?;
    let entries = serde_json::from_str(&content).unwrap_or_default();
    
    Ok(entries)
}

pub fn clear() -> Result<()> {
    let path = history_file()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
