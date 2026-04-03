# Quick Reference Card

## Installation (Users)

### Super Quick (Direct from repo)
```bash
# One command
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/

# Or step by step
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
```

**Note:** Binary is for Linux x86_64. For other platforms, use releases below.

### From GitHub Releases (All Platforms)

```bash
# Linux
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/

# macOS Intel
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-macos-x86_64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/

# macOS ARM
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-macos-arm64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/
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

# With proxy
media-cli --proxy auto yt "video"
media-cli --proxy socks5://127.0.0.1:1080 anime "title"
```

## Options

```bash
--proxy <URL>       Use proxy (or 'auto' for auto-detection)
--quality <Q>       Video quality (720p, 1080p, best)
--player <P>        Player (mpv, vlc)
--terminal, -t      Play in terminal (kitty)
--gui               Use GUI selector
--bypass-help       Show bypass help
```

## For Developers

### Setup Release System
1. Replace `sqrilizz` in all files with your GitHub username
2. Push to GitHub
3. Create tag: `git tag v0.1.0 && git push origin v0.1.0`
4. Wait for GitHub Actions to build
5. Done! Binaries at: `https://github.com/YOURsqrilizz/media-cli/releases`

### Create New Release
```bash
# Update Cargo.toml version
git add .
git commit -m "Release v0.2.0"
git push
git tag v0.2.0
git push origin v0.2.0
```

### Build Locally
```bash
cargo build --release
./target/release/media-cli
```

## Dependencies

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp streamlink fzf curl

# Arch
sudo pacman -S mpv yt-dlp streamlink fzf curl

# macOS
brew install mpv yt-dlp streamlink fzf curl
```

## Files Overview

| File | Purpose |
|------|---------|
| `.github/workflows/release.yml` | Auto-build on tag push |
| `install.sh` | Installation script |
| `README.md` | Main documentation |
| `EXAMPLES.md` | Usage examples |
| `PROXY.md` | Proxy configuration |
| `GITHUB_RELEASE_SETUP.md` | Setup guide for releases |
| `CONTRIBUTING.md` | Development guide |
| `RELEASE.md` | Release checklist |

## Links

- Releases: `https://github.com/sqrilizz/media-cli/releases`
- Actions: `https://github.com/sqrilizz/media-cli/actions`
- Issues: `https://github.com/sqrilizz/media-cli/issues`

## Support

1. Check documentation
2. Read EXAMPLES.md
3. Check GitHub Issues
4. Open new issue if needed
