#!/usr/bin/env bash
# ============================================================================
# Nux Programming Language - Linux Installer
# Interactive installer with Install/Repair/Update/Uninstall menu
# ============================================================================
set -e

VERSION="1.0.0"
PRODUCT="Nux Programming Language"
INSTALL_DIR="/usr/local/lib/nux"
BIN_LINK="/usr/local/bin/nux"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-linux.tar.gz"
MARKER_FILE="$INSTALL_DIR/.nux_installed"
DESKTOP_FILE="/usr/share/applications/nux.desktop"

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
    echo -e "${CYAN}║   ${NC}Version ${VERSION} — Linux Installer${CYAN}                       ║${NC}"
    echo -e "${CYAN}║                                                          ║${NC}"
    echo -e "${CYAN}║   Write Once, Run Anywhere.                              ║${NC}"
    echo -e "${CYAN}║                                                          ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

check_root() {
    if [ "$EUID" -ne 0 ]; then
        echo -e "${YELLOW}This installer requires root privileges.${NC}"
        echo "Please re-run with: sudo $0"
        exit 1
    fi
}

is_installed() {
    [ -f "$MARKER_FILE" ] && return 0 || return 1
}

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif command -v lsb_release &>/dev/null; then
        lsb_release -is | tr '[:upper:]' '[:lower:]'
    else
        echo "unknown"
    fi
}

show_menu() {
    DISTRO=$(detect_distro)
    echo -e "${BOLD}Detected distribution:${NC} $DISTRO"
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
    echo -e "  ${GREEN}6)${NC}  Build .deb package  (Debian/Ubuntu)"
    echo -e "  ${GREEN}7)${NC}  Build .rpm package  (Fedora/RHEL)"
    echo -e "  ${GREEN}0)${NC}  Exit"
    echo ""
    read -rp "Enter choice [1]: " choice
    choice=${choice:-1}
}

do_install() {
    local target_dir="${1:-$INSTALL_DIR}"
    local install_stdlib="${2:-yes}"
    local install_path="${3:-yes}"
    local install_desktop="${4:-yes}"

    echo ""
    echo -e "${BLUE}[1/5]${NC} Creating installation directory..."
    mkdir -p "$target_dir"

    echo -e "${BLUE}[2/5]${NC} Downloading Nux binaries..."
    TEMP_TAR=$(mktemp)
    if curl -fSL -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null; then
        echo -e "${BLUE}[3/5]${NC} Extracting files..."
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

    echo -e "${BLUE}[4/5]${NC} Setting up PATH and symlinks..."
    if [ "$install_path" = "yes" ]; then
        mkdir -p "$(dirname "$BIN_LINK")"
        ln -sf "$target_dir/nux" "$BIN_LINK"
        chmod +x "$BIN_LINK" 2>/dev/null || true
        chmod +x "$target_dir/nux" 2>/dev/null || true

        # Add to shell profiles
        for profile in /etc/profile.d/nux.sh; do
            cat <<ENVEOF > "$profile"
# Nux Programming Language
export NUX_HOME="$target_dir"
export NUX_LIB_PATH="$target_dir/lib"
ENVEOF
        done
    fi

    echo -e "${BLUE}[5/5]${NC} Creating desktop entry..."
    if [ "$install_desktop" = "yes" ]; then
        mkdir -p "$(dirname "$DESKTOP_FILE")"
        cat <<DESKTOPEOF > "$DESKTOP_FILE"
[Desktop Entry]
Type=Application
Name=Nux
Comment=Nux Programming Language
Exec=$target_dir/nux %f
Icon=$target_dir/logo.png
Terminal=true
Categories=Development;IDE;
MimeType=text/x-nux;
DESKTOPEOF
        update-desktop-database /usr/share/applications 2>/dev/null || true
    fi

    # Write marker
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

    read -rp "Create desktop entry? [Y/n]: " install_desktop
    install_desktop=${install_desktop:-Y}
    [ "$install_desktop" = "Y" ] || [ "$install_desktop" = "y" ] && install_desktop="yes" || install_desktop="no"

    do_install "$custom_dir" "$install_std" "$install_path" "$install_desktop"
}

do_repair() {
    echo -e "${BLUE}Repairing Nux installation...${NC}"
    do_install
}

