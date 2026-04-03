#!/bin/bash

set -e

# NOTE: Replace sqrilizz with your actual GitHub username before using!
REPO="sqrilizz/media-cli"  # TODO: Replace sqrilizz with your GitHub username
VERSION="latest"
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing media-cli..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux)
        case "$ARCH" in
            x86_64)
                BINARY="media-cli-linux-x86_64"
                ;;
            *)
                echo "❌ Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64)
                BINARY="media-cli-macos-x86_64"
                ;;
            arm64)
                BINARY="media-cli-macos-arm64"
                ;;
            *)
                echo "❌ Unsupported architecture: $ARCH"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Download URL
URL="https://github.com/$REPO/releases/$VERSION/download/$BINARY"

echo "📦 Downloading from: $URL"

# Download binary
if command -v curl &> /dev/null; then
    curl -L "$URL" -o media-cli
elif command -v wget &> /dev/null; then
    wget "$URL" -O media-cli
else
    echo "❌ Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Make executable
chmod +x media-cli

# Install
if [ -w "$INSTALL_DIR" ]; then
    mv media-cli "$INSTALL_DIR/"
    echo "✅ Installed to $INSTALL_DIR/media-cli"
else
    echo "🔐 Need sudo to install to $INSTALL_DIR"
    sudo mv media-cli "$INSTALL_DIR/"
    echo "✅ Installed to $INSTALL_DIR/media-cli"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "📋 Next steps:"
echo "1. Install dependencies:"
echo "   - Ubuntu/Debian: sudo apt install mpv yt-dlp streamlink fzf curl"
echo "   - Arch: sudo pacman -S mpv yt-dlp streamlink fzf curl"
echo "   - macOS: brew install mpv yt-dlp streamlink fzf curl"
echo ""
echo "2. Run media-cli:"
echo "   media-cli"
echo ""
echo "3. For help:"
echo "   media-cli --help"
echo ""
