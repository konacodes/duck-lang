#!/bin/bash
#
# Duck Language Installer
# The goose permits this installation. Barely.
#

set -e

REPO="konacodes/duck-lang"
INSTALL_DIR="${DUCK_INSTALL_DIR:-$HOME/.duck}"
BIN_DIR="$INSTALL_DIR/bin"
VERSION_FILE="$INSTALL_DIR/.version"

# Colors (the goose prefers monochrome but we compromised)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# -----------------------------------------------------------------------------
# ASCII Art
# -----------------------------------------------------------------------------

show_goose() {
    echo ""
    echo "                                  ___"
    echo "                               .-'   \`'."
    echo "                              /         \\"
    echo "                              |         ;"
    echo "                              |         |           ___.--,"
    echo "                             |          |_.---._ .-'       \`,"
    echo "                             /:        ./       ,'          ;"
    echo "                             \\':      :(        |           /"
    echo "                              \\':     :';       ;          /"
    echo "                               \\ \\    / ;      /    ____.--\\"
    echo "                                \`.\`._.' /    .'  .-\"        |"
    echo "                                  \`-...-\`   /  .-'          /"
    echo "                                         .'  (            /"
    echo "                                        /     \`-.       .'"
    echo "                                       /         \`----'\`"
    echo "                                      (                  "
    echo "                                       \`.               /"
    echo "                                         \`-._________.-'"
    echo ""
}

show_installing() {
    echo -e "${CYAN}"
    echo "    ____             __      __                     "
    echo "   / __ \\__  _______/ /__   / /   ____ _____  ____ _"
    echo "  / / / / / / / ___/ //_/  / /   / __ \`/ __ \\/ __ \`/"
    echo " / /_/ / /_/ / /__/ ,<    / /___/ /_/ / / / / /_/ / "
    echo "/_____/\\__,_/\\___/_/|_|  /_____/\\__,_/_/ /_/\\__, /  "
    echo "                                          /____/   "
    echo -e "${NC}"
}

show_success() {
    echo ""
    echo -e "${GREEN}"
    echo "    ___ _   _  ___ ___ ___  ___ ___ "
    echo "   / __| | | |/ __/ __/ _ \\/ __/ __|"
    echo "   \\__ \\ |_| | (_| (_|  __/\\__ \\__ \\"
    echo "   |___/\\__,_|\\___\\___\\___||___/___/"
    echo -e "${NC}"
    echo ""
}

# -----------------------------------------------------------------------------
# Utilities
# -----------------------------------------------------------------------------

info() {
    echo -e "${CYAN}[*]${NC} $1"
}

success() {
    echo -e "${GREEN}[+]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[!]${NC} $1"
}

error() {
    echo -e "${RED}[x]${NC} $1"
    exit 1
}

spinner() {
    local pid=$1
    local delay=0.1
    local frames='|/-\'
    while ps -p $pid > /dev/null 2>&1; do
        for (( i=0; i<${#frames}; i++ )); do
            printf "\r${CYAN}[%c]${NC} $2" "${frames:$i:1}"
            sleep $delay
        done
    done
    printf "\r"
}

progress_bar() {
    local current=$1
    local total=$2
    local width=40
    local percent=$((current * 100 / total))
    local filled=$((current * width / total))
    local empty=$((width - filled))

    printf "\r["
    printf "%${filled}s" | tr ' ' '#'
    printf "%${empty}s" | tr ' ' '-'
    printf "] %3d%%" $percent
}

# -----------------------------------------------------------------------------
# Detection
# -----------------------------------------------------------------------------

detect_os() {
    local os
    case "$(uname -s)" in
        Linux*)     os="linux";;
        Darwin*)    os="macos";;
        CYGWIN*|MINGW*|MSYS*) os="windows";;
        *)          error "Unsupported operating system: $(uname -s)";;
    esac
    echo "$os"
}

detect_arch() {
    local arch
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64";;
        aarch64|arm64)  arch="aarch64";;
        *)              error "Unsupported architecture: $(uname -m)";;
    esac
    echo "$arch"
}

# -----------------------------------------------------------------------------
# Installation
# -----------------------------------------------------------------------------

get_latest_version() {
    curl -sL "https://api.github.com/repos/$REPO/releases/latest" | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/'
}

get_download_url() {
    local version=$1
    local os=$2
    local arch=$3

    local filename="goose-${version}-${os}-${arch}"
    if [ "$os" = "windows" ]; then
        filename="${filename}.exe"
    fi

    echo "https://github.com/$REPO/releases/download/${version}/${filename}"
}

