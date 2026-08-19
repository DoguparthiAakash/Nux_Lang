#!/usr/bin/env bash
# ============================================================================
#  Nux Programming Language â€” macOS Installer
#  Security-hardened with checksum verification, symlink protection,
#  safe temp files, and Rust/Nux-inspired TUI art
# ============================================================================
set -euo pipefail

VERSION="1.0.0"
INSTALL_DIR_BASE="/usr/local/lib/nux"
INSTALL_DIR="$INSTALL_DIR_BASE/v$VERSION"
CURRENT_LINK="$INSTALL_DIR_BASE/current"
BIN_LINK="/usr/local/bin/nux"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-mac.tar.gz"
CHECKSUM_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-mac.sha256"
MARKER_FILE="$INSTALL_DIR/.nux_installed"

export PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
umask 022

# --- TUI Colors ---
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'
MAGENTA='\033[0;35m'; YELLOW='\033[1;33m'; WHITE='\033[1;37m'
DIM='\033[2m'; BOLD='\033[1m'; NC='\033[0m'
DARKGRAY='\033[38;5;8m'

nux_header() {
    local title="$1" subtitle="$2"
    echo ""
    printf "%bâ•­â”€ %bâ—† %b%s %bâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€\n" "$DARKGRAY" "$CYAN" "${WHITE}${BOLD}" "$title" "$DARKGRAY"
    printf "%bâ”‚  %bNux v%s  %bÂ·  %b%s\n" "$DARKGRAY" "${WHITE}${BOLD}" "$VERSION" "$DARKGRAY" "$DARKGRAY" "$subtitle"
    printf "%bâ•°â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€%b\n" "$DARKGRAY" "$NC"
}
nux_print() { local tag="$1" color="$2" msg="$3"; printf "%bâ”œâ”€ %bâœ¦ %b%s  %b%s%b\n" "$DARKGRAY" "$GREEN" "$color" "$tag" "$DARKGRAY" "$msg" "$NC"; }
nux_error() { printf "%bâ•°â”€ %bâœ• %berror  %b%s%b\n" "$DARKGRAY" "$RED" "$RED" "$WHITE" "$1" "$NC"; }
nux_warn()  { printf "%bâ”œâ”€ %bâš  %bwarning  %b%s%b\n" "$DARKGRAY" "$YELLOW" "$YELLOW" "$DARKGRAY" "$1" "$NC"; }
nux_finish() { local tag="$1" msg="$2"; printf "%bâ•°â”€ %bâ–¶ %b%s  %b%s%b\n" "$DARKGRAY" "$CYAN" "$CYAN" "$tag" "$WHITE" "$msg" "$NC"; }

banner() { nux_header "nux-installer" "interactive menu ..."; }

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        nux_error "This installer requires root privileges."
        echo "  Please re-run with: sudo $0"
        exit 1
    fi
}

is_installed() { [ -f "$MARKER_FILE" ] || command -v nux >/dev/null 2>&1 || [ -f "$INSTALL_DIR/nux" ]; }

verify_checksum() {
    local file="$1"
    nux_print "Verifying" "$CYAN" "download integrity..."
    local expected_hash=""
    expected_hash=$(curl -fsSL "$CHECKSUM_URL" 2>/dev/null | head -1 | awk '{print $1}') || true
    if [ -z "$expected_hash" ]; then
        nux_warn "Checksum file unavailable. Skipping."
        return 0
    fi
    local actual_hash
    actual_hash=$(shasum -a 256 "$file" 2>/dev/null | awk '{print $1}')
    if [ "$actual_hash" != "$expected_hash" ]; then
        nux_error "SHA-256 checksum mismatch! Download may be tampered."
        return 1
    fi
    nux_print "Verified" "$GREEN" "SHA-256 checksum OK"
    return 0
}

show_menu() {
    printf "  %bSelect an option:%b\n\n" "$BOLD" "$NC"
    printf "    %b1)%b  Install Now         %b(Recommended)%b\n" "$GREEN" "$NC" "$CYAN" "$NC"
    printf "    %b2)%b  Custom Install       Choose location and features\n" "$GREEN" "$NC"
    if is_installed; then
        printf "    %b3)%b  Repair              Reinstall all files\n" "$GREEN" "$NC"
        printf "    %b4)%b  Update              Fetch latest version\n" "$GREEN" "$NC"
        printf "    %b5)%b  Uninstall           Remove Nux completely\n" "$RED" "$NC"
    fi
    printf "    %b0)%b  Exit\n" "$GREEN" "$NC"
    echo ""
    read -rp "  Enter choice [1]: " choice
    choice=${choice:-1}
}

