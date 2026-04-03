# How to Use media-cli

## For Users: Download and Run

### Fastest Way (Linux x86_64)

```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
./media-cli
```

That's it! No compilation, no setup, just download and run.

### Install System-Wide

```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
media-cli
```

### Verify Download (Optional)

```bash
# Download checksum
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli.sha256 -o media-cli.sha256

# Verify
sha256sum -c media-cli.sha256
```

## For Developers: Update Binary

When you make changes and want to update the binary in the repo:

```bash
# 1. Build release
cargo build --release

# 2. Strip debug symbols (reduces size)
strip target/release/media-cli

# 3. Copy to repo root
cp target/release/media-cli ./media-cli

# 4. Update checksum
sha256sum media-cli > media-cli.sha256

# 5. Commit and push
git add media-cli media-cli.sha256
git commit -m "Update binary to v0.x.x"
git push
```

## Installation Methods Comparison

| Method | Speed | Platforms | Best For |
|--------|-------|-----------|----------|
| Direct download | ⚡ Fastest | Linux x86_64 | Quick testing |
| GitHub Releases | 🚀 Fast | All platforms | Production use |
| Build from source | 🐌 Slow | All platforms | Development |

## What's Included

- `media-cli` - Pre-compiled binary (Linux x86_64)
- `media-cli.sha256` - Checksum for verification
- `install-direct.sh` - Installation script
- `INSTALL_SIMPLE.md` - Simple installation guide

## Platform Support

### ✅ Direct Binary (in repo)
- Linux x86_64 with glibc 2.31+
- Ubuntu 20.04+, Debian 11+, Arch, Fedora 35+

### ✅ GitHub Releases
- Linux x86_64 (glibc)
- Linux x86_64-musl (Alpine, static)
- macOS Intel (x86_64)
- macOS Apple Silicon (ARM64)
- Windows x86_64

## First Time Setup

After downloading, install dependencies:

```bash
# Ubuntu/Debian
sudo apt install mpv yt-dlp streamlink fzf curl

# Arch Linux
sudo pacman -S mpv yt-dlp streamlink fzf curl

# macOS
brew install mpv yt-dlp streamlink fzf curl
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

# With proxy
media-cli --proxy auto yt "video"
```

## Getting Help

```bash
# Show help
media-cli --help

# Show bypass/proxy help
media-cli --bypass-help
```

## Documentation

- [README.md](README.md) - Full documentation
- [EXAMPLES.md](EXAMPLES.md) - Usage examples
- [PROXY.md](PROXY.md) - Proxy configuration
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick reference

## Troubleshooting

### Binary won't run
```bash
# Check if executable
ls -l media-cli

# Make executable
chmod +x media-cli

# Check platform
file media-cli
```

### Dependencies missing
```bash
# Check what's installed
which mpv yt-dlp streamlink fzf

# Install missing ones
sudo apt install mpv yt-dlp streamlink fzf curl
```

### Permission denied
```bash
# Run with sudo for system-wide install
sudo mv media-cli /usr/local/bin/

# Or run from current directory
./media-cli
```

## Updates

To get the latest version:

```bash
# Re-download
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli

# Or check releases
# https://github.com/sqrilizz/media-cli/releases/latest
```

---

**Enjoy! 🎉**
