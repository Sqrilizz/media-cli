# 🚀 START HERE - Quick Setup Guide

## Your GitHub: sqrilizz

All files are already configured with your username! ✅

## What You Have Now

1. ✅ **Pre-compiled binary** (`media-cli`) - ready to share
2. ✅ **GitHub Actions** - auto-builds on new releases
3. ✅ **Install scripts** - easy installation for users
4. ✅ **Full documentation** - README, examples, guides

## Quick Commands for Users

Share these with people who want to use your app:

### Super Quick Install
```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
./media-cli
```

### One-Line Install (system-wide)
```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/ && media-cli
```

### Install Script
```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/install-direct.sh | bash
```

## Next Steps

### 1. Push to GitHub

```bash
# Add all files
git add .

# Commit
git commit -m "Add pre-compiled binary and auto-release system"

# Push (if repo exists)
git push

# Or create new repo and push
git remote add origin https://github.com/sqrilizz/media-cli.git
git branch -M main
git push -u origin main
```

### 2. Create First Release (Optional)

If you want automatic builds for all platforms:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will build binaries for:
- Linux x86_64
- Linux musl (Alpine)
- macOS Intel
- macOS ARM
- Windows

### 3. Share with Users

After pushing, users can install with:
```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
./media-cli
```

## File Structure

```
media-cli/
├── media-cli              # Pre-compiled binary (Linux x86_64)
├── media-cli.sha256       # Checksum for verification
├── install-direct.sh      # Installation script
├── README.md              # Main documentation
├── EXAMPLES.md            # Usage examples
├── PROXY.md               # Proxy configuration
├── HOW_TO_USE.md          # User guide
├── QUICK_REFERENCE.md     # Quick reference
└── .github/
    └── workflows/
        └── release.yml    # Auto-build on releases
```

## Update Binary

When you make changes:

```bash
# 1. Build
cargo build --release

# 2. Strip
strip target/release/media-cli

# 3. Copy
cp target/release/media-cli ./media-cli

# 4. Update checksum
sha256sum media-cli > media-cli.sha256

# 5. Commit
git add media-cli media-cli.sha256
git commit -m "Update binary"
git push
```

## Documentation Files

- `README.md` - Main documentation with features and installation
- `EXAMPLES.md` - Real-world usage examples
- `PROXY.md` - Proxy and bypass configuration
- `HOW_TO_USE.md` - Simple user guide
- `QUICK_REFERENCE.md` - Command cheat sheet
- `INSTALL_SIMPLE.md` - Simplified installation guide
- `GITHUB_RELEASE_SETUP.md` - Release system setup
- `CONTRIBUTING.md` - For contributors
- `BINARY_INFO.md` - Binary information

## Installation Methods

| Method | Command | Best For |
|--------|---------|----------|
| Direct download | `curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli` | Quick testing |
| Install script | `curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/install-direct.sh \| bash` | Easy install |
| GitHub Releases | Download from releases page | All platforms |
| Build from source | `cargo build --release` | Development |

## What Users Get

After installation, users can:
- ✅ Watch YouTube videos
- ✅ Listen to music (audio-only mode)
- ✅ Watch anime (multiple providers)
- ✅ Watch Twitch streams
- ✅ Play local video files
- ✅ Use proxy/VPN for blocked content
- ✅ Interactive menu with fzf
- ✅ History tracking

## Example Usage

```bash
# Interactive menu
media-cli

# YouTube
media-cli yt "funny cats"

# Music
media-cli music "lofi hip hop"

# Anime
media-cli anime "naruto"

# Twitch
media-cli twitch shroud

# With proxy
media-cli --proxy auto yt "video"
```

## Support

Users can:
1. Read documentation in README.md
2. Check examples in EXAMPLES.md
3. Open issues on GitHub
4. Check releases for updates

---

## Ready to Share! 🎉

Your app is ready to be shared. Just push to GitHub and share the install command:

```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli && chmod +x media-cli && ./media-cli
```

**Repository:** https://github.com/sqrilizz/media-cli
