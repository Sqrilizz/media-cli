mod cli;
mod history;
mod player;
mod selector;
mod sources;
mod tui;
mod bypass;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let mut cli = Cli::parse();

    if cli.bypass_help {
        bypass::show_bypass_help();
        return Ok(());
    }

    let proxy = if let Some(proxy_arg) = &cli.proxy {
        if proxy_arg == "auto" {
            println!("\x1B[1;36m🔍 Auto-selecting proxy...\x1B[0m");
            bypass::auto_bypass()?
        } else {
            Some(proxy_arg.clone())
        }
    } else {
        None
    };

    if cli.command.is_none() {
        return run_main_menu(proxy.as_deref());
    }

    match cli.command.unwrap() {
        Commands::Yt { query, player, quality, terminal, auto, no_detach, proxy: cmd_proxy } => {
            let proxy_to_use = proxy.as_deref().or(cmd_proxy.as_deref());
            
            if sources::youtube::is_youtube_url(&query) {
                println!("🔗 Opening link...");
                history::add(&query, &query)?;
                player::play_youtube(&query, player.as_deref(), quality.as_deref(), terminal || no_detach, proxy_to_use)?;
                return Ok(());
            }
            
            let results = sources::youtube::search(&query)?;
            if results.is_empty() {
                println!("No results found");
                return Ok(());
            }
            
            let selected = if auto {
                Some(results[0].clone())
            } else {
                selector::select(&results)?
            };
            
            if let Some(mut item) = selected {
                loop {
                    history::add(&item.url, &item.title)?;
                    player::play_youtube(&item.url, player.as_deref(), quality.as_deref(), terminal || no_detach, proxy_to_use)?;
                    
                    let action = selector::select_action()?;
                    match action.as_str() {
                        "replay" => continue,
                        "next" => {
                            if let Some(next_item) = results.iter().skip_while(|i| i.url != item.url).nth(1) {
                                item = next_item.clone();
                            } else {
                                println!("This is the last video");
                                break;
                            }
                        }
                        "previous" => {
                            let mut found = false;
                            for i in (0..results.len()).rev() {
                                if results[i].url == item.url && i > 0 {
                                    item = results[i - 1].clone();
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                println!("This is the first video");
                                break;
                            }
                        }
                        "select" => {
                            if let Some(new_item) = selector::select(&results)? {
                                item = new_item;
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
        Commands::Music { query, proxy: cmd_proxy } => {
            let proxy_to_use = proxy.as_deref().or(cmd_proxy.as_deref());
            
            let results = sources::youtube::search(&query)?;
            if results.is_empty() {
                println!("No results found");
                return Ok(());
            }
            
            let selected = selector::select(&results)?;
            
            if let Some(mut item) = selected {
                loop {
                    history::add(&item.url, &item.title)?;
                    
                    println!("\x1B[1;32m♪ Playing:\x1B[0m {}", item.title);
                    
                    let mut cmd = std::process::Command::new("mpv");
                    cmd.arg("--no-video")
                        .arg("--term-osd-bar")
                        .arg("--term-playing-msg=\x1B[1;36m▶ ${media-title}\x1B[0m");
                    
                    if let Some(proxy_url) = proxy_to_use {
                        cmd.arg(format!("--http-proxy={}", proxy_url));
                    }
                    
                    cmd.arg(&item.url);
                    
                    let _ = cmd.status();
                    
                    let action = selector::select_action()?;
                    match action.as_str() {
                        "replay" => continue,
                        "next" => {
                            if let Some(next_item) = results.iter().skip_while(|i| i.url != item.url).nth(1) {
                                item = next_item.clone();
                            } else {
                                println!("This is the last track");
                                break;
                            }
                        }
                        "previous" => {
                            let mut found = false;
                            for i in (0..results.len()).rev() {
                                if results[i].url == item.url && i > 0 {
                                    item = results[i - 1].clone();
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                println!("This is the first track");
                                break;
                            }
                        }
                        "select" => {
                            if let Some(new_item) = selector::select(&results)? {
                                item = new_item;
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
        Commands::File { path, player } => {
            let files = sources::local::scan(path.as_deref())?;
            if files.is_empty() {
                println!("No files found");
                return Ok(());
            }
            
            if let Some(selected) = selector::select(&files)? {
                player::play(&selected.url, player.as_deref())?;
            }
        }
        Commands::History { clear } => {
            if clear {
                history::clear()?;
                println!("✓ History cleared");
                return Ok(());
            }
            
            let entries = history::load()?;
            if entries.is_empty() {
                println!("History is empty");
                return Ok(());
            }
            
            println!("📜 Watch History:\n");
            for (idx, entry) in entries.iter().enumerate().take(10) {
                println!("{}. {}", idx + 1, entry.title);
            }
        }
        Commands::Twitch { channel, player, quality, terminal, proxy } => {
            println!("📺 Connecting to Twitch...");
            
            let channel_name = if sources::twitch::is_twitch_url(&channel) {
                sources::twitch::extract_channel(&channel)
            } else {
                channel.clone()
            };
            
            let twitch_url = format!("https://twitch.tv/{}", channel_name);
            
            println!("🔴 Connecting to: {}", channel_name);
            history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;
            
            if terminal {
                player::play_stream_terminal(&twitch_url, quality.as_deref(), proxy.as_deref())?;
            } else {
                player::play_stream(&twitch_url, player.as_deref(), quality.as_deref(), false, proxy.as_deref())?;
            }
        }
        Commands::Anime { query, player, quality, terminal, proxy, mode, gui } => {
            println!("🎌 Searching anime...");
            
            let results = sources::anime::search_anime(&query, &mode)?;
            if results.is_empty() {
                println!("No results found");
                return Ok(());
            }
            
            let selected = if gui {
                tui::run_tui(results)?
            } else {
                selector::select(&results)?
            };
            
            if let Some(anime) = selected {
                let show_id = &anime.url;
                let episodes = sources::anime::get_episodes(show_id, &mode)?;
                
                if episodes.is_empty() {
                    println!("No episodes found");
                    return Ok(());
                }
                
                let ep_items: Vec<_> = episodes.iter().map(|ep| {
                    sources::MediaItem {
                        title: format!("Episode {}", ep),
                        url: format!("{}:{}", show_id, ep),
                        duration: None,
                    }
                }).collect();
                
                let selected_ep = if gui {
                    tui::run_tui(ep_items.clone())?
                } else {
                    selector::select(&ep_items)?
                };
                
                if let Some(mut episode) = selected_ep {
                    loop {
                        println!("▶ Playing {} - {}", anime.title, episode.title);
                        history::add(&episode.url, &format!("{} - {}", anime.title, episode.title))?;
                        
                        let parts: Vec<&str> = episode.url.split(':').collect();
                        if parts.len() >= 2 {
                            let show_id = parts[0];
                            let ep_no = parts[1];
                            
                            if let Err(e) = sources::anime::play_anime_episode(
                                show_id,
                                ep_no,
                                &mode,
                                player.as_deref(),
                                quality.as_deref(),
                                terminal,
                                proxy.as_deref()
                            ) {
                                println!("❌ Playback error: {}", e);
                            }
                        }
                        
                        let action = selector::select_action()?;
                        match action.as_str() {
                            "replay" => continue,
                            "next" => {
                                if let Some(next_ep) = ep_items.iter().skip_while(|e| e.url != episode.url).nth(1) {
                                    episode = next_ep.clone();
                                } else {
                                    println!("This is the last episode");
                                    break;
                                }
                            }
                            "previous" => {
                                let mut found = false;
                                for i in (0..ep_items.len()).rev() {
                                    if ep_items[i].url == episode.url && i > 0 {
                                        episode = ep_items[i - 1].clone();
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    println!("This is the first episode");
                                    break;
                                }
                            }
                            "select" => {
                                let new_ep = if gui {
                                    tui::run_tui(ep_items.clone())?
                                } else {
                                    selector::select(&ep_items)?
                                };
                                
                                if let Some(ep) = new_ep {
                                    episode = ep;
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn run_main_menu(proxy: Option<&str>) -> Result<()> {
    use std::io::{self, Write};

    loop {
        print!("\x1B[2J\x1B[1;1H");
        
        println!("\x1B[1;36m╔════════════════════════════════════════════════════════════╗\x1B[0m");
        println!("\x1B[1;36m║\x1B[0m           \x1B[1;35m▶  MEDIA-CLI\x1B[0m  \x1B[1;90m- Universal Player\x1B[0m          \x1B[1;36m║\x1B[0m");
        println!("\x1B[1;36m╚════════════════════════════════════════════════════════════╝\x1B[0m\n");
        
        println!("\x1B[1;33m  Select source:\x1B[0m\n");
        println!("    \x1B[1;31m[\x1B[1;97m1\x1B[1;31m]\x1B[0m  \x1B[1;31m▶\x1B[0m  \x1B[1;97mYouTube\x1B[0m          \x1B[90m- videos & music\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m2\x1B[1;31m]\x1B[0m  \x1B[1;35m♪\x1B[0m  \x1B[1;97mYouTube Music\x1B[0m    \x1B[90m- audio only\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m3\x1B[1;31m]\x1B[0m  \x1B[1;35m●\x1B[0m  \x1B[1;97mTwitch\x1B[0m           \x1B[90m- live streams\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m4\x1B[1;31m]\x1B[0m  \x1B[1;36m◆\x1B[0m  \x1B[1;97mAnime\x1B[0m            \x1B[90m- anime series\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m5\x1B[1;31m]\x1B[0m  \x1B[1;33m■\x1B[0m  \x1B[1;97mLocal Files\x1B[0m      \x1B[90m- ~/Videos\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m6\x1B[1;31m]\x1B[0m  \x1B[1;34m≡\x1B[0m  \x1B[1;97mHistory\x1B[0m          \x1B[90m- watched\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m7\x1B[1;31m]\x1B[0m  \x1B[1;33m⚡\x1B[0m  \x1B[1;97mBypass Help\x1B[0m      \x1B[90m- proxy info\x1B[0m");
        println!("    \x1B[1;31m[\x1B[1;97m8\x1B[1;31m]\x1B[0m  \x1B[1;31m✕\x1B[0m  \x1B[1;97mExit\x1B[0m\n");
        
        print!("\x1B[1;32m  ➜\x1B[0m  ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("\n\x1B[1;36m╭─ YouTube ──────────────────────────────────────────╮\x1B[0m\n");
                print!("\x1B[1;36m│\x1B[0m \x1B[1;33m▶ Search or URL:\x1B[0m ");
                io::stdout().flush()?;
                let mut query = String::new();
                io::stdin().read_line(&mut query)?;
                let query = query.trim();
                print!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                
                if query.is_empty() {
                    continue;
                }

                let results = sources::youtube::search(query)?;
                if results.is_empty() {
                    println!("No results found");
                    continue;
                }

                let selected = if sources::youtube::is_youtube_url(query) {
                    Some(sources::MediaItem {
                        title: query.to_string(),
                        url: query.to_string(),
                        duration: None,
                    })
                } else {
                    selector::select(&results)?
                };

                if let Some(mut item) = selected {
                    loop {
                        history::add(&item.url, &item.title)?;
                        player::play_youtube(&item.url, None, None, false, proxy)?;

                        let action = selector::select_action()?;
                        match action.as_str() {
                            "replay" => continue,
                            "next" => {
                                if let Some(next) = results.iter().skip_while(|i| i.url != item.url).nth(1) {
                                    item = next.clone();
                                } else {
                                    break;
                                }
                            }
                            "previous" => {
                                let mut found = false;
                                for i in (0..results.len()).rev() {
                                    if results[i].url == item.url && i > 0 {
                                        item = results[i - 1].clone();
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    break;
                                }
                            }
                            "select" => {
                                if let Some(new) = selector::select(&results)? {
                                    item = new;
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
            "2" => {
                print!("\n\x1B[1;36m╭─ YouTube Music ────────────────────────────────────╮\x1B[0m\n");
                print!("\x1B[1;36m│\x1B[0m \x1B[1;33m♪ Search music:\x1B[0m ");
                io::stdout().flush()?;
                let mut query = String::new();
                io::stdin().read_line(&mut query)?;
                let query = query.trim();
                print!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                
                if query.is_empty() {
                    continue;
                }

                let results = sources::youtube::search(query)?;
                if results.is_empty() {
                    println!("No results found");
                    continue;
                }
                
                if let Some(mut item) = selector::select(&results)? {
                    loop {
                        history::add(&item.url, &item.title)?;
                        
                        println!("\x1B[1;32m♪ Playing:\x1B[0m {}", item.title);
                        
                        let mut cmd = std::process::Command::new("mpv");
                        cmd.arg("--no-video")
                            .arg("--term-osd-bar")
                            .arg("--term-playing-msg=\x1B[1;36m▶ ${media-title}\x1B[0m");
                        
                        if let Some(proxy_url) = proxy {
                            cmd.arg(format!("--http-proxy={}", proxy_url));
                        }
                        
                        cmd.arg(&item.url);
                        
                        let _ = cmd.status();
                        
                        let action = selector::select_action()?;
                        match action.as_str() {
                            "replay" => continue,
                            "next" => {
                                if let Some(next_item) = results.iter().skip_while(|i| i.url != item.url).nth(1) {
                                    item = next_item.clone();
                                } else {
                                    println!("This is the last track");
                                    break;
                                }
                            }
                            "previous" => {
                                let mut found = false;
                                for i in (0..results.len()).rev() {
                                    if results[i].url == item.url && i > 0 {
                                        item = results[i - 1].clone();
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    println!("This is the first track");
                                    break;
                                }
                            }
                            "select" => {
                                if let Some(new_item) = selector::select(&results)? {
                                    item = new_item;
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
            "3" => {
                print!("\n\x1B[1;36m╭─ Twitch ───────────────────────────────────────────╮\x1B[0m\n");
                print!("\x1B[1;36m│\x1B[0m \x1B[1;33m● Channel or URL:\x1B[0m ");
                io::stdout().flush()?;
                let mut channel = String::new();
                io::stdin().read_line(&mut channel)?;
                let channel = channel.trim();
                print!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                
                if channel.is_empty() {
                    continue;
                }

                let channel_name = if sources::twitch::is_twitch_url(channel) {
                    sources::twitch::extract_channel(channel)
                } else {
                    channel.to_string()
                };

                let twitch_url = format!("https://twitch.tv/{}", channel_name);
                println!("🔴 Connecting to: {}", channel_name);
                history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;
                player::play_stream(&twitch_url, None, None, false, proxy)?;
            }
            "4" => {
                print!("\n\x1B[1;36m╭─ Anime ────────────────────────────────────────────╮\x1B[0m\n");
                print!("\x1B[1;36m│\x1B[0m \x1B[1;33m◆ Search anime:\x1B[0m ");
                io::stdout().flush()?;
                let mut query = String::new();
                io::stdin().read_line(&mut query)?;
                let query = query.trim();
                print!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                
                if query.is_empty() {
                    continue;
                }

                println!("\x1B[1;90m⏳ Searching anime...\x1B[0m");
                let results = sources::anime::search_anime(query, "sub")?;
                if results.is_empty() {
                    println!("No results found");
                    continue;
                }

                let selected = selector::select(&results)?;
                if let Some(anime) = selected {
                    let episodes = sources::anime::get_episodes(&anime.url, "sub")?;
                    if episodes.is_empty() {
                        println!("No episodes found");
                        continue;
                    }

                    let ep_items: Vec<_> = episodes.iter().map(|ep| {
                        sources::MediaItem {
                            title: format!("Episode {}", ep),
                            url: format!("{}:{}", anime.url, ep),
                            duration: None,
                        }
                    }).collect();

                    if let Some(mut episode) = selector::select(&ep_items)? {
                        loop {
                            let parts: Vec<&str> = episode.url.split(':').collect();
                            if parts.len() >= 2 {
                                let show_id = parts[0];
                                let ep_no = parts[1];
                                
                                println!("▶ {} - Episode {}", anime.title, ep_no);
                                history::add(&episode.url, &format!("{} - Episode {}", anime.title, ep_no))?;
                                
                                match sources::anime::play_anime_episode(show_id, ep_no, "sub", None, None, false, proxy) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        println!("❌ Error: {}", e);
                                        println!("\x1B[1;33m⏭  Trying next episode...\x1B[0m");
                                        
                                        if let Some(next_ep) = ep_items.iter().skip_while(|e| e.url != episode.url).nth(1) {
                                            episode = next_ep.clone();
                                            continue;
                                        } else {
                                            println!("No more episodes");
                                            break;
                                        }
                                    }
                                }
                            }
                            
                            let action = selector::select_action()?;
                            match action.as_str() {
                                "replay" => continue,
                                "next" => {
                                    if let Some(next) = ep_items.iter().skip_while(|e| e.url != episode.url).nth(1) {
                                        episode = next.clone();
                                    } else {
                                        println!("This is the last episode");
                                        break;
                                    }
                                }
                                "previous" => {
                                    let mut found = false;
                                    for i in (0..ep_items.len()).rev() {
                                        if ep_items[i].url == episode.url && i > 0 {
                                            episode = ep_items[i - 1].clone();
                                            found = true;
                                            break;
                                        }
                                    }
                                    if !found {
                                        println!("This is the first episode");
                                        break;
                                    }
                                }
                                "select" => {
                                    if let Some(new) = selector::select(&ep_items)? {
                                        episode = new;
                                    } else {
                                        break;
                                    }
                                }
                                _ => break,
                            }
                        }
                    }
                }
            }
            "5" => {
                print!("\n\x1B[1;36m╭─ Local Files ──────────────────────────────────────╮\x1B[0m\n");
                print!("\x1B[1;36m│\x1B[0m \x1B[1;33m■ Path (Enter = ~/Videos):\x1B[0m ");
                io::stdout().flush()?;
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                print!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                
                let path_opt = if path.is_empty() { None } else { Some(path) };
                let files = sources::local::scan(path_opt)?;
                
                if files.is_empty() {
                    println!("No files found");
                    continue;
                }

                if let Some(selected) = selector::select(&files)? {
                    player::play(&selected.url, None)?;
                }
            }
            "6" => {
                let entries = history::load()?;
                if entries.is_empty() {
                    println!("\n\x1B[1;33m≡ History is empty\x1B[0m\n");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }

                println!("\n\x1B[1;36m╭─ Watch History ────────────────────────────────────╮\x1B[0m");
                for (idx, entry) in entries.iter().enumerate().take(10) {
                    println!("\x1B[1;36m│\x1B[0m  \x1B[1;90m{:2}.\x1B[0m {}", idx + 1, entry.title);
                }
                println!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            "7" => {
                bypass::show_bypass_help();
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
            "8" => {
                println!("\n👋 Goodbye!");
                break;
            }
            _ => {
                println!("❌ Invalid choice");
            }
        }
    }

    Ok(())
}
