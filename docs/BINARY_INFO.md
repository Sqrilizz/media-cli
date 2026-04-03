# Binary Information

## About the Binary

The `media-cli` binary in this repository is:
- **Platform:** Linux x86_64
- **Size:** ~1.6 MB (stripped)
- **Built with:** Rust (release mode)
- **Static linking:** No (requires glibc)

## Quick Download

```bash
curl -L https://raw.githubusercontent.com/sqrilizz/media-cli/main/media-cli -o media-cli
chmod +x media-cli
./media-cli
```

## Compatibility

### ✅ Works on:
- Ubuntu 20.04+
- Debian 11+
- Arch Linux
- Fedora 35+
- Most modern Linux distributions with glibc 2.31+

### ❌ Won't work on:
- macOS (use releases for macOS binaries)
- Windows (use releases for Windows binaries)
- Alpine Linux (needs musl binary from releases)
- Very old Linux distributions

## For Other Platforms

Download from [GitHub Releases](https://github.com/sqrilizz/media-cli/releases/latest):
- Linux x86_64 (glibc)
- Linux x86_64-musl (static, works on Alpine)
- macOS Intel (x86_64)
- macOS Apple Silicon (ARM64)
- Windows x86_64

## Updating the Binary

To update the binary in the repository:

```bash
# Build release
cargo build --release

# Strip debug symbols
strip target/release/media-cli

# Copy to repo root
cp target/release/media-cli ./media-cli

# Commit
git add media-cli
git commit -m "Update binary"
git push
```

## Why Include Binary?

Including the binary makes it super easy for users to try the app:
- No compilation needed
- No Rust toolchain required
- One command to download and run
- Perfect for quick testing

For production use, we recommend:
1. Using GitHub Releases (all platforms)
2. Building from source (most secure)
3. Package managers (when available)

## Security Note

Always verify the source when downloading binaries. This binary is built from the source code in this repository. You can verify by:
1. Checking the commit that updated the binary
2. Building from source and comparing checksums
3. Using GitHub Releases (signed by GitHub Actions)

## Checksum

To verify the binary:

```bash
sha256sum media-cli
```

Compare with the checksum in the commit message that updated the binary.
