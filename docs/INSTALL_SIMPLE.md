# Simple Installation

## Recommended: One Command

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
```

The installer downloads the correct release binary and installs required runtime
dependencies (`mpv` and `yt-dlp`) where possible.

## Manual Release Download

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
media-cli
```

For macOS and Windows, download the matching asset from
[GitHub Releases](https://github.com/sqrilizz/media-cli/releases/latest).

## Build from Source

```bash
git clone https://github.com/sqrilizz/media-cli
cd media-cli
cargo build --release
sudo cp target/release/media-cli /usr/local/bin/
```

## First Run

```bash
media-cli
```

You'll see an interactive menu. Choose what you want to watch.

## Quick Examples

```bash
media-cli yt "funny cats"
media-cli music "lofi hip hop"
media-cli anime "naruto"
media-cli twitch shroud
```

## Need Help?

```bash
media-cli --help
```

Or check [README.md](../README.md) for full documentation.
