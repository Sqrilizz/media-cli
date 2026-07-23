# Release Checklist

## Before Release

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md (if exists)
3. Test all features:

   - YouTube playback
   - Music mode
   - Anime playback
   - Twitch streams
   - Local files

4. Run tests: `cargo test`
5. Check for warnings: `cargo clippy`
6. Format code: `cargo fmt`

## Creating Release

1. Commit all changes:

```bash
git add .
git commit -m "Release v0.x.x"
git push
```

1. Create and push tag:

```bash
git tag v0.x.x
git push origin v0.x.x
```

1. GitHub Actions will automatically:

- Build binaries for all platforms
- Create GitHub release
- Upload binaries

## After Release

1. Test installation from release:

```bash
curl -L https://github.com/sqrilizz/media-cli/releases/latest/download/media-cli-linux-x86_64 -o media-cli
chmod +x media-cli
./media-cli --help
```

1. Update README.md if needed
2. Announce release (optional)

## Manual Release (if needed)

If GitHub Actions fails, build manually:

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/media-cli

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS ARM
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

Then create release manually on GitHub and upload binaries.