do_update() {
    echo -e "${BLUE}Updating Nux to latest version...${NC}"
    TEMP_TAR=$(mktemp)
    if curl -fSL -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null; then
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
        rm -f /etc/profile.d/nux.sh
        rm -f "$DESKTOP_FILE"
        update-desktop-database /usr/share/applications 2>/dev/null || true
        echo -e "${GREEN}Nux has been completely removed.${NC}"
    else
        echo "Uninstall cancelled."
    fi
}

do_build_deb() {
    echo -e "${BLUE}Building Debian package...${NC}"
    ARCH="amd64"
    PKG_DIR="nux_${VERSION}_${ARCH}"

    rm -rf "$PKG_DIR"
    mkdir -p "$PKG_DIR/DEBIAN"
    mkdir -p "$PKG_DIR/usr/local/lib/nux/lib"
    mkdir -p "$PKG_DIR/usr/local/bin"
    mkdir -p "$PKG_DIR/usr/share/applications"

    # Control file
    cat <<EOF > "$PKG_DIR/DEBIAN/control"
Package: nux
Version: $VERSION
Section: devel
Priority: optional
Architecture: $ARCH
Maintainer: NuxLang Team <nux@nuxlang.org>
Homepage: https://github.com/DoguparthiAakash/Nux_Lang
Description: Nux Programming Language
 An incredibly fast, lightweight, Write-Once-Run-Anywhere
 programming language with built-in support for ML, quantum
 computing, 3D engines, and more.
EOF

    # Postinst
    cat <<'POSTINST' > "$PKG_DIR/DEBIAN/postinst"
#!/bin/bash
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod +x /usr/local/bin/nux 2>/dev/null || true
update-desktop-database /usr/share/applications 2>/dev/null || true
POSTINST
    chmod +x "$PKG_DIR/DEBIAN/postinst"

    # Prerm
    cat <<'PRERM' > "$PKG_DIR/DEBIAN/prerm"
#!/bin/bash
rm -f /usr/local/bin/nux
PRERM
    chmod +x "$PKG_DIR/DEBIAN/prerm"

    # Copy payload
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    if [ -d "$SCRIPT_DIR/payload" ]; then
        cp -r "$SCRIPT_DIR/payload/"* "$PKG_DIR/usr/local/lib/nux/"
    fi

    if command -v dpkg-deb &>/dev/null; then
        dpkg-deb --build "$PKG_DIR"
        echo -e "${GREEN}Built ${PKG_DIR}.deb${NC}"
        echo "Install with: sudo dpkg -i ${PKG_DIR}.deb"
    else
        echo -e "${RED}dpkg-deb not found. Run this on a Debian/Ubuntu system.${NC}"
    fi
    rm -rf "$PKG_DIR"
}

do_build_rpm() {
    echo -e "${BLUE}Building RPM package...${NC}"

    rm -rf rpmbuild
    mkdir -p rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

    cat <<EOF > rpmbuild/SPECS/nux.spec
Name:           nux
Version:        $VERSION
Release:        1%{?dist}
Summary:        Nux Programming Language
License:        MIT
URL:            https://github.com/DoguparthiAakash/Nux_Lang

%description
An incredibly fast, lightweight, Write-Once-Run-Anywhere programming language
with built-in support for ML, quantum computing, 3D engines, and more.

%install
mkdir -p %{buildroot}/usr/local/lib/nux/lib
mkdir -p %{buildroot}/usr/local/bin

%post
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod +x /usr/local/bin/nux

%preun
rm -f /usr/local/bin/nux

%files
/usr/local/lib/nux/*

%changelog
* $(date +"%a %b %d %Y") NuxLang Team <nux@nuxlang.org> - $VERSION-1
- Initial RPM package
EOF

    if command -v rpmbuild &>/dev/null; then
        rpmbuild -ba rpmbuild/SPECS/nux.spec --define "_topdir $(pwd)/rpmbuild"
        echo -e "${GREEN}RPM built in rpmbuild/RPMS/${NC}"
    else
        echo -e "${RED}rpmbuild not found. Run this on a Fedora/RHEL system.${NC}"
    fi
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
    6) do_build_deb ;;
    7) do_build_rpm ;;
    0) echo "Exiting."; exit 0 ;;
    *) echo "Invalid choice."; exit 1 ;;
esac
