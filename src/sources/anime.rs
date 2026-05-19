use anyhow::{Context, Result};
use serde_json::Value;
use std::process::Command;

use super::MediaItem;
use crate::player::detect_terminal_vo;

const ALLANIME_API: &str = "https://api.allanime.day";
const ALLANIME_REFR: &str = "https://allmanga.to";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

fn allanime_query(variables: &str, query: &str) -> Result<Value> {
    let output = Command::new("curl")
        .args([
            "-e", ALLANIME_REFR,
            "-s",
            "-G",
            &format!("{}/api", ALLANIME_API),
            "--data-urlencode", &format!("variables={}", variables),
            "--data-urlencode", &format!("query={}", query),
            "-A", USER_AGENT,
        ])
        .output()
        .context("Failed to execute API request. Is curl installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("API request failed: {}", stderr);
    }

    let response = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&response)
        .context(format!("Failed to parse API response: {}", &response[..200.min(response.len())]))?;

    Ok(json)
}

pub fn search_anime(query: &str, mode: &str) -> Result<Vec<MediaItem>> {
    let search_gql = r#"query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name availableEpisodes __typename } }}"#;
    
    let variables = format!(
        r#"{{"search":{{"allowAdult":false,"allowUnknown":false,"query":"{}"}},"limit":40,"page":1,"translationType":"{}","countryOrigin":"ALL"}}"#,
        query, mode
    );

    let json = allanime_query(&variables, search_gql)?;

    let mut results = Vec::new();

    if let Some(edges) = json.pointer("/data/shows/edges").and_then(|v| v.as_array()) {
        for edge in edges {
            let id = edge.get("_id").and_then(|v| v.as_str()).unwrap_or_default();
            let name = edge.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            let ep_count = edge.pointer(&format!("/availableEpisodes/{}", mode))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            if !id.is_empty() && !name.is_empty() {
                let title = if ep_count > 0 {
                    format!("{} ({} episodes)", name, ep_count)
                } else {
                    name.to_string()
                };
                results.push(MediaItem {
                    title,
                    url: id.to_string(),
                    duration: None,
                });
            }
        }
    }

    Ok(results)
}

pub fn get_episodes(show_id: &str, mode: &str) -> Result<Vec<String>> {
    let episodes_gql = r#"query ($showId: String!) { show(_id: $showId) { _id availableEpisodesDetail }}"#;
    
    let variables = format!(r#"{{"showId":"{}"}}"#, show_id);

    let json = allanime_query(&variables, episodes_gql)?;

    let mut episodes = Vec::new();

    if let Some(detail) = json.pointer("/data/show/availableEpisodesDetail") {
        if let Some(eps) = detail.get(mode).and_then(|v| v.as_array()) {
            for ep in eps {
                if let Some(s) = ep.as_str() {
                    episodes.push(s.to_string());
                } else if let Some(n) = ep.as_f64() {
                    if n == n.floor() {
                        episodes.push(format!("{}", n as i64));
                    } else {
                        episodes.push(format!("{}", n));
                    }
                }
            }
        }
    }

    episodes.sort_by(|a, b| {
        a.parse::<f64>().unwrap_or(0.0)
            .partial_cmp(&b.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(episodes)
}

pub fn get_episode_sources(show_id: &str, episode: &str, mode: &str) -> Result<String> {
    let episode_gql = r#"query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) { episode( showId: $showId translationType: $translationType episodeString: $episodeString ) { episodeString sourceUrls }}"#;
    
    let variables = format!(
        r#"{{"showId":"{}","translationType":"{}","episodeString":"{}"}}"#,
        show_id, mode, episode
    );

    let output = Command::new("curl")
        .args([
            "-e", ALLANIME_REFR,
            "-s",
            "-G",
            &format!("{}/api", ALLANIME_API),
            "--data-urlencode", &format!("variables={}", variables),
            "--data-urlencode", &format!("query={}", episode_gql),
            "-A", USER_AGENT,
        ])
        .output()
        .context("Failed to fetch sources")?;

    if !output.status.success() {
        anyhow::bail!("Error fetching sources");
    }

    let response = String::from_utf8_lossy(&output.stdout);
    
    let output2 = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"echo '{}' | tr '{{}}' '\n' | sed 's|\\u002F|/|g;s|\\||g' | sed -nE 's|.*sourceUrl":"--([^"]*)".*sourceName":"([^"]*)".*|\2:\1|p'"#,
            response.replace('\'', "'\\''")
        ))
        .output()
        .context("Failed to parse sources")?;

    let parsed = String::from_utf8_lossy(&output2.stdout);
    
    Ok(parsed.to_string())
}

pub fn play_anime_episode(
    show_id: &str,
    episode: &str,
    mode: &str,
    player: Option<&str>,
    quality: Option<&str>,
    terminal: bool,
    proxy: Option<&str>,
) -> Result<()> {
    use super::anime_providers;
    
    println!("🎬 Fetching sources...");
    
    let sources = get_episode_sources(show_id, episode, mode)?;
    
    if sources.trim().is_empty() {
        anyhow::bail!("No sources found for episode {}", episode);
    }
    
    println!("\x1B[1;90m📡 Found {} sources\x1B[0m", sources.lines().filter(|l| !l.trim().is_empty()).count());
    
    let links = anime_providers::get_all_links(&sources)?;
    
    if links.is_empty() {
        anyhow::bail!("Failed to get video links");
    }
    
    let quality_str = quality.unwrap_or("best");
    let link = anime_providers::select_quality(&links, quality_str)?;
    
    println!("✓ Playing: {} ({})", link.quality, &link.url[..50.min(link.url.len())]);
    
    let player_cmd = player.unwrap_or("mpv");
    let mut cmd = std::process::Command::new(player_cmd);
    
    if let Some(proxy_url) = proxy {
        cmd.arg(format!("--http-proxy={}", proxy_url));
    }
    
    if let Some(referer) = &link.referer {
        cmd.arg(format!("--referrer={}", referer));
    }
    
    if let Some(subtitle) = &link.subtitle {
        cmd.arg(format!("--sub-file={}", subtitle));
    }
    
    if terminal {
        let vo = detect_terminal_vo();
        cmd.arg(format!("--vo={}", vo));
        cmd.arg("--really-quiet");
        cmd.arg("--force-window=no");
        cmd.arg("--hwdec=auto");
    }
    
    cmd.arg(&link.url);
    
    let status = cmd.status()
        .context(format!("Failed to launch {}", player_cmd))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }
    
    Ok(())
}
