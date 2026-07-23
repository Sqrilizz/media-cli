# Release Process

## Automatic Release Flow

```
Developer                GitHub Actions              Users
    |                           |                       |
    | 1. git tag v0.2.0        |                       |
    |------------------------->|                       |
    |                           |                       |
    |                    2. Trigger workflow           |
    |                           |                       |
    |                    3. Build Linux x86_64         |
    |                    4. Build Linux musl           |
    |                    5. Build macOS Intel          |
    |                    6. Build macOS ARM            |
    |                    7. Build Windows              |
    |                           |                       |
    |                    8. Create Release             |
    |                    9. Upload Binaries            |
    |                           |                       |
    |                           |  10. Download binary  |
    |                           |<----------------------|
    |                           |                       |
    |                           |  11. chmod +x         |
    |                           |  12. Run media-cli    |
```

## Manual Steps

### Create Release
```bash
git tag v0.2.0
git push origin v0.2.0
```

### Everything Else is Automatic!

GitHub Actions will:
- ✅ Build for all platforms
- ✅ Strip binaries (reduce size)
- ✅ Create GitHub release
- ✅ Upload all binaries
- ✅ Generate release notes

## User Installation

### One Command
```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
./media-cli
```

### Or Install Script
```bash
curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash
```

## Release Checklist

Before creating tag:
- [ ] Update version in Cargo.toml
- [ ] Test all features
- [ ] Run `cargo test`
- [ ] Run `cargo clippy`
- [ ] Commit all changes

Create release:
- [ ] `git tag vX.Y.Z`
- [ ] `git push origin vX.Y.Z`
- [ ] Wait for GitHub Actions
- [ ] Test installation
- [ ] Announce release

## Supported Platforms

| Platform | Binary Name | Size |
|----------|-------------|------|
| Linux x86_64 | media-cli-linux-x86_64 | ~2MB |
| Linux musl | media-cli-linux-x86_64-musl | ~2MB |
| macOS Intel | media-cli-macos-x86_64 | ~2MB |
| macOS ARM | media-cli-macos-arm64 | ~2MB |
| Windows | media-cli-windows-x86_64.exe | ~2MB |
