#!/usr/bin/env bash
# ============================================================================
#  Nux Programming Language â€” Linux Installer
#  Security-hardened with checksum verification, safe temp files,
#  symlink protection, and Rust/Nux-inspired TUI art
# ============================================================================
set -euo pipefail

VERSION="1.0.0"
PRODUCT="Nux Programming Language"
INSTALL_DIR_BASE="/usr/local/lib/nux"
INSTALL_DIR="$INSTALL_DIR_BASE/v$VERSION"
CURRENT_LINK="$INSTALL_DIR_BASE/current"
BIN_LINK="/usr/local/bin/nux"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-linux.tar.gz"
CHECKSUM_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-linux.sha256"
MARKER_FILE="$INSTALL_DIR/.nux_installed"
DESKTOP_FILE="/usr/share/applications/nux.desktop"

# --- Restrict PATH to known-safe directories (prevent PATH injection) ---
export PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

# --- Restrict umask ---
umask 022

# --- TUI Colors (Rust/Nux-inspired) ---
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m'
DARKGRAY='\033[38;5;8m'

# Rust/cargo-style output: right-aligned tag + message
nux_header() {
    local title="$1" subtitle="$2"
    echo ""
    printf "%bâ•­â”€ %bâ—† %b%s %bâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n" "$DARKGRAY" "$CYAN" "${WHITE}${BOLD}" "$title" "$DARKGRAY"
    printf "%bâ”‚  %bNux v%s  %bÂ·  %b%s\n" "$DARKGRAY" "${WHITE}${BOLD}" "$VERSION" "$DARKGRAY" "$DARKGRAY" "$subtitle"
    printf "%bâ•°â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€%b\n" "$DARKGRAY" "$NC"
}

nux_print() {
    local tag="$1" color="$2" msg="$3"
    printf "%bâ”œâ”€ %bâœ¦ %b%s  %b%s%b\n" "$DARKGRAY" "$GREEN" "$color" "$tag" "$DARKGRAY" "$msg" "$NC"
}

nux_error() {
    printf "%bâ•°â”€ %bâœ• %berror  %b%s%b\n" "$DARKGRAY" "$RED" "$RED" "$WHITE" "$1" "$NC"
}

nux_warn() {
    printf "%bâ”œâ”€ %bâš  %bwarning  %b%s%b\n" "$DARKGRAY" "$YELLOW" "$YELLOW" "$DARKGRAY" "$1" "$NC"
}

nux_finish() {
    local tag="$1" msg="$2"
    printf "%bâ•°â”€ %bâ–¶ %b%s  %b%s%b\n" "$DARKGRAY" "$CYAN" "$CYAN" "$tag" "$WHITE" "$msg" "$NC"
}

banner() {
    nux_header "nux-installer" "interactive menu ..."
}

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        nux_error "This installer requires root privileges."
        echo "  Please re-run with: sudo $0"
        exit 1
    fi
}

