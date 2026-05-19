#!/bin/bash
set -e

REPO="sqrilizz/media-cli"
VERSION="latest"
INSTALL_DIR="/usr/local/bin"
DEPS=(mpv yt-dlp streamlink fzf curl)

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
PURPLE='\033[38;2;100;43;115m'
PINK='\033[38;2;198;66;110m'
DIM='\033[0;90m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "  ${CYAN}::${NC} $1"; }
ok()    { echo -e "  ${GREEN}✓${NC} $1"; }
warn()  { echo -e "  ${YELLOW}⚠${NC} $1"; }
err()   { echo -e "  ${RED}✕${NC} $1"; }

# --- Header ---
echo ""
echo -e "  ${PURPLE}███╗   ███╗███████╗██████╗ ██╗ █████╗      ██████╗██╗     ██╗${NC}"
echo -e "  ${PINK}╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝╚═╝  ╚═╝     ╚═════╝╚══════╝╚═╝${NC}"
echo ""
echo -e "  ${BOLD}Installer${NC}  ${DIM}──────────────────────────────────────${NC}"
echo ""

# --- Detect OS ---
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
DISTRO=""
PKG_MANAGER=""

detect_distro() {
    if [ "$OS" = "darwin" ]; then
        DISTRO="macos"
        PKG_MANAGER="brew"
        return
    fi

    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu|debian|linuxmint|pop|elementary|zorin|kali)
                DISTRO="debian"
                PKG_MANAGER="apt"
                ;;
            arch|manjaro|endeavouros|garuda|artix)
                DISTRO="arch"
                PKG_MANAGER="pacman"
                ;;
            fedora|rhel|centos|rocky|alma|nobara)
                DISTRO="fedora"
                PKG_MANAGER="dnf"
                ;;
            opensuse*|sles)
                DISTRO="suse"
                PKG_MANAGER="zypper"
                ;;
            void)
                DISTRO="void"
                PKG_MANAGER="xbps-install"
                ;;
            gentoo)
                DISTRO="gentoo"
                PKG_MANAGER="emerge"
                ;;
            nixos)
                DISTRO="nixos"
                PKG_MANAGER="nix-env"
                ;;
            *)
                DISTRO="unknown"
                ;;
        esac
    elif [ -f /etc/arch-release ]; then
        DISTRO="arch"
        PKG_MANAGER="pacman"
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
        PKG_MANAGER="apt"
    fi
}

detect_distro
info "OS: ${BOLD}${OS}${NC} (${ARCH})  Distro: ${BOLD}${DISTRO:-unknown}${NC}"
echo ""

# --- Install dependencies ---
install_deps() {
    local missing=()

    for dep in "${DEPS[@]}"; do
        if command -v "$dep" &>/dev/null; then
            ok "${dep} ${DIM}found${NC}"
        else
            warn "${dep} ${DIM}not found${NC}"
            missing+=("$dep")
        fi
    done

    if [ ${#missing[@]} -eq 0 ]; then
        echo ""
        ok "All dependencies installed"
        return 0
    fi

    echo ""
    info "Installing missing: ${BOLD}${missing[*]}${NC}"
    echo ""

    case "$PKG_MANAGER" in
        apt)
            sudo apt update -qq
            # yt-dlp may not be in apt on older versions
            local apt_pkgs=()
            local pip_pkgs=()
            for pkg in "${missing[@]}"; do
                if [ "$pkg" = "yt-dlp" ]; then
                    if apt-cache show yt-dlp &>/dev/null 2>&1; then
                        apt_pkgs+=("$pkg")
                    else
                        pip_pkgs+=("$pkg")
                    fi
                elif [ "$pkg" = "streamlink" ]; then
                    if apt-cache show streamlink &>/dev/null 2>&1; then
                        apt_pkgs+=("$pkg")
                    else
                        pip_pkgs+=("$pkg")
                    fi
                else
                    apt_pkgs+=("$pkg")
                fi
            done
            [ ${#apt_pkgs[@]} -gt 0 ] && sudo apt install -y "${apt_pkgs[@]}"
            if [ ${#pip_pkgs[@]} -gt 0 ]; then
                if ! command -v pip3 &>/dev/null && ! command -v pipx &>/dev/null; then
                    sudo apt install -y python3-pip
                fi
                for pkg in "${pip_pkgs[@]}"; do
                    if command -v pipx &>/dev/null; then
                        pipx install "$pkg" 2>/dev/null || pip3 install --user "$pkg"
                    else
                        pip3 install --user "$pkg"
                    fi
                done
            fi
            ;;
        pacman)
            sudo pacman -Sy --noconfirm
            sudo pacman -S --needed --noconfirm "${missing[@]}"
            ;;
        dnf)
            sudo dnf install -y "${missing[@]}"
            ;;
        zypper)
            sudo zypper install -y "${missing[@]}"
            ;;
        xbps-install)
            sudo xbps-install -y "${missing[@]}"
            ;;
        brew)
            brew install "${missing[@]}"
            ;;
        *)
            err "Unknown package manager. Install manually: ${missing[*]}"
            return 1
            ;;
    esac

    echo ""
    # Verify
    local still_missing=()
    for dep in "${missing[@]}"; do
        if command -v "$dep" &>/dev/null; then
            ok "${dep} ${DIM}installed${NC}"
        else
            still_missing+=("$dep")
        fi
    done

    if [ ${#still_missing[@]} -gt 0 ]; then
        warn "Could not install: ${still_missing[*]}"
        warn "Install them manually before using media-cli"
    fi
}

info "Checking dependencies..."
echo ""
install_deps

# --- Download binary ---
echo ""
case "$OS" in
    linux)
        case "$ARCH" in
            x86_64)  BINARY="media-cli-linux-x86_64" ;;
            aarch64) BINARY="media-cli-linux-arm64" ;;
            *)       err "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64)  BINARY="media-cli-macos-x86_64" ;;
            arm64)   BINARY="media-cli-macos-arm64" ;;
            *)       err "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        err "Unsupported OS: $OS"
        exit 1
        ;;
esac

URL="https://github.com/$REPO/releases/$VERSION/download/$BINARY"
info "Downloading media-cli..."

TMP=$(mktemp)
if command -v curl &>/dev/null; then
    curl -fSL "$URL" -o "$TMP" 2>/dev/null || {
        # Fallback: try direct from repo
        info "Trying direct download..."
        curl -fSL "https://raw.githubusercontent.com/$REPO/main/media-cli" -o "$TMP"
    }
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "$TMP" 2>/dev/null || {
        info "Trying direct download..."
        wget -q "https://raw.githubusercontent.com/$REPO/main/media-cli" -O "$TMP"
    }
else
    err "Neither curl nor wget found"
    exit 1
fi

chmod +x "$TMP"

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP" "$INSTALL_DIR/media-cli"
else
    sudo mv "$TMP" "$INSTALL_DIR/media-cli"
fi

ok "Installed to ${BOLD}${INSTALL_DIR}/media-cli${NC}"

# --- Done ---
echo ""
echo -e "  ${GREEN}${BOLD}Installation complete!${NC}"
echo ""
echo -e "  ${DIM}Run:${NC}  ${BOLD}media-cli${NC}"
echo -e "  ${DIM}Help:${NC} ${BOLD}media-cli --help${NC}"
echo ""
