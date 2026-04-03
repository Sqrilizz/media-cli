#!/bin/bash

set -e

REPO="sqrilizz/media-cli"  # Replace with your GitHub username
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing media-cli..."

# Download binary directly from repo
URL="https://raw.githubusercontent.com/$REPO/main/media-cli"

echo "📦 Downloading from: $URL"

# Download
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
