# 🚀 GitHub Release Setup - Complete Guide

This guide will help you set up automatic binary releases for your media-cli project.

## What You Get

After setup, users can install your app with one command:

```bash
curl -L https://github.com/YOURsqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
./media-cli
```

## Setup Steps

### 1. Replace sqrilizz in Files

Replace `sqrilizz` with your GitHub username in:

- `README.md` (all occurrences)
- `install.sh` (line 5)
- `INSTALL_ONELINER.md` (all occurrences)

Quick replace command:

```bash
# macOS
find . -type f \( -name "*.md" -o -name "*.sh" \) -exec sed -i '' 's/sqrilizz/yourusername/g' {} +

# Linux
find . -type f \( -name "*.md" -o -name "*.sh" \) -exec sed -i 's/sqrilizz/yourusername/g' {} +
```

### 2. Push to GitHub

```bash
# If not initialized
git init
git add .
git commit -m "Initial commit with auto-release setup"

# Add remote (replace YOURsqrilizz)
git remote add origin https://github.com/YOURsqrilizz/media-cli.git

# Push
git branch -M main
git push -u origin main
```

### 3. Create First Release

```bash
# Create and push tag
git tag v0.2.0
git push origin v0.2.0
```

### 4. Wait for Build

1. Go to: `https://github.com/YOURsqrilizz/media-cli/actions`
2. Watch the build process (takes ~5-10 minutes)
3. When complete, check: `https://github.com/YOURsqrilizz/media-cli/releases`

### 5. Test Installation

```bash
# Download and test
curl -L https://github.com/YOURsqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
./media-cli --help
```

## What Gets Built

The GitHub Action automatically builds:

- ✅ Linux x86_64 (standard)
- ✅ Linux x86_64-musl (static binary)
- ✅ macOS Intel (x86_64)
- ✅ macOS Apple Silicon (ARM64)
- ✅ Windows x86_64

## Files Created

### `.github/workflows/release.yml`

- Automatic build on tag push
- Builds for all platforms
- Creates GitHub release
- Uploads binaries

### `install.sh`

- Auto-detects OS and architecture
- Downloads correct binary
- Installs to `/usr/local/bin`

### Documentation

- `SETUP_GITHUB.md` - This guide
- `CONTRIBUTING.md` - For contributors
- `RELEASE.md` - Release checklist
- `INSTALL_ONELINER.md` - One-line install commands

## Future Releases

To create new releases:

```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git add .
git commit -m "Release v0.2.0"
git push

# 3. Create and push tag
git tag v0.2.0
git push origin v0.2.0

# 4. GitHub Actions does the rest!
```

## Troubleshooting

### Build Fails

- Check Actions tab for errors
- Test locally: `cargo build --release`
- Fix errors and push new tag

### No Binaries in Release

- Check if all jobs completed
- Look for "Upload artifact" step in logs
- Re-run failed jobs

### Can't Download Binary

- Check release exists: `https://github.com/YOURsqrilizz/media-cli/releases`
- Verify binary name matches download URL
- Check repository is public

## Share Your Project

After setup, share these commands:

**One-line install:**

```bash
curl -L https://github.com/YOURsqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli && chmod +x media-cli && sudo mv media-cli /usr/local/bin/
```

**Install script:**

```bash
curl -fsSL https://raw.githubusercontent.com/YOURsqrilizz/media-cli/main/install.sh | bash
```

## Next Steps

1. ✅ Replace sqrilizz in all files
2. ✅ Push to GitHub
3. ✅ Create first tag (v0.2.0)
4. ✅ Wait for build
5. ✅ Test installation
6. ✅ Share with users!

## Support

If you have issues:

1. Check GitHub Actions logs
2. Read CONTRIBUTING.md
3. Open an issue on GitHub

---

**That's it! Your project now has automatic binary releases! 🎉**
