mod cli;
mod config;
mod history;
mod http;
mod mpv_ipc;
mod paths;
mod player;
mod sources;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use sources::MediaItem;

fn effective_player(cli_player: Option<player::Player>, config: &config::Config) -> player::Player {
    cli_player.unwrap_or(config.player)
}

fn effective_quality<'a>(
    cli_quality: Option<&'a str>,
    config: &'a config::Config,
) -> Option<&'a str> {
    cli_quality.or_else(|| config.quality_arg())
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn find_current_index(items: &[MediaItem], current: &MediaItem) -> Option<usize> {
    items
        .iter()
        .position(|item| item.url == current.url && item.episode == current.episode)
}

fn play_history_item(item: &MediaItem, config: &config::Config, terminal: bool) -> Result<()> {
    if sources::youtube::is_youtube_url(&item.url) {
        return player::play_youtube(&item.url, config.player, config.quality_arg(), terminal);
    }
    if sources::twitch::is_twitch_url(&item.url) {
        return player::play_stream(&item.url, config.player, config.quality_arg(), terminal);
    }
    if std::path::Path::new(&item.url).is_file() {
        return player::play(&item.url, config.player);
    }
    if let Some(episode) = &item.episode {
        return sources::anime::play_anime_episode(
            &episode.show_id,
            &episode.episode,
            &config.anime_mode,
            config.player,
            config.quality_arg(),
            terminal,
        );
    }
    anyhow::bail!("This history entry is no longer playable")
}

