pub mod youtube;
pub mod local;
pub mod twitch;
pub mod anime;
pub mod anime_providers;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub title: String,
    pub url: String,
    pub duration: Option<String>,
}

impl std::fmt::Display for MediaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dur) = &self.duration {
            write!(f, "{} [{}]", self.title, dur)
        } else {
            write!(f, "{}", self.title)
        }
    }
}
