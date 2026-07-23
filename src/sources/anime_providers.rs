use anyhow::{Context, Result};
use serde_json::Value;

const ALLANIME_BASE: &str = "allanime.day";
const ALLANIME_REFR: &str = "https://youtu-chan.com";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:150.0) Gecko/20100101 Firefox/150.0";

pub struct VideoLink {
    pub quality: String,
    pub url: String,
    pub referer: Option<String>,
    pub subtitle: Option<String>,
}

fn decode_provider_id(encoded: &str) -> String {
    if encoded.len() % 2 != 0 {
        return String::new();
    }

    let decoded = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            std::str::from_utf8(pair)
                .ok()
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        })
        .collect::<Option<Vec<_>>>();

    decoded
        .and_then(|bytes| {
            String::from_utf8(bytes.into_iter().map(|byte| byte ^ 0x38).collect()).ok()
        })
        .map(|value| value.replace("/clock", "/clock.json"))
        .unwrap_or_default()
}

fn find_string(value: &Value, key: &str) -> Option<String> {
    match value {
        Value::Object(object) => {
            if let Some(value) = object.get(key).and_then(Value::as_str) {
                return Some(value.to_owned());
            }
            object.values().find_map(|value| find_string(value, key))
        }
        Value::Array(values) => values.iter().find_map(|value| find_string(value, key)),
        _ => None,
    }
}

fn collect_direct_links(value: &Value, links: &mut Vec<(String, String)>) {
    match value {
        Value::Object(object) => {
            if let Some(url) = object.get("link").and_then(Value::as_str) {
                let quality = object
                    .get("resolutionStr")
                    .and_then(Value::as_str)
                    .unwrap_or("best");
                links.push((quality.to_owned(), url.to_owned()));
            } else if object.get("hardsub_lang").and_then(Value::as_str) == Some("en-US") {
                if let Some(url) = object.get("url").and_then(Value::as_str) {
                    links.push(("best".to_owned(), url.to_owned()));
                }
            }
            object
                .values()
                .for_each(|value| collect_direct_links(value, links));
        }
        Value::Array(values) => values
            .iter()
            .for_each(|value| collect_direct_links(value, links)),
        _ => {}
    }
}

fn find_english_subtitle(value: &Value) -> Option<String> {
    match value {
        Value::Object(object) => {
            let is_english = object.get("lang").and_then(Value::as_str) == Some("en")
                || object.get("label").and_then(Value::as_str) == Some("English");
            if is_english {
                if let Some(source) = object.get("src").and_then(Value::as_str) {
                    return Some(source.to_owned());
                }
            }
            object.values().find_map(find_english_subtitle)
        }
        Value::Array(values) => values.iter().find_map(find_english_subtitle),
        _ => None,
    }
}

fn parse_m3u8(content: &str, playlist_url: &str, referer: &str) -> Vec<VideoLink> {
    let base = playlist_url
        .rsplit_once('/')
        .map(|(base, _)| format!("{}/", base))
        .unwrap_or_default();
    let mut links = Vec::new();
    let mut pending_quality = None;

    for line in content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        if line.starts_with("#EXT-X-STREAM-INF") {
            pending_quality = line
                .split(',')
                .find_map(|attribute| attribute.trim().strip_prefix("RESOLUTION="))
                .and_then(|resolution| resolution.rsplit_once('x').map(|(_, height)| height))
                .map(|height| format!("{}p", height));
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        if let Some(quality) = pending_quality.take() {
            let url = if line.starts_with("http://") || line.starts_with("https://") {
                line.to_owned()
            } else {
                format!("{}{}", base, line)
            };
            links.push(VideoLink {
                quality,
                url,
                referer: Some(referer.to_owned()),
                subtitle: None,
            });
        }
    }

    links.sort_by_key(|link| {
        std::cmp::Reverse(
            link.quality
                .trim_end_matches('p')
                .parse::<u16>()
                .unwrap_or_default(),
        )
    });
    links
}

