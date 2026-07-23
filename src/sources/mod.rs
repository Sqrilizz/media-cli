pub mod anime;
pub mod anime_providers;
pub mod local;
pub mod twitch;
pub mod youtube;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub title: String,
    pub url: String,
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episode: Option<EpisodeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeRef {
    pub show_id: String,
    pub episode: String,
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
