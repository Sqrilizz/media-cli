# Contributing to media-cli

## Development Setup

### Prerequisites

- Rust 1.74+ (`rustup install stable`)
- Git

### Clone and Build

```bash
git clone https://github.com/sqrilizz/media-cli
cd media-cli
cargo build
cargo run -- --help
```

### Testing

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Making Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test your changes
5. Commit (`git commit -m 'Add amazing feature'`)
6. Push (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Code Style

- Follow Rust conventions
- Use `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Add comments for complex logic
- Write tests for new features

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml`
2. Create and push a tag:

```bash
git tag v0.2.0
git push origin v0.2.0
```

1. GitHub Actions will build and create release automatically
