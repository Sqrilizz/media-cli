use aes::Aes256;
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use ctr::cipher::{KeyIvInit, StreamCipher};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::MediaItem;
use crate::player::{detect_terminal_vo, ensure_terminal_supported, Player};

const ALLANIME_API: &str = "https://api.allanime.day";
const ALLANIME_REFR: &str = "https://youtu-chan.com";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:150.0) Gecko/20100101 Firefox/150.0";
const ALLANIME_SECRET: &str = "Xot36i3lK3:v1";

fn parse_response(bytes: &[u8]) -> Result<Value> {
    let value: Value = serde_json::from_slice(bytes).context("Invalid AllAnime response")?;
    if let Some(message) = value.pointer("/errors/0/message").and_then(Value::as_str) {
        anyhow::bail!("AllAnime API error: {}", message);
    }
    let Some(payload) = value.get("tobeparsed").and_then(Value::as_str) else {
        return Ok(value);
    };
    let encrypted = STANDARD
        .decode(payload)
        .context("Invalid encrypted response")?;
    if encrypted.len() < 29 {
        anyhow::bail!("Encrypted response is too short");
    }
    let key = Sha256::digest(ALLANIME_SECRET.as_bytes());
    let mut iv = [0_u8; 16];
    iv[..12].copy_from_slice(&encrypted[1..13]);
    iv[15] = 2;
    let mut plaintext = encrypted[13..encrypted.len() - 16].to_vec();
    let mut cipher = ctr::Ctr128BE::<Aes256>::new(&key, &iv.into());
    cipher.apply_keystream(&mut plaintext);
    serde_json::from_slice(&plaintext).context("Invalid decrypted response")
}

fn allanime_query(variables: Value, query: &str) -> Result<Value> {
    let request = serde_json::json!({"variables": variables, "query": query});
    let response = crate::http::post_json(
        &format!("{}/api", ALLANIME_API),
        &request,
        &[
            ("Referer", ALLANIME_REFR),
            ("User-Agent", USER_AGENT),
            ("Content-Type", "application/json"),
        ],
    )?;
    parse_response(&response)
}

pub fn search_anime(query: &str, mode: &str) -> Result<Vec<MediaItem>> {
    let search_gql = r#"query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name availableEpisodes __typename } }}"#;

    let variables = serde_json::json!({
        "search": {"allowAdult": false, "allowUnknown": false, "query": query},
        "limit": 40,
        "page": 1,
        "translationType": mode,
        "countryOrigin": "ALL"
    });

    let json = allanime_query(variables, search_gql)?;

    let mut results = Vec::new();

    if let Some(edges) = json.pointer("/data/shows/edges").and_then(|v| v.as_array()) {
        for edge in edges {
            let id = edge.get("_id").and_then(|v| v.as_str()).unwrap_or_default();
            let name = edge
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let ep_count = edge
                .pointer(&format!("/availableEpisodes/{}", mode))
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
                    episode: None,
                });
            }
        }
    }

    Ok(results)
}

pub fn get_episodes(show_id: &str, mode: &str) -> Result<Vec<String>> {
    let episodes_gql =
        r#"query ($showId: String!) { show(_id: $showId) { _id availableEpisodesDetail }}"#;

    let variables = serde_json::json!({"showId": show_id});

    let json = allanime_query(variables, episodes_gql)?;

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
        a.parse::<f64>()
            .unwrap_or(0.0)
            .partial_cmp(&b.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(episodes)
}

pub fn get_episode_sources(show_id: &str, episode: &str, mode: &str) -> Result<String> {
    let episode_gql = r#"query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) { episode(showId: $showId translationType: $translationType episodeString: $episodeString) { episodeString sourceUrls }}"#;
    let variables = serde_json::json!({
        "showId": show_id,
        "translationType": mode,
        "episodeString": episode
    });
    let query_variables = variables.to_string();
    let extensions = serde_json::json!({
        "persistedQuery": {
            "version": 1,
            "sha256Hash": "d405d0edd690624b66baba3068e0edc3ac90f1597d898a1ec8db4e5c43c00fec"
        }
    })
    .to_string();

    let bytes = crate::http::get_query(
        &format!("{}/api", ALLANIME_API),
        &[("variables", &query_variables), ("extensions", &extensions)],
        &[
            ("Referer", ALLANIME_REFR),
            ("Origin", ALLANIME_REFR),
            ("User-Agent", USER_AGENT),
        ],
    )?;
    let response = match parse_response(&bytes) {
        Ok(response) if response.pointer("/data/episode/sourceUrls").is_some() => response,
        _ => allanime_query(variables, episode_gql)?,
    };
    let sources = response
        .pointer("/data/episode/sourceUrls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|source| {
            let source_url = source.get("sourceUrl")?.as_str()?;
            let name = source.get("sourceName")?.as_str()?;
            Some(format!("{}:{}", name, source_url))
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(sources)
}

pub fn play_anime_episode(
    show_id: &str,
    episode: &str,
    mode: &str,
    player: Player,
    quality: Option<&str>,
    terminal: bool,
) -> Result<()> {
    use super::anime_providers;

    println!("🎬 Fetching sources...");

    let sources = get_episode_sources(show_id, episode, mode)?;

    if sources.trim().is_empty() {
        anyhow::bail!("No sources found for episode {}", episode);
    }

    println!(
        "\x1B[1;90m📡 Found {} sources\x1B[0m",
        sources.lines().filter(|l| !l.trim().is_empty()).count()
    );

    let links = anime_providers::get_all_links(&sources)?;

    if links.is_empty() {
        anyhow::bail!("Failed to get video links");
    }

    let quality_str = quality.unwrap_or("best");
    let link = anime_providers::select_quality(&links, quality_str)?;

    println!(
        "✓ Playing: {} ({})",
        link.quality,
        &link.url[..50.min(link.url.len())]
    );

    ensure_terminal_supported(player, terminal)?;
    let mut cmd = std::process::Command::new(player.command());

    if let Some(referer) = &link.referer {
        if player == Player::Mpv {
            cmd.arg(format!("--referrer={}", referer));
        } else {
            cmd.arg(format!("--http-referrer={}", referer));
        }
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

    let status = cmd
        .status()
        .context(format!("Failed to launch {}", player))?;

    if !status.success() {
        anyhow::bail!("Player exited with error");
    }

    Ok(())
}
