# Simple Installation

## Super Quick (One Command)

```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/ && media-cli
```

That's it! 🎉

## Step by Step

```bash
# 1. Download
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli

# 2. Make executable
chmod +x media-cli

# 3. Move to PATH (optional, but recommended)
sudo mv media-cli /usr/local/bin/

# 4. Run
media-cli
```

## Or Just Run Locally

```bash
# Download
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli

# Make executable
chmod +x media-cli

# Run from current directory
./media-cli
```

## Install Dependencies

After installing media-cli, install these tools:

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp streamlink fzf curl

# Arch Linux
sudo pacman -S mpv yt-dlp streamlink fzf curl

# macOS
brew install mpv yt-dlp streamlink fzf curl
```

## First Run

```bash
media-cli
```

You'll see an interactive menu. Choose what you want to watch!

## Quick Examples

```bash
# YouTube
media-cli yt "funny cats"

# Music
media-cli music "lofi hip hop"

# Anime
media-cli anime "naruto"

# Twitch
media-cli twitch shroud
```

## Need Help?

```bash
media-cli --help
```

Or check [README.md](README.md) for full documentation.

---

**Note:** The binary in the repo is for Linux x86_64. If you're on macOS or Windows, download from [releases](https://github.com/sqrilizz/media-cli/releases/latest) instead.
