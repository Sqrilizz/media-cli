#!/bin/bash
set -e

FULL_INSTALLER="https://raw.githubusercontent.com/sqrilizz/media-cli/main/scripts/install.sh"

echo "Redirecting to the full installer so all runtime dependencies are installed..."

if command -v curl &>/dev/null; then
    exec bash <(curl -fsSL "$FULL_INSTALLER")
elif command -v wget &>/dev/null; then
    exec bash <(wget -qO- "$FULL_INSTALLER")
else
    echo "❌ curl or wget is required to start the installer."
    exit 1
fi
