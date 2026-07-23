# Binary Information

`media-cli` binaries are produced by GitHub Actions and published on the
[latest release](https://github.com/sqrilizz/media-cli/releases/latest).

## Release Assets

Current release workflow builds:

- Linux x86_64 glibc: `media-cli-linux-x86_64`
- Linux x86_64 musl/static-friendly: `media-cli-linux-x86_64-musl`
- macOS Intel: `media-cli-macos-x86_64`
- macOS Apple Silicon: `media-cli-macos-arm64`
- Windows x86_64: `media-cli-windows-x86_64.exe`

## Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
```

The installer detects your platform, installs runtime dependencies, and downloads
the matching release asset.

## Manual Download

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
sudo mv media-cli /usr/local/bin/
```

## Verification

Each release includes `SHA256SUMS`:

```bash
sha256sum -c SHA256SUMS
```

## Build Locally

```bash
cargo build --release
./target/release/media-cli --help
```

Use local builds for development and GitHub Releases for distribution.
