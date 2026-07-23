# One-Line Installation

Copy and paste these commands to install media-cli:

## Linux (x86_64)

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/ && echo "✅ Installed! Run: media-cli"
```

## macOS (Intel)

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-macos-x86_64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/ && echo "✅ Installed! Run: media-cli"
```

## macOS (Apple Silicon)

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-macos-arm64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/ && echo "✅ Installed! Run: media-cli"
```

## Install Script (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
```

## After Installation

The installer automatically installs these dependencies. For manual recovery:

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp

# Arch Linux
sudo pacman -S mpv yt-dlp

# macOS
brew install mpv yt-dlp
```

Then run:

```bash
media-cli
```
