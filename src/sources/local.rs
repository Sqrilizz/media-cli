use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::MediaItem;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "webm", "avi", "mov"];

pub fn scan(path: Option<&str>) -> Result<Vec<MediaItem>> {
    let base_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        let home = std::env::var("HOME")?;
        PathBuf::from(home).join("Videos")
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
                let title = path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                results.push(MediaItem {
                    title,
                    url: path.to_string_lossy().to_string(),
                    duration: None,
                });
            }
        }
    }

    Ok(results)
}