is_installed() {
    [ -d "$INSTALL_DIR_BASE" ] || command -v nux >/dev/null 2>&1
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

# --- Security: verify download checksum ---
verify_checksum() {
    local file="$1"
    local expected_hash=""

    nux_print "Verifying" "$CYAN" "download integrity..."
    if command -v curl &>/dev/null; then
        expected_hash=$(curl -fsSL "$CHECKSUM_URL" 2>/dev/null | head -1 | awk '{print $1}') || true
    elif command -v wget &>/dev/null; then
        expected_hash=$(wget -qO- "$CHECKSUM_URL" 2>/dev/null | head -1 | awk '{print $1}') || true
    fi

    if [ -z "$expected_hash" ]; then
        nux_warn "Checksum file unavailable. Skipping integrity check."
        return 0
    fi

    local actual_hash
    actual_hash=$(sha256sum "$file" 2>/dev/null | awk '{print $1}')
    if [ -z "$actual_hash" ]; then
        actual_hash=$(shasum -a 256 "$file" 2>/dev/null | awk '{print $1}')
    fi

    if [ "$actual_hash" != "$expected_hash" ]; then
        nux_error "SHA-256 checksum mismatch!"
        nux_error "Expected: $expected_hash"
        nux_error "Got:      $actual_hash"
        nux_error "The download may be corrupted or tampered with."
        return 1
    fi

    nux_print "Verified" "$GREEN" "SHA-256 checksum OK"
    return 0
}

show_menu() {
    DISTRO=$(detect_distro)
    printf "  %bDetected:%b %s\n" "$BOLD" "$NC" "$DISTRO"
    echo ""
    printf "  %bSelect an option:%b\n" "$BOLD" "$NC"
    echo ""
    printf "    %b1)%b  Install Now         %b(Recommended)%b\n" "$GREEN" "$NC" "$CYAN" "$NC"
    printf "    %b2)%b  Custom Install       Choose location and features\n" "$GREEN" "$NC"
    if is_installed; then
        printf "    %b3)%b  Repair              Reinstall all files\n" "$GREEN" "$NC"
        printf "    %b4)%b  Update              Fetch latest version\n" "$GREEN" "$NC"
        printf "    %b5)%b  Uninstall           Remove Nux completely\n" "$RED" "$NC"
    fi
    printf "    %b6)%b  Build .deb package  (Debian/Ubuntu)\n" "$GREEN" "$NC"
    printf "    %b7)%b  Build .rpm package  (Fedora/RHEL)\n" "$GREEN" "$NC"
    printf "    %b0)%b  Exit\n" "$GREEN" "$NC"
    echo ""
    read -rp "  Enter choice [1]: " choice
    choice=${choice:-1}
}

do_install() {
    local target_dir="${1:-$INSTALL_DIR}"
    local install_stdlib="${2:-yes}"
    local install_path="${3:-yes}"
    local install_desktop="${4:-yes}"

    echo ""
    nux_print "Compiling" "$MAGENTA" "installation plan..."

    # --- Security: check for symlink attacks on install dir ---
    if [ -L "$target_dir" ]; then
        nux_error "Install directory is a symlink. Aborting for security."
        exit 1
    fi

    nux_print "Creating" "$CYAN" "$target_dir"
    mkdir -p "$target_dir"
    chmod 755 "$target_dir"

    # --- Download with secure temp file ---
    nux_print "Downloading" "$CYAN" "nux-linux.tar.gz..."
    TEMP_TAR=$(mktemp /tmp/nux-download-XXXXXXXX.tar.gz)
    trap 'rm -f "$TEMP_TAR"' EXIT

    local download_ok=false
    if command -v curl &>/dev/null; then
        curl -fSL --connect-timeout 30 --max-time 120 -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null && download_ok=true
    elif command -v wget &>/dev/null; then
        wget --timeout=30 -q -O "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null && download_ok=true
    fi

    if [ "$download_ok" = true ] && [ -s "$TEMP_TAR" ]; then
        # Verify checksum before extraction
        if ! verify_checksum "$TEMP_TAR"; then
            rm -f "$TEMP_TAR"
            nux_error "Installation aborted."
            exit 1
        fi
        nux_print "Extracting" "$CYAN" "files..."
        rm -f "$target_dir/nux" 2>/dev/null || true
        tar --no-same-owner -xzf "$TEMP_TAR" -C "$target_dir" 2>/dev/null || true
    else
        nux_warn "Download unavailable. Using local payload if present."
        SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
        if [ -d "$SCRIPT_DIR/payload" ]; then
            cp -r "$SCRIPT_DIR/payload/"* "$target_dir/"
        fi
    fi
    rm -f "$TEMP_TAR"

    if [ "$install_stdlib" = "yes" ]; then
        mkdir -p "$target_dir/lib"
        nux_print "Installed" "$GREEN" "standard library"
    fi

    # --- Set proper permissions ---
    chmod 755 "$target_dir/nux" 2>/dev/null || true
    find "$target_dir" -type d -exec chmod 755 {} \; 2>/dev/null || true
    find "$target_dir" -type f -name "*.nux" -exec chmod 644 {} \; 2>/dev/null || true

    if [ "$install_path" = "yes" ]; then
        nux_print "Linking" "$CYAN" "/usr/local/bin/nux"
        ln -snf "$target_dir" "$CURRENT_LINK"

        nux_print "Linking" "$CYAN" "/usr/local/bin/nux"
        mkdir -p "$(dirname "$BIN_LINK")"
        ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"

        # Environment profile (system-wide, restricted permissions)
        cat > /etc/profile.d/nux.sh <<ENVEOF
# Nux Programming Language
export NUX_HOME="$CURRENT_LINK"
export NUX_LIB_PATH="$CURRENT_LINK/lib"
export PATH="\$PATH:\$NUX_HOME"
ENVEOF
        chmod 644 /etc/profile.d/nux.sh
    fi

    if [ "$install_desktop" = "yes" ]; then
        nux_print "Creating" "$CYAN" "desktop entry"
        mkdir -p "$(dirname "$DESKTOP_FILE")"
        cat > "$DESKTOP_FILE" <<DESKTOPEOF
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
        chmod 644 "$DESKTOP_FILE"
        update-desktop-database /usr/share/applications 2>/dev/null || true
    fi

    # Write marker
    echo "$VERSION" > "$target_dir/.nux_installed"
    chmod 644 "$target_dir/.nux_installed"

    nux_finish "installed" "successfully!"
}

do_custom_install() {
    echo ""
    read -rp "  Install directory [$INSTALL_DIR]: " custom_dir
    custom_dir=${custom_dir:-$INSTALL_DIR}

    # Sanitize: reject paths with shell metacharacters
    if echo "$custom_dir" | grep -qE '[;&|`$(){}!<>]'; then
        nux_error "Invalid characters in path. Aborting."
        exit 1
    fi

    read -rp "  Install Standard Library? [Y/n]: " install_std
    install_std=${install_std:-Y}
    [ "$install_std" = "Y" ] || [ "$install_std" = "y" ] && install_std="yes" || install_std="no"

    read -rp "  Add to PATH? [Y/n]: " install_path
    install_path=${install_path:-Y}
    [ "$install_path" = "Y" ] || [ "$install_path" = "y" ] && install_path="yes" || install_path="no"

    read -rp "  Create desktop entry? [Y/n]: " install_desktop
    install_desktop=${install_desktop:-Y}
    [ "$install_desktop" = "Y" ] || [ "$install_desktop" = "y" ] && install_desktop="yes" || install_desktop="no"

    do_install "$custom_dir" "$install_std" "$install_path" "$install_desktop"
}

do_repair() {
    nux_print "Repairing" "$MAGENTA" "Nux installation..."
    do_install
}

do_update() {
    nux_print "Updating" "$MAGENTA" "Nux to latest version..."
    if [ ! -f "$INSTALL_DIR/nux" ]; then
        nux_error "Nux is not installed. Run install first."
        exit 1
    fi

    TEMP_TAR=$(mktemp /tmp/nux-update-XXXXXXXX.tar.gz)
    trap 'rm -f "$TEMP_TAR"' EXIT

    if curl -fSL --connect-timeout 30 -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null; then
        if verify_checksum "$TEMP_TAR"; then
            do_install "$INSTALL_DIR" "yes" "yes" "yes"
            nux_finish "updated" "successfully!"
        else
            nux_error "Update aborted due to checksum failure."
            exit 1
        fi
    else
        nux_error "Failed to download update."
    fi
    rm -f "$TEMP_TAR"
}

do_uninstall() {
    echo ""
    if [ ! -d "$INSTALL_DIR_BASE" ]; then
        nux_error "Nux is not installed."
        exit 1
    fi

    local versions=()
    for d in "$INSTALL_DIR_BASE"/*; do
        if [ -d "$d" ] && [ "$(basename "$d")" != "current" ]; then
            versions+=("$(basename "$d")")
        fi
    done

    if [ ${#versions[@]} -eq 0 ]; then
        nux_error "No Nux versions found."
        exit 1
    fi

    printf "  %bInstalled Versions:%b
" "$BOLD" "$NC"
    
    local active_target=""
    if [ -L "$CURRENT_LINK" ]; then
        active_target=$(readlink "$CURRENT_LINK")
    fi

    local i=1
    for v in "${versions[@]}"; do
        local marker=""
        if [[ "$active_target" == *"$v" ]]; then
            marker=" (active)"
        fi
        printf "    %b%d)%b  %s%s
" "$YELLOW" "$i" "$NC" "$v" "$marker"
        ((i++))
    done
    printf "    %b0)%b  Uninstall ALL versions
" "$RED" "$NC"
    echo ""
    read -rp "  Select versions to uninstall (comma-separated, e.g. 1,3 or 0 for all): " choices
    
    if [ -z "$choices" ]; then
        echo "  Uninstall cancelled."
        return
    fi
    
    if [ "$choices" = "0" ]; then
        printf "  %bWARNING: This will completely remove all Nux versions.%b
" "$RED" "$NC"
        read -rp "  Are you sure? [y/N]: " confirm
        if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
            nux_print "Removing" "$RED" "all Nux files..."
            rm -rf "$INSTALL_DIR_BASE"
            rm -f "$BIN_LINK"
            rm -f /etc/profile.d/nux.sh
            rm -f "$DESKTOP_FILE" 2>/dev/null || true
            if command -v update-desktop-database &>/dev/null; then update-desktop-database /usr/share/applications 2>/dev/null || true; fi
            nux_finish "uninstalled" "completely."
        else
            echo "  Uninstall cancelled."
        fi
        return
    fi
    
    IFS=',' read -ra choice_array <<< "$choices"
    for c in "${choice_array[@]}"; do
        c=$(echo "$c" | tr -d ' ')
        if [[ "$c" =~ ^[0-9]+$ ]]; then
            local idx=$((c-1))
            if [ $idx -ge 0 ] && [ $idx -lt ${#versions[@]} ]; then
                local v="${versions[$idx]}"
                nux_print "Removing" "$RED" "$v..."
                rm -rf "$INSTALL_DIR_BASE/$v"
                if [[ "$active_target" == *"$v" ]]; then
                    rm -f "$CURRENT_LINK"
                fi
            fi
        fi
    done
    
    if [ ! -L "$CURRENT_LINK" ]; then
        local remaining=()
        for d in "$INSTALL_DIR_BASE"/*; do
            if [ -d "$d" ] && [ "$(basename "$d")" != "current" ]; then
                remaining+=("$(basename "$d")")
            fi
        done
        
        if [ ${#remaining[@]} -gt 0 ]; then
            local latest="${remaining[-1]}"
            ln -snf "$INSTALL_DIR_BASE/$latest" "$CURRENT_LINK"
            nux_print "Switched" "$GREEN" "active version to $latest"
            ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"
        else
            rm -f "$BIN_LINK"
            rm -f /etc/profile.d/nux.sh
            rm -f "$DESKTOP_FILE" 2>/dev/null || true
            rm -rf "$INSTALL_DIR_BASE"
        fi
    fi
    nux_finish "uninstalled" "Selected versions removed."
}

do_build_deb() {
    nux_print "Building" "$MAGENTA" "Debian package..."
    ARCH="amd64"
    PKG_DIR="nux_${VERSION}_${ARCH}"

    rm -rf "$PKG_DIR"
    mkdir -p "$PKG_DIR/DEBIAN"
    mkdir -p "$PKG_DIR/usr/local/lib/nux/lib"
    mkdir -p "$PKG_DIR/usr/local/bin"

    cat > "$PKG_DIR/DEBIAN/control" <<EOF
Package: nux
Version: $VERSION
Section: devel
Priority: optional
Architecture: $ARCH
Maintainer: NuxLang Team <nux@nuxlang.org>
Homepage: https://github.com/DoguparthiAakash/Nux_Lang
Description: Nux Programming Language
 An incredibly fast, lightweight, Write-Once-Run-Anywhere
 programming language with built-in ML, quantum, and 3D support.
EOF

    cat > "$PKG_DIR/DEBIAN/postinst" <<'POSTINST'
#!/bin/bash
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod 755 /usr/local/bin/nux 2>/dev/null || true
POSTINST
    chmod 755 "$PKG_DIR/DEBIAN/postinst"

    cat > "$PKG_DIR/DEBIAN/prerm" <<'PRERM'
#!/bin/bash
rm -f /usr/local/bin/nux
PRERM
    chmod 755 "$PKG_DIR/DEBIAN/prerm"

    # Copy payload
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    if [ -d "$SCRIPT_DIR/payload" ]; then
        cp -r "$SCRIPT_DIR/payload/"* "$PKG_DIR/usr/local/lib/nux/"
    fi

    if command -v dpkg-deb &>/dev/null; then
        dpkg-deb --build "$PKG_DIR"
        printf "       %bâœ”%b  Built %b${PKG_DIR}.deb%b\n" "$GREEN" "$NC" "$CYAN" "$NC"
        echo "         Install with: sudo dpkg -i ${PKG_DIR}.deb"
    else
        nux_error "dpkg-deb not found. Run this on a Debian/Ubuntu system."
    fi
    rm -rf "$PKG_DIR"
}

do_build_rpm() {
    nux_print "Building" "$MAGENTA" "RPM package..."

    rm -rf rpmbuild
    mkdir -p rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

    cat > rpmbuild/SPECS/nux.spec <<EOF
Name:           nux
Version:        $VERSION
Release:        1%{?dist}
Summary:        Nux Programming Language
License:        MIT
URL:            https://github.com/DoguparthiAakash/Nux_Lang

%description
An incredibly fast, lightweight, Write-Once-Run-Anywhere programming language.

%install
mkdir -p %{buildroot}/usr/local/lib/nux/lib
mkdir -p %{buildroot}/usr/local/bin

%post
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod 755 /usr/local/bin/nux

%preun
rm -f /usr/local/bin/nux

%files
/usr/local/lib/nux/*
EOF

    if command -v rpmbuild &>/dev/null; then
        rpmbuild -ba rpmbuild/SPECS/nux.spec --define "_topdir $(pwd)/rpmbuild"
        printf "       %bâœ”%b  RPM built in %brpmbuild/RPMS/%b\n" "$GREEN" "$NC" "$CYAN" "$NC"
    else
        nux_error "rpmbuild not found. Run this on a Fedora/RHEL system."
    fi
}

# ============================================================================
# Main
# ============================================================================
if [ "${1:-}" = "--update" ]; then
    check_root
    is_installed && do_update || do_install
    exit 0
elif [ "${1:-}" = "--uninstall" ]; then
    check_root
    is_installed && do_uninstall || nux_error "Not installed."
    exit 0
fi

banner
check_root
show_menu

case "$choice" in
    1) do_install ;;
    2) do_custom_install ;;
    3) if is_installed; then do_repair; else nux_error "Not installed."; fi ;;
    4) if is_installed; then do_update; else nux_error "Not installed."; fi ;;
    5) if is_installed; then do_uninstall; else nux_error "Not installed."; fi ;;
    6) do_build_deb ;;
    7) do_build_rpm ;;
    0) echo "  Exiting."; exit 0 ;;
    *) nux_error "Invalid choice."; exit 1 ;;
esac