do_install() {
    local target_dir="${1:-$INSTALL_DIR}"
    local install_stdlib="${2:-yes}" install_path="${3:-yes}"

    echo ""
    nux_print "Compiling" "$MAGENTA" "installation plan..."

    if [ -L "$target_dir" ]; then
        nux_error "Install directory is a symlink. Aborting."
        exit 1
    fi

    mkdir -p "$target_dir" && chmod 755 "$target_dir"

    nux_print "Downloading" "$CYAN" "nux-mac.tar.gz..."
    TEMP_TAR=$(mktemp /tmp/nux-download-XXXXXXXX.tar.gz)
    trap 'rm -f "$TEMP_TAR"' EXIT

    local download_ok=false
    curl -fSL --connect-timeout 30 --max-time 120 -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null && download_ok=true

    if [ "$download_ok" = true ] && [ -s "$TEMP_TAR" ]; then
        if ! verify_checksum "$TEMP_TAR"; then
            rm -f "$TEMP_TAR"
            nux_error "Installation aborted."
            exit 1
        fi
        nux_print "Extracting" "$CYAN" "files..."
        rm -f "$target_dir/nux" 2>/dev/null || true
        tar --no-same-owner -xzf "$TEMP_TAR" -C "$target_dir" 2>/dev/null || true
    else
        nux_warn "Download unavailable. Attempting to compile from source..."
        SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
        SRC_DIR="$SCRIPT_DIR/../../nux/nux_oleg/nux_portable/runtime_c"
        if [ -d "$SRC_DIR" ]; then
            if command -v clang &>/dev/null; then
                nux_print "Compiling" "$CYAN" "using clang..."
                clang -O3 "$SRC_DIR/main.c" "$SRC_DIR/vm.c" "$SRC_DIR/vision/vision.c" -lm -o "$target_dir/nux"
            elif command -v gcc &>/dev/null; then
                nux_print "Compiling" "$CYAN" "using gcc..."
                gcc -O3 "$SRC_DIR/main.c" "$SRC_DIR/vm.c" "$SRC_DIR/vision/vision.c" -lm -o "$target_dir/nux"
            else
                nux_error "No C compiler (clang/gcc) found. Installation failed."
                exit 1
            fi
            if [ -f "$SRC_DIR/compiler.nuxi" ]; then
                cp "$SRC_DIR/compiler.nuxi" "$target_dir/"
            fi
        elif [ -d "$SCRIPT_DIR/payload" ]; then
            nux_warn "Source directory not found. Using local payload."
            cp -r "$SCRIPT_DIR/payload/"* "$target_dir/"
        else
            nux_error "No payload or source available. Installation aborted."
            exit 1
        fi
    fi
    rm -f "$TEMP_TAR"

    [ "$install_stdlib" = "yes" ] && mkdir -p "$target_dir/lib"

    chmod 755 "$target_dir/nux" 2>/dev/null || true

    if [ "$install_path" = "yes" ]; then
        nux_print "Linking" "$CYAN" "/usr/local/bin/nux"
        ln -sf "$target_dir/nux" "$BIN_LINK"
        for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
            if [ -f "$profile" ] && ! grep -q "NUX_HOME" "$profile" 2>/dev/null; then
                printf '\n# Nux Programming Language\nexport NUX_HOME="%s"\nexport NUX_LIB_PATH="%s/lib"\n' "$target_dir" "$target_dir" >> "$profile"
            fi
        done
    fi

    echo "$VERSION" > "$target_dir/.nux_installed"
    nux_finish "installed" "successfully!"
}

do_custom_install() {
    read -rp "  Install directory [$INSTALL_DIR]: " custom_dir
    custom_dir=${custom_dir:-$INSTALL_DIR}
    echo "$custom_dir" | grep -qE '[;&|`$(){}!<>]' && { nux_error "Invalid path characters."; exit 1; }
    read -rp "  Install Standard Library? [Y/n]: " s; s=${s:-Y}
    [ "$s" = "Y" ] || [ "$s" = "y" ] && s="yes" || s="no"
    read -rp "  Add to PATH? [Y/n]: " p; p=${p:-Y}
    [ "$p" = "Y" ] || [ "$p" = "y" ] && p="yes" || p="no"
    do_install "$custom_dir" "$s" "$p"
}

do_update() {
    nux_print "Updating" "$MAGENTA" "Nux..."
    TEMP_TAR=$(mktemp /tmp/nux-update-XXXXXXXX.tar.gz)
    trap 'rm -f "$TEMP_TAR"' EXIT
    if curl -fSL --connect-timeout 30 -o "$TEMP_TAR" "$DOWNLOAD_URL" 2>/dev/null && verify_checksum "$TEMP_TAR"; then
        rm -f "$INSTALL_DIR/nux" 2>/dev/null || true
        tar --no-same-owner -xzf "$TEMP_TAR" -C "$INSTALL_DIR"
        nux_finish "updated" "successfully!"
    else
        nux_error "Update failed."
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
            for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
                [ -f "$profile" ] && sed -i.bak '/NUX_HOME\|NUX_LIB_PATH\|# Nux Programming/d' "$profile" 2>/dev/null && rm -f "${profile}.bak"
            done
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
            for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
                [ -f "$profile" ] && sed -i.bak '/NUX_HOME\|NUX_LIB_PATH\|# Nux Programming/d' "$profile" 2>/dev/null && rm -f "${profile}.bak"
            done
            rm -rf "$INSTALL_DIR_BASE"
        fi
    fi
    nux_finish "uninstalled" "Selected versions removed."
}

if [ "${1:-}" = "--update" ]; then
    check_root
    is_installed && do_update || do_install
    exit 0
elif [ "${1:-}" = "--uninstall" ]; then
    check_root
    is_installed && do_uninstall || nux_error "Not installed."
    exit 0
fi

banner; check_root; show_menu
case "$choice" in
    1) do_install ;; 2) do_custom_install ;; 3) is_installed && do_install || nux_error "Not installed." ;;
    4) is_installed && do_update || nux_error "Not installed." ;; 5) is_installed && do_uninstall || nux_error "Not installed." ;;
    0) exit 0 ;; *) nux_error "Invalid choice."; exit 1 ;;
esac
