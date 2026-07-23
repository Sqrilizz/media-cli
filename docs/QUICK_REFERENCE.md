# Quick Reference Card

## Installation

```bash
# Recommended installer
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash

# Manual Linux x86_64 release download
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
```

## Usage

```bash
# Interactive menu
media-cli

# YouTube
media-cli yt "search query"
media-cli yt "https://youtube.com/watch?v=..."

# Music
media-cli music "song name"

# Anime
media-cli anime "anime title"

# Twitch
media-cli twitch channel_name

# Local files
media-cli file
media-cli file /path/to/folder

# History
media-cli history
media-cli history --clear

# Settings
media-cli settings --init
media-cli settings --defaults
```

## Options

```bash
--quality <Q>     Video quality (720p, 1080p, best), overrides config
--player <P>      Player (mpv, vlc), overrides config
--terminal, -t    Play in terminal (kitty), overrides config
```

## Config

```bash
media-cli settings --init
# Edit ~/.config/media-cli/config.toml
```

Config supports `player`, `quality`, `terminal`, `local_dir`, `anime_mode`, and Music Deck visualizer settings.

Music Deck keys: `Space` pause, `M` mute, `V` visualizer style, `Q` stop.

## Developer Commands

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

## Dependencies

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp

# Arch
sudo pacman -S mpv yt-dlp

# macOS
brew install mpv yt-dlp
```

## Files Overview

| File | Purpose |
| --- | --- |
| `.github/workflows/ci.yml` | Format, lint, and test checks |
| `.github/workflows/release.yml` | Release builds on tag push |
| `scripts/install.sh` | Main installation script |
| `README.md` | Main documentation |
| `docs/EXAMPLES.md` | Usage examples |
| `docs/CONFIG.md` | Configuration reference |
| `docs/CONTRIBUTING.md` | Development guide |
| `docs/RELEASE.md` | Release checklist |
