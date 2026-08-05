#!/usr/bin/env bash
# ============================================================================
# Nux Programming Language - BSD Installer
# Interactive installer with Install/Repair/Update/Uninstall menu
# Supports FreeBSD, OpenBSD, NetBSD
# ============================================================================
set -e

VERSION="1.0.0"
PRODUCT="Nux Programming Language"
INSTALL_DIR="/usr/local/lib/nux"
BIN_LINK="/usr/local/bin/nux"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-bsd.tar.gz"
MARKER_FILE="$INSTALL_DIR/.nux_installed"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

banner() {
    echo ""
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║                                                          ║${NC}"
    echo -e "${CYAN}║   ${BOLD}${BLUE}Nux Programming Language${NC}${CYAN}                                ║${NC}"
    echo -e "${CYAN}║   ${NC}Version ${VERSION} — BSD Installer${CYAN}                         ║${NC}"
    echo -e "${CYAN}║                                                          ║${NC}"
    echo -e "${CYAN}║   Write Once, Run Anywhere.                              ║${NC}"
    echo -e "${CYAN}║                                                          ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        echo -e "${YELLOW}This installer requires root privileges.${NC}"
        echo "Please re-run with: sudo $0"
        exit 1
    fi
}

is_installed() {
    [ -f "$MARKER_FILE" ] && return 0 || return 1
}

detect_bsd() {
    uname -s 2>/dev/null || echo "Unknown"
}

show_menu() {
    BSD_TYPE=$(detect_bsd)
    echo -e "${BOLD}Detected OS:${NC} $BSD_TYPE"
    echo ""
    echo -e "${BOLD}Select an option:${NC}"
    echo ""
    echo -e "  ${GREEN}1)${NC}  Install Now         ${CYAN}(Recommended)${NC}"
    echo -e "  ${GREEN}2)${NC}  Custom Install       Choose location and features"
    if is_installed; then
        echo -e "  ${GREEN}3)${NC}  Repair              Reinstall all files"
        echo -e "  ${GREEN}4)${NC}  Update              Fetch latest version"
        echo -e "  ${RED}5)${NC}  Uninstall           Remove Nux completely"
    fi
    echo -e "  ${GREEN}6)${NC}  Build FreeBSD pkg   Create native package"
    echo -e "  ${GREEN}0)${NC}  Exit"
    echo ""
    read -rp "Enter choice [1]: " choice
    choice=${choice:-1}
}

