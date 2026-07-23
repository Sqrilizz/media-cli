use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "media-cli")]
#[command(version)]
#[command(about = "Universal CLI media player", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(
        short = 't',
        long,
        global = true,
        help = "Play video in terminal (inline). Overrides config.toml"
    )]
    pub terminal: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Search or play a YouTube video")]
    Yt {
        query: String,
        #[arg(short, long)]
        player: Option<crate::player::Player>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(short, long)]
        auto: bool,
        #[arg(long)]
        no_detach: bool,
    },
    #[command(about = "Search YouTube Music and play audio only")]
    Music { query: String },
    #[command(about = "Browse and play local media files")]
    File {
        path: Option<String>,
        #[arg(short, long)]
        player: Option<crate::player::Player>,
    },
    #[command(about = "Show or clear watch history")]
    History {
        #[arg(short, long)]
        clear: bool,
    },
    #[command(about = "Play a Twitch channel")]
    Twitch {
        channel: String,
        #[arg(short, long)]
        player: Option<crate::player::Player>,
        #[arg(short, long)]
        quality: Option<String>,
    },
    #[command(about = "Search and watch anime")]
    Anime {
        query: String,
        #[arg(short, long)]
        player: Option<crate::player::Player>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(long)]
        mode: Option<String>,
    },
    #[command(about = "Show or create the user config file")]
    Settings {
        #[arg(
            long,
            help = "Create config.toml with documented defaults if it does not exist"
        )]
        init: bool,
        #[arg(long, help = "Print the default config template")]
        defaults: bool,
    },
}
