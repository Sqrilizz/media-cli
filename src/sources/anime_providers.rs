use anyhow::{Context, Result};
use std::process::Command;

const ALLANIME_BASE: &str = "allanime.day";
const ALLANIME_REFR: &str = "https://allmanga.to";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

pub struct VideoLink {
    pub quality: String,
    pub url: String,
    pub referer: Option<String>,
    pub subtitle: Option<String>,
}

fn decode_provider_id(encoded: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"echo '{}' | sed 's/../&\n/g' | sed 's/^79$/A/g;s/^7a$/B/g;s/^7b$/C/g;s/^7c$/D/g;s/^7d$/E/g;s/^7e$/F/g;s/^7f$/G/g;s/^70$/H/g;s/^71$/I/g;s/^72$/J/g;s/^73$/K/g;s/^74$/L/g;s/^75$/M/g;s/^76$/N/g;s/^77$/O/g;s/^68$/P/g;s/^69$/Q/g;s/^6a$/R/g;s/^6b$/S/g;s/^6c$/T/g;s/^6d$/U/g;s/^6e$/V/g;s/^6f$/W/g;s/^60$/X/g;s/^61$/Y/g;s/^62$/Z/g;s/^59$/a/g;s/^5a$/b/g;s/^5b$/c/g;s/^5c$/d/g;s/^5d$/e/g;s/^5e$/f/g;s/^5f$/g/g;s/^50$/h/g;s/^51$/i/g;s/^52$/j/g;s/^53$/k/g;s/^54$/l/g;s/^55$/m/g;s/^56$/n/g;s/^57$/o/g;s/^48$/p/g;s/^49$/q/g;s/^4a$/r/g;s/^4b$/s/g;s/^4c$/t/g;s/^4d$/u/g;s/^4e$/v/g;s/^4f$/w/g;s/^40$/x/g;s/^41$/y/g;s/^42$/z/g;s/^08$/0/g;s/^09$/1/g;s/^0a$/2/g;s/^0b$/3/g;s/^0c$/4/g;s/^0d$/5/g;s/^0e$/6/g;s/^0f$/7/g;s/^00$/8/g;s/^01$/9/g;s/^15$/-/g;s/^16$/./g;s/^67$/_/g;s/^46$/~/g;s/^02$/:/g;s/^17$/\//g;s/^07$/?/g;s/^1b$/#/g;s/^63$/\[/g;s/^65$/\]/g;s/^78$/@/g;s/^19$/!/g;s/^1c$/$/g;s/^1e$/&/g;s/^10$/\(/g;s/^11$/\)/g;s/^12$/*/g;s/^13$/+/g;s/^14$/,/g;s/^03$/;/g;s/^05$/=/g;s/^1d$/%/g' | tr -d '\n' | sed 's/\/clock/\/clock.json/'"#,
            encoded
        ))
        .output();
    
    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn get_links(provider_id: &str) -> Result<Vec<VideoLink>> {
    let url = format!("https://{}{}", ALLANIME_BASE, provider_id);
    
    let output = Command::new("curl")
        .args(["-e", ALLANIME_REFR, "-s", &url, "-A", USER_AGENT])
        .output()
        .context("Failed to fetch links")?;

    if !output.status.success() {
        anyhow::bail!("Error fetching links");
    }

    let response = String::from_utf8_lossy(&output.stdout);
    
    let output2 = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"echo '{}' | sed 's|}}{{|\n|g' | sed -nE 's|.*link":"([^"]*)".*"resolutionStr":"([^"]*)".*|\2>\1|p;s|.*hls","url":"([^"]*)".*"hardsub_lang":"en-US".*|\1|p'"#,
            response.replace('\'', "'\\''")
        ))
        .output()
        .context("Failed to parse links")?;

    let episode_link = String::from_utf8_lossy(&output2.stdout).trim().to_string();
    
    if episode_link.is_empty() {
        return Ok(Vec::new());
    }
    
    let mut links = Vec::new();
    
    if episode_link.contains("master.m3u8") {
        let m3u8_refr_output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                r#"echo '{}' | sed -nE 's|.*Referer":"([^"]*)".*|\1|p'"#,
                response.replace('\'', "'\\''")
            ))
            .output()
            .context("Failed to extract referer")?;
        
        let m3u8_refr = String::from_utf8_lossy(&m3u8_refr_output.stdout).trim().to_string();
        let referer = if m3u8_refr.is_empty() {
            ALLANIME_REFR.to_string()
        } else {
            m3u8_refr.clone()
        };
        
        let parts: Vec<&str> = episode_link.split('>').collect();
        let extract_link = if parts.len() == 2 { parts[1] } else { &episode_link };
        
        let relative_link = extract_link.rsplit_once('/').map(|(base, _)| format!("{}/", base)).unwrap_or_default();
        
        let m3u8_output = Command::new("curl")
            .args(["-e", &referer, "-s", extract_link, "-A", USER_AGENT])
            .output()
            .context("Failed to fetch m3u8")?;
        
        let m3u8_streams = String::from_utf8_lossy(&m3u8_output.stdout);
        
        if m3u8_streams.contains("EXTM3U") {
            let streams_output = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    r#"echo '{}' | sed 's|^#EXT-X-STREAM.*x||g; s|,.*|p|g; /^#/d; $!N; s|\n| >|;/EXT-X-I-FRAME/d' | sed 's|>|cc>{}|g' | sort -nr"#,
                    m3u8_streams.replace('\'', "'\\''"),
                    relative_link
                ))
                .output()
                .context("Failed to parse m3u8")?;
            
            let streams = String::from_utf8_lossy(&streams_output.stdout);
            
            for line in streams.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let parts: Vec<&str> = line.split("cc>").collect();
                if parts.len() == 2 {
                    links.push(VideoLink {
                        quality: parts[0].to_string(),
                        url: parts[1].to_string(),
                        referer: Some(referer.clone()),
                        subtitle: None,
                    });
                }
            }
        }
        
        let subtitle_output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                r#"echo '{}' | sed -nE 's|.*"subtitles":\[\{{"lang":"en","label":"English","default":"default","src":"([^"]*)".*|subtitle>\1|p'"#,
                response.replace('\'', "'\\''")
            ))
            .output()
            .context("Failed to extract subtitles")?;
        
        let subtitle = String::from_utf8_lossy(&subtitle_output.stdout).trim().to_string();
        if !subtitle.is_empty() && subtitle.starts_with("subtitle>") {
            let sub_url = subtitle.strip_prefix("subtitle>").unwrap_or("");
            if !links.is_empty() {
                links[0].subtitle = Some(sub_url.to_string());
            }
        }
    } else {
        let parts: Vec<&str> = episode_link.split('>').collect();
        if parts.len() == 2 {
            links.push(VideoLink {
                quality: parts[0].to_string(),
                url: parts[1].to_string(),
                referer: Some(ALLANIME_REFR.to_string()),
                subtitle: None,
            });
        } else {
            links.push(VideoLink {
                quality: "best".to_string(),
                url: episode_link,
                referer: Some(ALLANIME_REFR.to_string()),
                subtitle: None,
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
        
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }
        
        let provider_name = parts[0];
        let encoded_id = parts[1];
        
        let provider_id = decode_provider_id(encoded_id);
        
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
                if link.quality.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
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