download_binary() {
    local url=$1
    local dest=$2

    if command -v curl > /dev/null 2>&1; then
        curl -sL "$url" -o "$dest"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$url" -O "$dest"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

install_goose() {
    local version=$1

    show_installing
    show_goose

    echo -e "${DIM}The goose has arrived. Installation will proceed.${NC}"
    echo ""

    # Detect system
    info "Detecting system..."
    local os=$(detect_os)
    local arch=$(detect_arch)
    success "Detected: $os ($arch)"

    # Get version
    if [ -z "$version" ]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
        if [ -z "$version" ]; then
            error "Failed to fetch latest version. Check your internet connection."
        fi
    fi
    success "Version: $version"

    # Create directories
    info "Creating directories..."
    mkdir -p "$BIN_DIR"
    success "Install directory: $INSTALL_DIR"

    # Download
    local url=$(get_download_url "$version" "$os" "$arch")
    local tmp_file=$(mktemp)

    info "Downloading goose..."
    echo -e "${DIM}$url${NC}"

    download_binary "$url" "$tmp_file" &
    local download_pid=$!

    # Show spinner while downloading
    local dots=""
    while ps -p $download_pid > /dev/null 2>&1; do
        for i in 1 2 3 4 5; do
            if ! ps -p $download_pid > /dev/null 2>&1; then
                break
            fi
            dots="${dots}."
            if [ ${#dots} -gt 5 ]; then
                dots="."
            fi
            printf "\r${CYAN}[*]${NC} Downloading${dots}     "
            sleep 0.3
        done
    done
    printf "\r"

    wait $download_pid || error "Download failed. The goose is displeased."

    # Verify download
    if [ ! -s "$tmp_file" ]; then
        rm -f "$tmp_file"
        error "Downloaded file is empty. Release may not exist for your platform."
    fi

    success "Download complete"

    # Install binary
    info "Installing binary..."
    mv "$tmp_file" "$BIN_DIR/goose"
    chmod +x "$BIN_DIR/goose"
    success "Installed to $BIN_DIR/goose"

    # Save version
    echo "$version" > "$VERSION_FILE"

    # Setup PATH
    setup_path

    show_success

    echo -e "${BOLD}The goose has been installed.${NC}"
    echo ""
    echo "  Version:  $version"
    echo "  Location: $BIN_DIR/goose"
    echo ""

    if ! echo "$PATH" | grep -q "$BIN_DIR"; then
        echo -e "${YELLOW}To complete installation, add this to your shell profile:${NC}"
        echo ""
        echo "  export PATH=\"\$PATH:$BIN_DIR\""
        echo ""
        echo "Then restart your terminal or run:"
        echo ""
        echo "  source ~/.bashrc  # or ~/.zshrc"
        echo ""
    fi

    echo "Run 'goose --help' to get started."
    echo ""
    echo -e "${DIM}\"I permit this binary to exist on your system. For now.\" - The Goose${NC}"
    echo ""
}

setup_path() {
    local shell_rc=""
    local shell_name=$(basename "$SHELL")

    case "$shell_name" in
        bash)
            if [ -f "$HOME/.bashrc" ]; then
                shell_rc="$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                shell_rc="$HOME/.bash_profile"
            fi
            ;;
        zsh)
            shell_rc="$HOME/.zshrc"
            ;;
        fish)
            # Fish uses a different syntax
            return
            ;;
    esac

    if [ -n "$shell_rc" ] && [ -f "$shell_rc" ]; then
        if ! grep -q "$BIN_DIR" "$shell_rc" 2>/dev/null; then
            echo "" >> "$shell_rc"
            echo "# Duck Language (goose interpreter)" >> "$shell_rc"
            echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
            success "Added $BIN_DIR to PATH in $shell_rc"
        fi
    fi
}

# -----------------------------------------------------------------------------
# Main
# -----------------------------------------------------------------------------

main() {
    local version=""

    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            -v|--version)
                version="$2"
                shift 2
                ;;
            -h|--help)
                echo "Duck Language Installer"
                echo ""
                echo "Usage: install.sh [options]"
                echo ""
                echo "Options:"
                echo "  -v, --version VERSION    Install specific version"
                echo "  -h, --help               Show this help"
                echo ""
                echo "Environment variables:"
                echo "  DUCK_INSTALL_DIR         Installation directory (default: ~/.duck)"
                echo ""
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done

    install_goose "$version"
}

main "$@"
