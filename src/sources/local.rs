use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::MediaItem;

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "webm", "avi", "mov", "flv", "wmv", "m4v", "ts", "m2ts", "mpg", "mpeg", "3gp",
    "ogv", "vob", "mp3", "flac", "ogg", "opus", "m4a", "wav", "wma", "aac",
];

fn expand_path(path: &str) -> Result<PathBuf> {
    if path == "~" {
        return crate::paths::home_dir();
    }
    if let Some(rest) = path.strip_prefix("~/") {
        return Ok(crate::paths::home_dir()?.join(rest));
    }
    Ok(Path::new(path).to_path_buf())
}

pub fn scan(path: Option<&str>) -> Result<Vec<MediaItem>> {
    let base_path = if let Some(p) = path {
        expand_path(p)?
    } else {
        crate::paths::home_dir()?.join("Videos")
    };

    if !base_path.exists() {
        anyhow::bail!("Path does not exist: {}", base_path.display());
    }

    let mut results = Vec::new();

    for entry in WalkDir::new(&base_path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if VIDEO_EXTENSIONS.contains(&ext_str.as_str()) {
                let Some(file_name) = path.file_name() else {
                    continue;
                };
                let title = file_name.to_string_lossy().to_string();

                results.push(MediaItem {
                    title,
                    url: path.to_string_lossy().to_string(),
                    duration: None,
                    episode: None,
                });
            }
        }
    }

    results.sort_by(|left, right| left.url.cmp(&right.url));
    let duplicate_titles =
        results
            .iter()
            .fold(std::collections::HashMap::new(), |mut counts, item| {
                *counts.entry(item.title.clone()).or_insert(0_usize) += 1;
                counts
            });
    for item in &mut results {
        if duplicate_titles
            .get(&item.title)
            .copied()
            .unwrap_or_default()
            > 1
        {
            item.title = format!("{} — {}", item.title, item.url);
        }
    }
    Ok(results)
}
