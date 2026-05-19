#!/bin/bash
# Quick install - binary only (no dependency installation)
# For full install with dependencies, use install.sh instead:
#   curl -fsSL https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh | bash

set -e

REPO="sqrilizz/media-cli"
INSTALL_DIR="/usr/local/bin"

echo -e "\033[1m🚀 Installing media-cli (binary only)...\033[0m"
echo ""

URL="https://raw.githubusercontent.com/$REPO/main/media-cli"

TMP=$(mktemp)
if command -v curl &>/dev/null; then
    curl -fSL "$URL" -o "$TMP"
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "$TMP"
else
    echo "❌ Neither curl nor wget found."
    exit 1
fi

chmod +x "$TMP"

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP" "$INSTALL_DIR/media-cli"
else
    sudo mv "$TMP" "$INSTALL_DIR/media-cli"
fi

echo "✅ Installed to $INSTALL_DIR/media-cli"
echo ""
echo "⚠  Dependencies not installed. Run full installer for auto-setup:"
echo "   curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/install.sh | bash"
echo ""
