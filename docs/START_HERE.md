# 🚀 START HERE - Quick Setup Guide

## Project Status

1. ✅ Rust CLI/TUI application
2. ✅ GitHub Actions release workflow for multi-platform binaries
3. ✅ CI workflow for formatting, linting, and tests
4. ✅ Install scripts for users
5. ✅ Documentation and examples

## Quick Install for Users

```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
media-cli
```

## Developer Workflow

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

## Create a Release

```bash
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions will build binaries for:

- Linux x86_64
- Linux musl
- macOS Intel
- macOS Apple Silicon
- Windows x86_64

The release also includes `SHA256SUMS` for verification.

## File Structure

```text
media-cli/
├── src/                    # Rust source code
├── scripts/                # Installation scripts
├── docs/                   # Documentation
├── .github/workflows/      # CI and release automation
├── README.md               # Main documentation
├── CHANGELOG.md            # Version history
└── Cargo.toml              # Rust package metadata
```

## Documentation Files

- `README.md` - Main documentation with features and installation
- `docs/EXAMPLES.md` - Usage examples
- `docs/HOW_TO_USE.md` - User guide
- `docs/QUICK_REFERENCE.md` - Command cheat sheet
- `docs/INSTALL_SIMPLE.md` - Simplified installation guide
- `docs/GITHUB_RELEASE_SETUP.md` - Release system setup
- `docs/CONTRIBUTING.md` - Contributor guide
- `docs/BINARY_INFO.md` - Release binary information

## Example Usage

```bash
media-cli
media-cli yt "funny cats"
media-cli music "lofi hip hop"
media-cli anime "naruto"
media-cli twitch shroud
```