fn get_links(provider_id: &str) -> Result<Vec<VideoLink>> {
    if provider_id.contains("tools.fast4speed.rsvp") {
        return Ok(vec![VideoLink {
            quality: "best".to_owned(),
            url: provider_id.to_owned(),
            referer: Some(ALLANIME_REFR.to_owned()),
            subtitle: None,
        }]);
    }

    let url = format!("https://{}{}", ALLANIME_BASE, provider_id);

    let bytes = crate::http::get(
        &url,
        &[("Referer", ALLANIME_REFR), ("User-Agent", USER_AGENT)],
    )?;
    let response: Value = serde_json::from_slice(&bytes).context("Invalid provider response")?;
    let mut direct_links = Vec::new();
    collect_direct_links(&response, &mut direct_links);
    if direct_links.is_empty() {
        return Ok(Vec::new());
    }
    let referer = find_string(&response, "Referer").unwrap_or_else(|| ALLANIME_REFR.to_owned());
    let subtitle = find_english_subtitle(&response);
    let mut links = Vec::new();
    for (quality, url) in direct_links {
        if url.contains("master.m3u8") {
            let playlist =
                crate::http::get(&url, &[("Referer", &referer), ("User-Agent", USER_AGENT)])?;
            let mut variants = parse_m3u8(&String::from_utf8_lossy(&playlist), &url, &referer);
            if let Some(subtitle) = &subtitle {
                variants
                    .iter_mut()
                    .for_each(|link| link.subtitle = Some(subtitle.clone()));
            }
            links.append(&mut variants);
        } else {
            links.push(VideoLink {
                quality,
                url,
                referer: Some(referer.clone()),
                subtitle: subtitle.clone(),
            });
        }
    }
    Ok(links)
}

pub fn get_all_links(sources: &str) -> Result<Vec<VideoLink>> {
    let mut all_links = Vec::new();

    for line in sources.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let Some((provider_name, raw_id)) = line.split_once(':') else {
            continue;
        };
        let provider_id = raw_id
            .strip_prefix("--")
            .map(decode_provider_id)
            .unwrap_or_else(|| raw_id.to_owned());

        if provider_id.is_empty() {
            continue;
        }

        println!("🔗 Provider: {}", provider_name);

        match get_links(&provider_id) {
            Ok(mut links) => {
                all_links.append(&mut links);
            }
            Err(e) => {
                eprintln!("⚠️  Provider {} error: {}", provider_name, e);
            }
        }
    }

    Ok(all_links)
}

pub fn select_quality<'a>(links: &'a [VideoLink], quality: &str) -> Result<&'a VideoLink> {
    if links.is_empty() {
        anyhow::bail!("No available links");
    }

    match quality {
        "best" => Ok(&links[0]),
        "worst" => {
            for link in links.iter().rev() {
                if link
                    .quality
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    return Ok(link);
                }
            }
            Ok(&links[links.len() - 1])
        }
        _ => {
            for link in links {
                if link.quality.contains(quality) {
                    return Ok(link);
                }
            }
            println!("⚠️  Quality {} not found, using best", quality);
            Ok(&links[0])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(value: &str) -> String {
        value
            .bytes()
            .map(|byte| format!("{:02x}", byte ^ 0x38))
            .collect()
    }

    #[test]
    fn decodes_provider_path_without_a_shell() {
        assert_eq!(
            decode_provider_id(&encode("/apivtwo/clock")),
            "/apivtwo/clock.json"
        );
    }

    #[test]
    fn rejects_invalid_provider_encoding() {
        assert!(decode_provider_id("xyz").is_empty());
    }

    #[test]
    fn quality_selection_uses_requested_stream() {
        let links = vec![
            VideoLink {
                quality: "1080p".to_owned(),
                url: "high".to_owned(),
                referer: None,
                subtitle: None,
            },
            VideoLink {
                quality: "720p".to_owned(),
                url: "medium".to_owned(),
                referer: None,
                subtitle: None,
            },
        ];
        assert_eq!(select_quality(&links, "720").unwrap().url, "medium");
        assert_eq!(select_quality(&links, "best").unwrap().url, "high");
    }

    #[test]
    fn extracts_nested_provider_links_and_subtitles() {
        let response = serde_json::json!({
            "result": {
                "links": [{"link": "https://cdn.test/master.m3u8", "resolutionStr": "best"}],
                "headers": {"Referer": "https://video.test"},
                "subtitles": [{"lang": "en", "label": "English", "src": "https://cdn.test/en.vtt"}]
            }
        });
        let mut links = Vec::new();
        collect_direct_links(&response, &mut links);

        assert_eq!(
            links,
            vec![("best".to_owned(), "https://cdn.test/master.m3u8".to_owned())]
        );
        assert_eq!(
            find_string(&response, "Referer").as_deref(),
            Some("https://video.test")
        );
        assert_eq!(
            find_english_subtitle(&response).as_deref(),
            Some("https://cdn.test/en.vtt")
        );
    }

    #[test]
    fn parses_and_sorts_relative_m3u8_variants() {
        let playlist = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=800000,RESOLUTION=1280x720\n720/index.m3u8\n#EXT-X-STREAM-INF:BANDWIDTH=1600000,RESOLUTION=1920x1080\nhttps://cdn.test/1080/index.m3u8\n";
        let links = parse_m3u8(
            playlist,
            "https://cdn.test/master.m3u8",
            "https://video.test",
        );

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].quality, "1080p");
        assert_eq!(links[1].url, "https://cdn.test/720/index.m3u8");
    }
}