do_install() {
    local target_dir="${1:-$INSTALL_DIR}"
    local install_stdlib="${2:-yes}"
    local install_path="${3:-yes}"

    echo ""
    echo -e "${BLUE}[1/4]${NC} Creating installation directory..."
    mkdir -p "$target_dir"

    echo -e "${BLUE}[2/4]${NC} Downloading Nux binaries..."
    TEMP_TAR=$(mktemp)
    if curl -fSL -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null || fetch -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null; then
        echo -e "${BLUE}[3/4]${NC} Extracting files..."
        tar -xzf "$TEMP_TAR" -C "$target_dir" 2>/dev/null || true
    else
        echo -e "${YELLOW}       Download unavailable. Using local payload if present.${NC}"
        SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
        if [ -d "$SCRIPT_DIR/payload" ]; then
            cp -r "$SCRIPT_DIR/payload/"* "$target_dir/"
        fi
    fi
    rm -f "$TEMP_TAR"

    if [ "$install_stdlib" = "yes" ]; then
        mkdir -p "$target_dir/lib"
        echo "       Standard library installed."
    fi

    echo -e "${BLUE}[4/4]${NC} Setting up PATH and symlinks..."
    if [ "$install_path" = "yes" ]; then
        mkdir -p "$(dirname "$BIN_LINK")"
        ln -sf "$target_dir/nux" "$BIN_LINK"
        chmod +x "$BIN_LINK" 2>/dev/null || true
        chmod +x "$target_dir/nux" 2>/dev/null || true

        # Add to shell profiles
        for profile in "$HOME/.profile" "$HOME/.shrc" "$HOME/.bashrc"; do
            if [ -f "$profile" ]; then
                if ! grep -q "NUX_HOME" "$profile" 2>/dev/null; then
                    echo "" >> "$profile"
                    echo "# Nux Programming Language" >> "$profile"
                    echo "export NUX_HOME=\"$target_dir\"" >> "$profile"
                    echo "export NUX_LIB_PATH=\"$target_dir/lib\"" >> "$profile"
                fi
            fi
        done
    fi

    echo "$VERSION" > "$target_dir/.nux_installed"

    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  Installation Successful!                                ║${NC}"
    echo -e "${GREEN}║                                                          ║${NC}"
    echo -e "${GREEN}║  You can now use the 'nux' command from any terminal.    ║${NC}"
    echo -e "${GREEN}║  Try: nux run hello.nux                                  ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

do_custom_install() {
    echo ""
    read -rp "Installation directory [$INSTALL_DIR]: " custom_dir
    custom_dir=${custom_dir:-$INSTALL_DIR}

    read -rp "Install Standard Library? [Y/n]: " install_std
    install_std=${install_std:-Y}
    [ "$install_std" = "Y" ] || [ "$install_std" = "y" ] && install_std="yes" || install_std="no"

    read -rp "Add to PATH? [Y/n]: " install_path
    install_path=${install_path:-Y}
    [ "$install_path" = "Y" ] || [ "$install_path" = "y" ] && install_path="yes" || install_path="no"

    do_install "$custom_dir" "$install_std" "$install_path"
}

do_repair() {
    echo -e "${BLUE}Repairing Nux installation...${NC}"
    do_install
}

do_update() {
    echo -e "${BLUE}Updating Nux to latest version...${NC}"
    TEMP_TAR=$(mktemp)
    if curl -fSL -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null || fetch -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null; then
        tar -xzf "$TEMP_TAR" -C "$INSTALL_DIR"
        echo "$VERSION" > "$MARKER_FILE"
        echo -e "${GREEN}Update successful!${NC}"
    else
        echo -e "${RED}Failed to download update.${NC}"
    fi
    rm -f "$TEMP_TAR"
}

do_uninstall() {
    echo ""
    echo -e "${RED}WARNING: This will completely remove Nux from your system.${NC}"
    read -rp "Are you sure? [y/N]: " confirm
    if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
        echo -e "${BLUE}Removing Nux...${NC}"
        rm -rf "$INSTALL_DIR"
        rm -f "$BIN_LINK"

        for profile in "$HOME/.profile" "$HOME/.shrc" "$HOME/.bashrc"; do
            if [ -f "$profile" ]; then
                sed -i.bak '/NUX_HOME/d' "$profile" 2>/dev/null || true
                sed -i.bak '/NUX_LIB_PATH/d' "$profile" 2>/dev/null || true
                sed -i.bak '/# Nux Programming Language/d' "$profile" 2>/dev/null || true
                rm -f "${profile}.bak"
            fi
        done

        echo -e "${GREEN}Nux has been completely removed.${NC}"
    else
        echo "Uninstall cancelled."
    fi
}

do_build_freebsd_pkg() {
    echo -e "${BLUE}Building FreeBSD package...${NC}"

    rm -rf pkg_stage
    mkdir -p pkg_stage/usr/local/lib/nux/lib
    mkdir -p pkg_stage/usr/local/bin

    # Copy payload
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    if [ -d "$SCRIPT_DIR/payload" ]; then
        cp -r "$SCRIPT_DIR/payload/"* "pkg_stage/usr/local/lib/nux/"
    fi

    cat <<EOF > pkg_stage/+MANIFEST
name: "nux"
version: "$VERSION"
origin: "lang/nux"
prefix: "/usr/local"
comment: "Nux Programming Language"
desc: "An incredibly fast, lightweight, Write-Once-Run-Anywhere programming language with built-in support for ML, quantum computing, 3D engines, and more."
maintainer: "nux@nuxlang.org"
www: "https://github.com/DoguparthiAakash/Nux_Lang"
categories: ["lang"]
EOF

    cat <<'POSTINST' > pkg_stage/+POST_INSTALL
#!/bin/sh
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod +x /usr/local/bin/nux
POSTINST

    cat <<'PRERM' > pkg_stage/+PRE_DEINSTALL
#!/bin/sh
rm -f /usr/local/bin/nux
PRERM

    if command -v pkg &>/dev/null; then
        pkg create -M pkg_stage/+MANIFEST -r pkg_stage -o .
        echo -e "${GREEN}FreeBSD package created.${NC}"
        echo "Install with: pkg install ./nux-${VERSION}.pkg"
    else
        echo -e "${RED}pkg command not found. Run this on FreeBSD.${NC}"
    fi
    rm -rf pkg_stage
}

# ============================================================================
# Main
# ============================================================================
banner
check_root
show_menu

case "$choice" in
    1) do_install ;;
    2) do_custom_install ;;
    3) if is_installed; then do_repair; else echo "Not installed."; fi ;;
    4) if is_installed; then do_update; else echo "Not installed."; fi ;;
    5) if is_installed; then do_uninstall; else echo "Not installed."; fi ;;
    6) do_build_freebsd_pkg ;;
    0) echo "Exiting."; exit 0 ;;
    *) echo "Invalid choice."; exit 1 ;;
esac
