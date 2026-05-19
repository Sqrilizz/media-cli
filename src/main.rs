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
use sources::MediaItem;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn find_current_index(items: &[MediaItem], current: &MediaItem) -> Option<usize> {
    items.iter().position(|i| i.url == current.url)
}

fn playback_loop<F>(
    items: &[MediaItem],
    mut current: MediaItem,
    use_gui: bool,
    mut play_fn: F,
) -> Result<()>
where
    F: FnMut(&MediaItem) -> Result<()>,
{
    loop {
        history::add(&current.url, &current.title)?;
        if let Err(e) = play_fn(&current) {
            println!("\x1B[1;31m✕ Playback error:\x1B[0m {}", e);
        }

        let action = selector::select_action()?;
        match action.as_str() {
            "replay" => continue,
            "next" => {
                if let Some(idx) = find_current_index(items, &current) {
                    if idx + 1 < items.len() {
                        current = items[idx + 1].clone();
                    } else {
                        println!("\x1B[1;33m⚠ This is the last item\x1B[0m");
                        continue;
                    }
                } else {
                    break;
                }
            }
            "previous" => {
                if let Some(idx) = find_current_index(items, &current) {
                    if idx > 0 {
                        current = items[idx - 1].clone();
                    } else {
                        println!("\x1B[1;33m⚠ This is the first item\x1B[0m");
                        continue;
                    }
                } else {
                    break;
                }
            }
            "select" => {
                let new_item = if use_gui {
                    tui::run_tui(items.to_vec())?
                } else {
                    selector::select(items)?
                };
                if let Some(item) = new_item {
                    current = item;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    Ok(())
}

fn check_deps() {
    let deps = [
        ("mpv", "video player"),
        ("yt-dlp", "YouTube downloads"),
        ("streamlink", "Twitch streams"),
        ("fzf", "interactive selection"),
        ("curl", "API requests"),
    ];
    let mut missing = Vec::new();
    for (cmd, desc) in &deps {
        if std::process::Command::new(cmd)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_err()
        {
            missing.push((*cmd, *desc));
        }
    }
    if !missing.is_empty() {
        println!("\x1B[1;33m\u{26a0} Missing dependencies:\x1B[0m");
        for (cmd, desc) in &missing {
            println!("  \x1B[38;5;203m\u{2715}\x1B[0m  \x1B[1m{}\x1B[0m \x1B[38;5;245m- {}\x1B[0m", cmd, desc);
        }
        println!();
        println!("\x1B[38;5;245mRun the installer to fix:\x1B[0m");
        println!("  \x1B[1mcurl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash\x1B[0m");
        println!();
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.bypass_help {
        bypass::show_bypass_help();
        return Ok(());
    }

    check_deps();

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

    let terminal = cli.terminal;

    if cli.command.is_none() {
        return run_main_menu(proxy.as_deref(), terminal);
    }

    match cli.command.unwrap() {
        Commands::Yt { query, player, quality, auto, no_detach, proxy: cmd_proxy } => {
            let proxy_to_use = proxy.as_deref().or(cmd_proxy.as_deref());

            if sources::youtube::is_youtube_url(&query) {
                println!("\x1B[1;36m🔗 Opening link...\x1B[0m");
                history::add(&query, &query)?;
                player::play_youtube(&query, player.as_deref(), quality.as_deref(), terminal || no_detach, proxy_to_use)?;
                return Ok(());
            }

            let results = sources::youtube::search(&query)?;
            if results.is_empty() {
                println!("\x1B[1;33m⚠ No results found\x1B[0m");
                return Ok(());
            }

            let selected = if auto {
                Some(results[0].clone())
            } else {
                selector::select(&results)?
            };

            if let Some(item) = selected {
                let p = player.clone();
                let q = quality.clone();
                let t = terminal || no_detach;
                let px = proxy_to_use.map(|s| s.to_string());
                playback_loop(&results, item, false, |item| {
                    player::play_youtube(&item.url, p.as_deref(), q.as_deref(), t, px.as_deref())
                })?;
            }
        }
        Commands::Music { query, proxy: cmd_proxy } => {
            let proxy_to_use = proxy.as_deref().or(cmd_proxy.as_deref());

            let results = sources::youtube::search(&query)?;
            if results.is_empty() {
                println!("\x1B[1;33m⚠ No results found\x1B[0m");
                return Ok(());
            }

            if let Some(item) = selector::select(&results)? {
                let px = proxy_to_use.map(|s| s.to_string());
                playback_loop(&results, item, false, |item| {
                    player::play_music(&item.url, &item.title, px.as_deref())
                })?;
            }
        }
        Commands::File { path, player: player_opt } => {
            let files = sources::local::scan(path.as_deref())?;
            if files.is_empty() {
                println!("\x1B[1;33m⚠ No files found\x1B[0m");
                return Ok(());
            }

            if let Some(selected) = selector::select(&files)? {
                player::play(&selected.url, player_opt.as_deref())?;
            }
        }
        Commands::History { clear } => {
            if clear {
                history::clear()?;
                println!("\x1B[1;32m✓ History cleared\x1B[0m");
                return Ok(());
            }

            let entries = history::load()?;
            if entries.is_empty() {
                println!("\x1B[1;33m⚠ History is empty\x1B[0m");
                return Ok(());
            }

            println!("\n\x1B[1;36m╭─ Watch History ────────────────────────────────────╮\x1B[0m");
            for (idx, entry) in entries.iter().enumerate().take(20) {
                println!("\x1B[1;36m│\x1B[0m  \x1B[1;90m{:2}.\x1B[0m {}", idx + 1, entry.title);
            }
            println!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m");
        }
        Commands::Twitch { channel, player, quality, proxy } => {
            let channel_name = if sources::twitch::is_twitch_url(&channel) {
                sources::twitch::extract_channel(&channel)
            } else {
                channel.clone()
            };

            let twitch_url = format!("https://twitch.tv/{}", channel_name);
            println!("\x1B[1;35m🔴 Connecting to:\x1B[0m {}", channel_name);
            history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;

            if terminal {
                player::play_stream_terminal(&twitch_url, quality.as_deref(), proxy.as_deref())?;
            } else {
                player::play_stream(&twitch_url, player.as_deref(), quality.as_deref(), false, proxy.as_deref())?;
            }
        }
        Commands::Anime { query, player, quality, proxy, mode, gui } => {
            println!("\x1B[1;35m🎌 Searching anime...\x1B[0m");

            let results = sources::anime::search_anime(&query, &mode)?;
            if results.is_empty() {
                println!("\x1B[1;33m⚠ No results found\x1B[0m");
                return Ok(());
            }

            let selected = if gui {
                tui::run_tui(results)?
            } else {
                selector::select(&results)?
            };

            if let Some(anime) = selected {
                let show_id = anime.url.clone();
                let episodes = sources::anime::get_episodes(&show_id, &mode)?;

                if episodes.is_empty() {
                    println!("\x1B[1;33m⚠ No episodes found\x1B[0m");
                    return Ok(());
                }

                let ep_items: Vec<_> = episodes.iter().map(|ep| {
                    MediaItem {
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

                if let Some(episode) = selected_ep {
                    let anime_title = anime.title.clone();
                    let m = mode.clone();
                    let p = player.clone();
                    let q = quality.clone();
                    let t = terminal;
                    let px = proxy.clone();
                    playback_loop(&ep_items, episode, gui, move |ep| {
                        println!("\x1B[1;36m▶ Playing {} - {}\x1B[0m", anime_title, ep.title);
                        let parts: Vec<&str> = ep.url.split(':').collect();
                        if parts.len() >= 2 {
                            sources::anime::play_anime_episode(
                                parts[0], parts[1], &m,
                                p.as_deref(), q.as_deref(), t, px.as_deref()
                            )
                        } else {
                            anyhow::bail!("Invalid episode URL format")
                        }
                    })?;
                }
            }
        }
    }

    Ok(())
}

fn prompt_input(label: &str, icon: &str, section: &str) -> Result<String> {
    use std::io::{self, Write};
    print!("\n\x1B[1;36m╭─ {} ─{}\x1B[0m\n", section, "─".repeat(48 - section.len()));
    print!("\x1B[1;36m│\x1B[0m \x1B[1;33m{} {}:\x1B[0m ", icon, label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    print!("\x1B[1;36m╰{}\x1B[0m\n", "─".repeat(52));
    Ok(input)
}

fn wait_for_enter() {
    use std::io::{self, Write};
    print!("\n\x1B[1;90m  Press Enter to continue...\x1B[0m");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn run_main_menu(proxy: Option<&str>, terminal: bool) -> Result<()> {
    use std::io::{self, Write};

    loop {
        print!("\x1B[2J\x1B[1;1H");

        println!();
        println!("  \x1B[38;2;100;43;115m███╗   ███╗███████╗██████╗ ██╗ █████╗      ██████╗██╗     ██╗\x1B[0m");
        println!("  \x1B[38;2;124;50;114m████╗ ████║██╔════╝██╔══██╗██║██╔══██╗    ██╔════╝██║     ██║\x1B[0m");
        println!("  \x1B[38;2;148;57;112m██╔████╔██║█████╗  ██║  ██║██║███████║    ██║     ██║     ██║\x1B[0m");
        println!("  \x1B[38;2;172;63;111m██║╚██╔╝██║██╔══╝  ██║  ██║██║██╔══██║    ██║     ██║     ██║\x1B[0m");
        println!("  \x1B[38;2;185;65;110m██║ ╚═╝ ██║███████╗██████╔╝██║██║  ██║    ╚██████╗███████╗██║\x1B[0m");
        println!("  \x1B[38;2;198;66;110m╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝╚═╝  ╚═╝     ╚═════╝╚══════╝╚═╝\x1B[0m");
        println!();
        let mode_tag = if terminal { "  \x1B[38;2;198;66;110m[terminal mode]\x1B[0m" } else { "" };
        println!("  \x1B[38;5;245mUniversal Media Player\x1B[0m  \x1B[38;5;240mv{}\x1B[0m{}", VERSION, mode_tag);
        println!("  \x1B[38;5;240mby Sqrilizz\x1B[0m");
        println!("  \x1B[38;5;239m-----------------------------------------------------------------\x1B[0m");
        println!();
        println!("  \x1B[38;5;249mSelect source:\x1B[0m");
        println!();
        println!("    \x1B[38;5;203m[\x1B[1;97m1\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mYouTube\x1B[0m            \x1B[38;5;245m- search & play videos\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m2\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mYouTube Music\x1B[0m      \x1B[38;5;245m- audio only mode\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m3\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mTwitch\x1B[0m             \x1B[38;5;245m- live streams\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m4\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mAnime\x1B[0m              \x1B[38;5;245m- anime series & movies\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m5\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mLocal Files\x1B[0m        \x1B[38;5;245m- play from ~/Videos\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m6\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mHistory\x1B[0m            \x1B[38;5;245m- recently watched\x1B[0m");
        println!("    \x1B[38;5;203m[\x1B[1;97m7\x1B[0;38;5;203m]\x1B[0m  \x1B[1;97mBypass Help\x1B[0m        \x1B[38;5;245m- proxy & DPI info\x1B[0m");
        println!();
        println!("    \x1B[38;5;240m[\x1B[38;5;245m0\x1B[38;5;240m]\x1B[0m  \x1B[38;5;245mExit\x1B[0m");
        println!();

        print!("    \x1B[1;38;5;78m❯\x1B[0m ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                let query = prompt_input("Search or URL", "▶", "YouTube")?;
                if query.is_empty() { continue; }

                if sources::youtube::is_youtube_url(&query) {
                    println!("\x1B[1;36m🔗 Opening link...\x1B[0m");
                    history::add(&query, &query)?;
                    player::play_youtube(&query, None, None, terminal, proxy)?;
                } else {
                    let results = sources::youtube::search(&query)?;
                    if results.is_empty() {
                        println!("\x1B[1;33m⚠ No results found\x1B[0m");
                        wait_for_enter();
                        continue;
                    }

                    if let Some(item) = selector::select(&results)? {
                        let px = proxy.map(|s| s.to_string());
                        playback_loop(&results, item, false, |item| {
                            player::play_youtube(&item.url, None, None, terminal, px.as_deref())
                        })?;
                    }
                }
            }
            "2" => {
                let query = prompt_input("Search music", "♪", "YouTube Music")?;
                if query.is_empty() { continue; }

                let results = sources::youtube::search(&query)?;
                if results.is_empty() {
                    println!("\x1B[1;33m⚠ No results found\x1B[0m");
                    wait_for_enter();
                    continue;
                }

                if let Some(item) = selector::select(&results)? {
                    let px = proxy.map(|s| s.to_string());
                    playback_loop(&results, item, false, |item| {
                        player::play_music(&item.url, &item.title, px.as_deref())
                    })?;
                }
            }
            "3" => {
                let channel = prompt_input("Channel or URL", "●", "Twitch")?;
                if channel.is_empty() { continue; }

                let channel_name = if sources::twitch::is_twitch_url(&channel) {
                    sources::twitch::extract_channel(&channel)
                } else {
                    channel.to_string()
                };

                let twitch_url = format!("https://twitch.tv/{}", channel_name);
                println!("\x1B[1;35m🔴 Connecting to:\x1B[0m {}", channel_name);
                history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;
                player::play_stream(&twitch_url, None, None, terminal, proxy)?;
            }
            "4" => {
                let query = prompt_input("Search anime", "◆", "Anime")?;
                if query.is_empty() { continue; }

                println!("\x1B[38;5;245m⏳ Searching anime...\x1B[0m");
                let results = sources::anime::search_anime(&query, "sub")?;
                if results.is_empty() {
                    println!("\x1B[1;33m⚠ No results found\x1B[0m");
                    wait_for_enter();
                    continue;
                }

                if let Some(anime) = selector::select(&results)? {
                    let episodes = sources::anime::get_episodes(&anime.url, "sub")?;
                    if episodes.is_empty() {
                        println!("\x1B[1;33m⚠ No episodes found\x1B[0m");
                        wait_for_enter();
                        continue;
                    }

                    let ep_items: Vec<_> = episodes.iter().map(|ep| {
                        MediaItem {
                            title: format!("Episode {}", ep),
                            url: format!("{}:{}", anime.url, ep),
                            duration: None,
                        }
                    }).collect();

                    if let Some(episode) = selector::select(&ep_items)? {
                        let anime_title = anime.title.clone();
                        let px = proxy.map(|s| s.to_string());
                        playback_loop(&ep_items, episode, false, |ep| {
                            println!("\x1B[1;36m▶ Playing {} - {}\x1B[0m", anime_title, ep.title);
                            let parts: Vec<&str> = ep.url.split(':').collect();
                            if parts.len() >= 2 {
                                sources::anime::play_anime_episode(
                                    parts[0], parts[1], "sub", None, None, terminal, px.as_deref()
                                )
                            } else {
                                anyhow::bail!("Invalid episode URL format")
                            }
                        })?;
                    }
                }
            }
            "5" => {
                let path = prompt_input("Path (Enter = ~/Videos)", "■", "Local Files")?;
                let path_opt = if path.is_empty() { None } else { Some(path.as_str()) };
                let files = sources::local::scan(path_opt)?;

                if files.is_empty() {
                    println!("\x1B[1;33m⚠ No files found\x1B[0m");
                    wait_for_enter();
                    continue;
                }

                if let Some(selected) = selector::select(&files)? {
                    player::play(&selected.url, None)?;
                }
            }
            "6" => {
                let entries = history::load()?;
                if entries.is_empty() {
                    println!("\n\x1B[1;33m⚠ History is empty\x1B[0m");
                    wait_for_enter();
                    continue;
                }

                println!();
                println!("    \x1B[38;5;39m╭─ Watch History ─────────────────────────────────╮\x1B[0m");
                for (idx, entry) in entries.iter().enumerate().take(20) {
                    println!("    \x1B[38;5;39m│\x1B[0m  \x1B[38;5;245m{:2}.\x1B[0m {}", idx + 1, entry.title);
                }
                println!("    \x1B[38;5;39m╰─────────────────────────────────────────────────╯\x1B[0m");
                wait_for_enter();
            }
            "7" => {
                bypass::show_bypass_help();
                wait_for_enter();
            }
            "0" | "8" | "q" => {
                println!("\n\x1B[38;5;245m  👋 Goodbye!\x1B[0m\n");
                break;
            }
            _ => {
                println!("\x1B[1;31m  ✕ Invalid choice\x1B[0m");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }

    Ok(())
}
