# How to Use media-cli

## Install

### Recommended Installer

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
media-cli
```

### From GitHub Releases

Download the matching binary from
[GitHub Releases](https://github.com/sqrilizz/media-cli/releases/latest):

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
media-cli
```

### Build from Source

```bash
git clone https://github.com/sqrilizz/media-cli
cd media-cli
cargo build --release
sudo cp target/release/media-cli /usr/local/bin/
```

## Runtime Dependencies

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp

# Arch Linux
sudo pacman -S mpv yt-dlp

# macOS
brew install mpv yt-dlp
```

## Usage Examples

```bash
# Interactive menu
media-cli

# YouTube
media-cli yt "funny cats"

# Music (audio only)
media-cli music "lofi hip hop"

# Anime
media-cli anime "naruto"

# Twitch stream
media-cli twitch shroud

# Local files
media-cli file ~/Videos
```

## Getting Help

```bash
media-cli --help
```

## Documentation

- [README.md](../README.md) - Full documentation
- [EXAMPLES.md](EXAMPLES.md) - Usage examples
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick reference

## Troubleshooting

### Dependencies missing

```bash
which mpv yt-dlp
```

Install missing tools with your system package manager.

### Permission denied

```bash
chmod +x media-cli
```

For system-wide installs, move the binary into a directory on `PATH`, for example
`/usr/local/bin`.

## Updates

Use the installer again or download the latest asset from GitHub Releases:

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
```
