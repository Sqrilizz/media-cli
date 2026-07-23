use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"# media-cli configuration
# Location: ~/.config/media-cli/config.toml

# Default media player for video/streams. Audio deck uses mpv for IPC controls.
player = "mpv"

# Default video quality: "480p", "720p", "1080p", "best", etc.
quality = "best"

# Play videos inline in compatible terminals by default.
terminal = false

# Default directory for `media-cli file` and the Local files menu.
# local_dir = "~/Videos"

# Default anime translation mode: "sub" or "dub".
anime_mode = "sub"

[music]
# Enable the in-app music deck visualizer.
visualizer = true

# Visualizer style: "mirror", "bars", or "wave".
visualizer_style = "bars"

# Visualizer sensitivity. 1.0 is neutral; try 0.7-1.8.
sensitivity = 1.15
"#;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub player: crate::player::Player,
    pub quality: String,
    pub terminal: bool,
    pub local_dir: Option<String>,
    pub anime_mode: String,
    pub music: MusicConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MusicConfig {
    pub visualizer: bool,
    pub visualizer_style: VisualizerStyle,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VisualizerStyle {
    Mirror,
    Bars,
    Wave,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player: crate::player::Player::Mpv,
            quality: "best".to_owned(),
            terminal: false,
            local_dir: None,
            anime_mode: "sub".to_owned(),
            music: MusicConfig::default(),
        }
    }
}

impl Default for MusicConfig {
    fn default() -> Self {
        Self {
            visualizer: true,
            visualizer_style: VisualizerStyle::Bars,
            sensitivity: 1.15,
        }
    }
}

pub fn config_file() -> Result<PathBuf> {
    Ok(crate::paths::config_dir()?.join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_file()?;
    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let mut config: Config = toml::from_str(&content)
        .with_context(|| format!("Invalid config file: {}", path.display()))?;
    config.normalize();
    Ok(config)
}

pub fn ensure_exists() -> Result<PathBuf> {
    let path = config_file()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        fs::write(&path, DEFAULT_CONFIG)?;
    }
    Ok(path)
}

pub fn default_config_text() -> &'static str {
    DEFAULT_CONFIG
}

impl Config {
    fn normalize(&mut self) {
        if self.quality.trim().is_empty() {
            self.quality = "best".to_owned();
        }
        if self.anime_mode.trim().is_empty() {
            self.anime_mode = "sub".to_owned();
        }
        self.music.sensitivity = self.music.sensitivity.clamp(0.2, 3.0);
    }

    pub fn quality_arg(&self) -> Option<&str> {
        let quality = self.quality.trim();
        if quality.is_empty() || quality.eq_ignore_ascii_case("best") {
            None
        } else {
            Some(quality)
        }
    }
}
