use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn home_dir() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .or_else(|| {
            let drive = std::env::var_os("HOMEDRIVE")?;
            let path = std::env::var_os("HOMEPATH")?;
            Some(PathBuf::from(drive).join(path))
        })
        .context("Could not determine the user home directory")
}

pub fn cache_dir() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(path).join("media-cli"));
    }
    if let Some(path) = std::env::var_os("LOCALAPPDATA") {
        return Ok(PathBuf::from(path).join("media-cli"));
    }
    Ok(home_dir()?.join(".cache/media-cli"))
}

pub fn config_dir() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join("media-cli"));
    }
    if let Some(path) = std::env::var_os("APPDATA") {
        return Ok(PathBuf::from(path).join("media-cli"));
    }
    Ok(home_dir()?.join(".config/media-cli"))
}
