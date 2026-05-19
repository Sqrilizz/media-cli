use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "media-cli")]
#[command(version)]
#[command(about = "Universal CLI media player", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    #[arg(short = 't', long, global = true, help = "Play video in terminal (inline)")]
    pub terminal: bool,

    #[arg(long, global = true, num_args = 0..=1, default_missing_value = "auto")]
    pub proxy: Option<String>,
    
    #[arg(long, global = true)]
    pub bypass_help: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Yt {
        query: String,
        #[arg(short, long, default_value = "mpv")]
        player: Option<String>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(short, long)]
        auto: bool,
        #[arg(long)]
        no_detach: bool,
        #[arg(long)]
        proxy: Option<String>,
    },
    Music {
        query: String,
        #[arg(long)]
        proxy: Option<String>,
    },
    File {
        path: Option<String>,
        #[arg(short, long, default_value = "mpv")]
        player: Option<String>,
    },
    History {
        #[arg(short, long)]
        clear: bool,
    },
    Twitch {
        channel: String,
        #[arg(short, long, default_value = "mpv")]
        player: Option<String>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(long)]
        proxy: Option<String>,
    },
    Anime {
        query: String,
        #[arg(short, long, default_value = "mpv")]
        player: Option<String>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(long)]
        proxy: Option<String>,
        #[arg(long, default_value = "sub")]
        mode: String,
        #[arg(long)]
        gui: bool,
    },
}
