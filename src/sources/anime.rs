use anyhow::{Context, Result};
use std::process::Command;

use super::MediaItem;

const ALLANIME_API: &str = "https://api.allanime.day";
const ALLANIME_BASE: &str = "allanime.day";
const ALLANIME_REFR: &str = "https://allmanga.to";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

pub fn search_anime(query: &str, mode: &str) -> Result<Vec<MediaItem>> {
    let search_gql = r#"query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name availableEpisodes __typename } }}"#;
    
    let variables = format!(
        r#"{{"search":{{"allowAdult":false,"allowUnknown":false,"query":"{}"}},"limit":40,"page":1,"translationType":"{}","countryOrigin":"ALL"}}"#,
        query, mode
    );

    let output = Command::new("curl")
        .args([
            "-e", ALLANIME_REFR,
            "-s",
            "-G",
            &format!("{}/api", ALLANIME_API),
            "--data-urlencode", &format!("variables={}", variables),
            "--data-urlencode", &format!("query={}", search_gql),
            "-A", USER_AGENT,
        ])
        .output()
        .context("Failed to execute search")?;

    if !output.status.success() {
        anyhow::bail!("Search error");
    }

    let response = String::from_utf8_lossy(&output.stdout);
    
    let output2 = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"echo '{}' | sed 's|Show|\n|g' | sed -nE 's|.*_id":"([^"]*)",.*name":"(.+)",.*{}":([1-9][^,]*).*|\1\t\2 (\3 episodes)|p' | sed 's/\\"//g'"#,
            response.replace('\'', "'\\''"),
            mode
        ))
        .output()
        .context("Failed to parse results")?;

    let parsed = String::from_utf8_lossy(&output2.stdout);
    
    let mut results = Vec::new();
    for line in parsed.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            results.push(MediaItem {
                title: parts[1].to_string(),
                url: parts[0].to_string(),
                duration: None,
            });
        }
    }

    Ok(results)
}

pub fn get_episodes(show_id: &str, mode: &str) -> Result<Vec<String>> {
    let episodes_gql = r#"query ($showId: String!) { show(_id: $showId) { _id availableEpisodesDetail }}"#;
    
    let variables = format!(r#"{{"showId":"{}"}}"#, show_id);

    let output = Command::new("curl")
        .args([
            "-e", ALLANIME_REFR,
            "-s",
            "-G",
            &format!("{}/api", ALLANIME_API),
            "--data-urlencode", &format!("variables={}", variables),
            "--data-urlencode", &format!("query={}", episodes_gql),
            "-A", USER_AGENT,
        ])
        .output()
        .context("Failed to fetch episode list")?;

    if !output.status.success() {
        anyhow::bail!("Error fetching episodes");
    }

    let response = String::from_utf8_lossy(&output.stdout);
    
    let output2 = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"echo '{}' | sed -nE 's|.*"{}":(\[[0-9.,"]*\]).*|\1|p' | sed 's|[][]||g; s|,|\n|g; s|"||g' | grep -E '^[0-9.]+$' | sort -n"#,
            response.replace('\'', "'\\''"),
            mode
        ))
        .output()
        .context("Failed to parse episodes")?;

    let parsed = String::from_utf8_lossy(&output2.stdout);
    
    let episodes: Vec<String> = parsed
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

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
        cmd.arg("--vo=kitty");
        cmd.arg("--quiet");
        cmd.arg("--really-quiet");
    }
    
    cmd.arg(&link.url);
    
    let status = cmd.status()
        .context(format!("Failed to launch {}", player_cmd))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }
    
    Ok(())
}
