use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: u64,
    #[serde(default)]
    pub episode: Option<crate::sources::EpisodeRef>,
}

fn history_file() -> Result<std::path::PathBuf> {
    let path = crate::paths::cache_dir()?;
    fs::create_dir_all(&path)?;
    Ok(path.join("history.json"))
}

pub fn add(url: &str, title: &str) -> Result<()> {
    add_item(&crate::sources::MediaItem {
        url: url.to_owned(),
        title: title.to_owned(),
        duration: None,
        episode: None,
    })
}

pub fn add_item(item: &crate::sources::MediaItem) -> Result<()> {
    let mut entries = load()?;

    entries.retain(|entry| entry.url != item.url || entry.episode != item.episode);

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    entries.insert(
        0,
        HistoryEntry {
            url: item.url.clone(),
            title: item.title.clone(),
            timestamp,
            episode: item.episode.clone(),
        },
    );

    entries.truncate(50);

    save_to(&history_file()?, &entries)?;

    Ok(())
}

pub fn load() -> Result<Vec<HistoryEntry>> {
    let path = history_file()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    load_from(&path)
}

pub fn clear() -> Result<()> {
    let path = history_file()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn load_from(path: &std::path::Path) -> Result<Vec<HistoryEntry>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read history: {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("History is malformed and was preserved: {}", path.display()))
}

fn save_to(path: &std::path::Path, entries: &[HistoryEntry]) -> Result<()> {
    let parent = path
        .parent()
        .context("History path has no parent directory")?;
    let temporary = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("history"),
        std::process::id()
    ));
    let json = serde_json::to_vec_pretty(entries)?;
    fs::write(&temporary, json)
        .with_context(|| format!("Failed to write temporary history: {}", temporary.display()))?;
    fs::rename(&temporary, path)
        .with_context(|| format!("Failed to replace history: {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("media-cli-history-{name}-{}", std::process::id()))
    }

    #[test]
    fn atomic_save_replaces_history() {
        let path = temp_path("save");
        let entries = vec![HistoryEntry {
            url: "u".into(),
            title: "t".into(),
            timestamp: 1,
            episode: None,
        }];
        save_to(&path, &entries).unwrap();
        assert_eq!(load_from(&path).unwrap()[0].title, "t");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn malformed_history_is_not_discarded() {
        let path = temp_path("malformed");
        fs::write(&path, "not json").unwrap();
        assert!(load_from(&path).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "not json");
        fs::remove_file(path).unwrap();
    }
}