fn playback_loop<F>(items: &[MediaItem], mut current: MediaItem, mut play_fn: F) -> Result<()>
where
    F: FnMut(&MediaItem) -> Result<()>,
{
    loop {
        history::add_item(&current)?;
        if let Err(e) = play_fn(&current) {
            println!("\x1B[1;31m✕ Playback error:\x1B[0m {}", e);
        }

        let action = tui::run_action()?;
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
                let new_item = tui::run_tui(items.to_vec())?;
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

fn check_deps(deps: &[(&str, &str)]) -> Result<()> {
    let mut missing = Vec::new();
    for (cmd, desc) in deps {
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
            println!(
                "  \x1B[38;5;203m\u{2715}\x1B[0m  \x1B[1m{}\x1B[0m \x1B[38;5;245m- {}\x1B[0m",
                cmd, desc
            );
        }
        println!();
        println!("\x1B[38;5;245mRun the installer to fix:\x1B[0m");
        println!("  \x1B[1mcurl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash\x1B[0m");
        println!();
        anyhow::bail!("Missing required dependencies; install them before playback.");
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Settings { init, defaults }) = &cli.command {
        if *defaults {
            print!("{}", config::default_config_text());
        } else {
            let path = if *init {
                config::ensure_exists()?
            } else {
                config::config_file()?
            };
            println!("Config file: {}", path.display());
            if !path.exists() {
                println!("Run `media-cli settings --init` to create it.");
            }
        }
        return Ok(());
    }

    let config = config::load()?;

    let terminal = cli.terminal || config.terminal;

    let Some(command) = cli.command else {
        return run_main_menu(&config, terminal);
    };

    match command {
        Commands::Yt {
            query,
            player,
            quality,
            auto,
            no_detach,
        } => {
            check_deps(&[
                (effective_player(player, &config).command(), "media player"),
                ("yt-dlp", "YouTube search"),
            ])?;
            if sources::youtube::is_youtube_url(&query) {
                println!("\x1B[1;36m🔗 Opening link...\x1B[0m");
                history::add(&query, &query)?;
                player::play_youtube(
                    &query,
                    effective_player(player, &config),
                    effective_quality(quality.as_deref(), &config),
                    terminal || no_detach,
                )?;
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
                tui::run_tui(results.clone())?
            };

            if let Some(item) = selected {
                let p = player;
                let q = quality.clone();
                let t = terminal || no_detach;
                playback_loop(&results, item, |item| {
                    player::play_youtube(
                        &item.url,
                        effective_player(p, &config),
                        effective_quality(q.as_deref(), &config),
                        t,
                    )
                })?;
            }
        }
        Commands::Music { query } => {
            player::ensure_music_supported(config.player)?;
            check_deps(&[("mpv", "audio player"), ("yt-dlp", "YouTube search")])?;
            let results = sources::youtube::search(&query)?;
            if results.is_empty() {
                println!("\x1B[1;33m⚠ No results found\x1B[0m");
                return Ok(());
            }

            if let Some(item) = tui::run_tui(results.clone())? {
                playback_loop(&results, item, |item| {
                    player::play_music(&item.url, &item.title, &config.music)
                })?;
            }
        }
        Commands::File {
            path,
            player: player_opt,
        } => {
            check_deps(&[(
                effective_player(player_opt, &config).command(),
                "media player",
            )])?;
            let files = sources::local::scan(path.as_deref().or(config.local_dir.as_deref()))?;
            if files.is_empty() {
                println!("\x1B[1;33m⚠ No files found\x1B[0m");
                return Ok(());
            }

            if let Some(selected) = tui::run_tui(files)? {
                history::add(&selected.url, &selected.title)?;
                player::play(&selected.url, effective_player(player_opt, &config))?;
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
                println!(
                    "\x1B[1;36m│\x1B[0m  \x1B[1;90m{:2}.\x1B[0m {}",
                    idx + 1,
                    entry.title
                );
            }
            println!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m");
        }
        Commands::Twitch {
            channel,
            player,
            quality,
        } => {
            check_deps(&[
                (effective_player(player, &config).command(), "media player"),
                ("yt-dlp", "Twitch stream extraction"),
            ])?;
            let channel_name = if sources::twitch::is_twitch_url(&channel) {
                sources::twitch::extract_channel(&channel)
            } else {
                channel.clone()
            };

            let twitch_url = format!("https://twitch.tv/{}", channel_name);
            println!("\x1B[1;35m🔴 Connecting to:\x1B[0m {}", channel_name);
            history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;

            if terminal {
                player::play_stream_terminal(
                    &twitch_url,
                    effective_quality(quality.as_deref(), &config),
                )?;
            } else {
                player::play_stream(
                    &twitch_url,
                    effective_player(player, &config),
                    effective_quality(quality.as_deref(), &config),
                    false,
                )?;
            }
        }
        Commands::Anime {
            query,
            player,
            quality,
            mode,
        } => {
            check_deps(&[(effective_player(player, &config).command(), "media player")])?;
            let mode = mode.unwrap_or_else(|| config.anime_mode.clone());
            println!("\x1B[1;35m🎌 Searching anime...\x1B[0m");

            let results = sources::anime::search_anime(&query, &mode)?;
            if results.is_empty() {
                println!("\x1B[1;33m⚠ No results found\x1B[0m");
                return Ok(());
            }

            let selected = tui::run_tui(results)?;

            if let Some(anime) = selected {
                let show_id = anime.url.clone();
                let episodes = sources::anime::get_episodes(&show_id, &mode)?;

                if episodes.is_empty() {
                    println!("\x1B[1;33m⚠ No episodes found\x1B[0m");
                    return Ok(());
                }

                let ep_items: Vec<_> = episodes
                    .iter()
                    .map(|ep| MediaItem {
                        title: format!("Episode {}", ep),
                        url: show_id.clone(),
                        duration: None,
                        episode: Some(sources::EpisodeRef {
                            show_id: show_id.clone(),
                            episode: ep.clone(),
                        }),
                    })
                    .collect();

                let selected_ep = tui::run_tui(ep_items.clone())?;

                if let Some(episode) = selected_ep {
                    let anime_title = anime.title.clone();
                    let m = mode.clone();
                    let p = player;
                    let q = quality.clone();
                    let t = terminal;
                    playback_loop(&ep_items, episode, move |ep| {
                        println!("\x1B[1;36m▶ Playing {} - {}\x1B[0m", anime_title, ep.title);
                        if let Some(episode) = &ep.episode {
                            sources::anime::play_anime_episode(
                                &episode.show_id,
                                &episode.episode,
                                &m,
                                effective_player(p, &config),
                                effective_quality(q.as_deref(), &config),
                                t,
                            )
                        } else {
                            anyhow::bail!("Invalid episode URL format")
                        }
                    })?;
                }
            }
        }
        Commands::Settings { .. } => unreachable!("settings is handled before loading config"),
    }

    Ok(())
}

fn prompt_input(label: &str, icon: &str, section: &str) -> Result<String> {
    let prompt = format!("{} {}", icon, label);
    Ok(tui::run_input(section, &prompt, "Start typing...")?.unwrap_or_default())
}

fn run_main_menu(config: &config::Config, terminal: bool) -> Result<()> {
    loop {
        let choice = tui::run_home(VERSION, terminal)?;
        let result = (|| -> Result<bool> {
            match choice {
                tui::HomeAction::YouTube => {
                    check_deps(&[
                        (config.player.command(), "media player"),
                        ("yt-dlp", "YouTube search"),
                    ])?;
                    let query = prompt_input("Search or URL", "▶", "YouTube")?;
                    if query.is_empty() {
                        return Ok(false);
                    }

                    if sources::youtube::is_youtube_url(&query) {
                        println!("\x1B[1;36m🔗 Opening link...\x1B[0m");
                        history::add(&query, &query)?;
                        player::play_youtube(
                            &query,
                            config.player,
                            config.quality_arg(),
                            terminal,
                        )?;
                    } else {
                        let results = sources::youtube::search(&query)?;
                        if results.is_empty() {
                            tui::run_notice("Nothing found", "Try a different search query.")?;
                            return Ok(false);
                        }

                        if let Some(item) = tui::run_tui(results.clone())? {
                            playback_loop(&results, item, |item| {
                                player::play_youtube(
                                    &item.url,
                                    config.player,
                                    config.quality_arg(),
                                    terminal,
                                )
                            })?;
                        }
                    }
                }
                tui::HomeAction::Music => {
                    player::ensure_music_supported(config.player)?;
                    check_deps(&[("mpv", "audio player"), ("yt-dlp", "YouTube search")])?;
                    let query = prompt_input("Search music", "♪", "YouTube Music")?;
                    if query.is_empty() {
                        return Ok(false);
                    }

                    let results = sources::youtube::search(&query)?;
                    if results.is_empty() {
                        tui::run_notice("Nothing found", "Try a different search query.")?;
                        return Ok(false);
                    }

                    if let Some(item) = tui::run_tui(results.clone())? {
                        playback_loop(&results, item, |item| {
                            player::play_music(&item.url, &item.title, &config.music)
                        })?;
                    }
                }
                tui::HomeAction::Twitch => {
                    check_deps(&[
                        (config.player.command(), "media player"),
                        ("yt-dlp", "Twitch stream extraction"),
                    ])?;
                    let channel = prompt_input("Channel or URL", "●", "Twitch")?;
                    if channel.is_empty() {
                        return Ok(false);
                    }

                    let channel_name = if sources::twitch::is_twitch_url(&channel) {
                        sources::twitch::extract_channel(&channel)
                    } else {
                        channel.to_string()
                    };

                    let twitch_url = format!("https://twitch.tv/{}", channel_name);
                    println!("\x1B[1;35m🔴 Connecting to:\x1B[0m {}", channel_name);
                    history::add(&twitch_url, &format!("Twitch: {}", channel_name))?;
                    player::play_stream(
                        &twitch_url,
                        config.player,
                        config.quality_arg(),
                        terminal,
                    )?;
                }
                tui::HomeAction::Anime => {
                    check_deps(&[(config.player.command(), "media player")])?;
                    let query = prompt_input("Search anime", "◆", "Anime")?;
                    if query.is_empty() {
                        return Ok(false);
                    }

                    println!("\x1B[38;5;245m⏳ Searching anime...\x1B[0m");
                    let results = sources::anime::search_anime(&query, &config.anime_mode)?;
                    if results.is_empty() {
                        tui::run_notice("Nothing found", "Try a different anime title.")?;
                        return Ok(false);
                    }

                    if let Some(anime) = tui::run_tui(results)? {
                        let episodes =
                            sources::anime::get_episodes(&anime.url, &config.anime_mode)?;
                        if episodes.is_empty() {
                            tui::run_notice(
                                "No episodes",
                                "This title has no available episodes.",
                            )?;
                            return Ok(false);
                        }

                        let ep_items: Vec<_> = episodes
                            .iter()
                            .map(|ep| MediaItem {
                                title: format!("Episode {}", ep),
                                url: anime.url.clone(),
                                duration: None,
                                episode: Some(sources::EpisodeRef {
                                    show_id: anime.url.clone(),
                                    episode: ep.clone(),
                                }),
                            })
                            .collect();

                        if let Some(episode) = tui::run_tui(ep_items.clone())? {
                            let anime_title = anime.title.clone();
                            playback_loop(&ep_items, episode, |ep| {
                                println!(
                                    "\x1B[1;36m▶ Playing {} - {}\x1B[0m",
                                    anime_title, ep.title
                                );
                                if let Some(episode) = &ep.episode {
                                    sources::anime::play_anime_episode(
                                        &episode.show_id,
                                        &episode.episode,
                                        &config.anime_mode,
                                        config.player,
                                        config.quality_arg(),
                                        terminal,
                                    )
                                } else {
                                    anyhow::bail!("Invalid episode URL format")
                                }
                            })?;
                        }
                    }
                }
                tui::HomeAction::Local => {
                    check_deps(&[(config.player.command(), "media player")])?;
                    let path = prompt_input("Path (Enter = ~/Videos)", "■", "Local Files")?;
                    let path_opt = if path.is_empty() {
                        config.local_dir.as_deref()
                    } else {
                        Some(path.as_str())
                    };
                    let files = sources::local::scan(path_opt)?;

                    if files.is_empty() {
                        tui::run_notice(
                            "No media files",
                            "Choose another directory or add media files.",
                        )?;
                        return Ok(false);
                    }

                    if let Some(selected) = tui::run_tui(files)? {
                        history::add(&selected.url, &selected.title)?;
                        player::play(&selected.url, config.player)?;
                    }
                }
                tui::HomeAction::History => {
                    let entries = history::load()?;
                    if entries.is_empty() {
                        tui::run_notice("History", "Your playback history is empty.")?;
                        return Ok(false);
                    }

                    let items = entries
                        .into_iter()
                        .map(|entry| MediaItem {
                            title: entry.title,
                            url: entry.url,
                            duration: None,
                            episode: entry.episode,
                        })
                        .collect();
                    if let Some(selected) = tui::run_tui(items)? {
                        play_history_item(&selected, config, terminal)?;
                    }
                }
                tui::HomeAction::Settings => {
                    let path = config::config_file()?;
                    let message = format!(
                        "Config file:\n  {}\n\nCreate it:\n  media-cli settings --init\n\nCurrent defaults:\n  player = {}\n  quality = {}\n  terminal = {}\n  anime_mode = {}\n  music.visualizer_style = {:?}\n  music.sensitivity = {:.1}",
                        path.display(),
                        config.player,
                        config.quality,
                        config.terminal,
                        config.anime_mode,
                        config.music.visualizer_style,
                        config.music.sensitivity
                    );
                    tui::run_notice("Settings", &message)?;
                }
                tui::HomeAction::Quit => return Ok(true),
            }
            Ok(false)
        })();

        match result {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => tui::run_notice("Something went wrong", &format!("{error:#}"))?,
        }
    }

    Ok(())
}
