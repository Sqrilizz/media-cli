# GitHub Setup Guide

This guide explains how to set up your repository for automatic releases.

## Step 1: Update Repository URLs

Replace `sqrilizz` with your actual GitHub username in these files:

### 1. `install.sh`
```bash
REPO="sqrilizz/media-cli"  # Change to: REPO="yourusername/media-cli"
```

### 2. `README.md`
Replace all instances of `sqrilizz` with your GitHub username:
- Installation commands
- Release links

### 3. `.github/workflows/release.yml`
No changes needed - it uses `${{ github.repository }}` automatically.

## Step 2: Push to GitHub

```bash
# Initialize git (if not already)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit"

# Add remote (replace sqrilizz)
git remote add origin https://github.com/sqrilizz/media-cli.git

# Push
git branch -M main
git push -u origin main
```

## Step 3: Create First Release

```bash
# Create and push a tag
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will automatically:
1. Build binaries for Linux, macOS, and Windows
2. Create a GitHub release
3. Upload all binaries

## Step 4: Verify Release

1. Go to `https://github.com/sqrilizz/media-cli/releases`
2. You should see `v0.1.0` release with binaries
3. Test installation:

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
./media-cli --help
```

## Step 5: Update Install Script

After first successful release, update `install.sh` and `README.md` with your actual username.

## Troubleshooting

### GitHub Actions Not Running

1. Check Actions tab: `https://github.com/sqrilizz/media-cli/actions`
2. Ensure Actions are enabled in repository settings
3. Check workflow file syntax

### Build Failures

1. Check Actions logs for errors
2. Test build locally: `cargo build --release`
3. Fix errors and push again

### Missing Binaries

1. Check if all jobs completed successfully
2. Verify artifact upload in Actions logs
3. Re-run failed jobs if needed

## Quick Find & Replace

Use this command to replace sqrilizz in all files:

```bash
# macOS
find . -type f -name "*.md" -o -name "*.sh" | xargs sed -i '' 's/sqrilizz/yourusername/g'

# Linux
find . -type f -name "*.md" -o -name "*.sh" | xargs sed -i 's/sqrilizz/yourusername/g'
```

Replace `yourusername` with your actual GitHub username.
