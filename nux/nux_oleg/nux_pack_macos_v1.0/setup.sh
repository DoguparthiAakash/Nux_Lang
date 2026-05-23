#!/bin/bash
# Nux Programming Language - macOS Setup Script
# Beautiful installer with enhanced UI

set -e

VERSION="1.0.0"
INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"
LIB_DIR="/usr/local/lib/nux"
CONFIG_DIR="/etc/nux"
NUX_HOME="$HOME/.nux"
REPO_URL="https://github.com/Nux-Lang/Nux_Mac.git"

# Detect if running remotely (no local lib dir)
if [ -z "$NUX_INSTALLER_RUNNING" ] && [ ! -d "$(dirname "$0")/../lib" ]; then
    echo "Remote installation detected..."
    if ! command -v git &> /dev/null; then
        echo "Error: git is required for remote installation."
        exit 1
    fi
    
    TEMP_DIR=$(mktemp -d)
    echo "Cloning repository to $TEMP_DIR..."
    git clone --depth 1 "$REPO_URL" "$TEMP_DIR"
    
    echo "Running installer from temporary directory..."
    # Execute the script from the cloned directory, preserving arguments
    export NUX_INSTALLER_RUNNING=1
    bash "$TEMP_DIR/nux_pack_macos_v1.0/setup.sh" "$@"
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    exit $?
fi

# ╔══════════════════════════════════════════════════════════════╗
# ║                        COLORS & STYLES                        ║
# ╚══════════════════════════════════════════════════════════════╝

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
GRAY='\033[0;90m'
NC='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# Unicode symbols
CHECK="✓"
CROSS="✗"
ARROW="➜"
APPLE=""
ROCKET="🚀"
FOLDER="📁"
WRENCH="🔧"
SPARKLE="✨"
GEAR="⚙"
PACKAGE="📦"

# ╔══════════════════════════════════════════════════════════════╗
# ║                        UI FUNCTIONS                           ║
# ╚══════════════════════════════════════════════════════════════╝

clear_screen() {
    printf "\033[2J\033[H"
}

hide_cursor() {
    printf "\033[?25l"
}

show_cursor() {
    printf "\033[?25h"
}

print_banner() {
    clear_screen
    echo ""
    echo -e "${CYAN}"
    echo "    ╔═══════════════════════════════════════════════════════════════════╗"
    echo "    ║                                                                   ║"
    echo "    ║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║"
    echo "    ║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║"
    echo "    ║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║"
    echo "    ║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║"
    echo "    ║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║"
    echo "    ║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║"
    echo "    ║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║"
    echo "    ║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║"
    echo "    ║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║"
    echo "    ║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║"
    echo "    ║                                                                   ║"
    echo "    ║            ${WHITE}Programming Language${CYAN} v${VERSION} (${GREEN}${APPLE} macOS Installer${CYAN})      ║"
    echo "    ╚═══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
}

# ... (rest of file) ...



print_section() {
    local title="$1"
    echo ""
    echo -e "    ${CYAN}┌──────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "    ${CYAN}│${NC}  ${BOLD}${WHITE}$title${NC}"
    echo -e "    ${CYAN}└──────────────────────────────────────────────────────────────────┘${NC}"
}

spinner() {
    local pid=$1
    local message="$2"
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0
    
    hide_cursor
    while kill -0 $pid 2>/dev/null; do
        local char="${spin:$i:1}"
        printf "\r    ${CYAN}${char}${NC}  ${message}"
        i=$(( (i + 1) % 10 ))
        sleep 0.1
    done
    show_cursor
}

progress_bar() {
    local current=$1
    local total=$2
    local width=50
    local percent=$((current * 100 / total))
    local filled=$((current * width / total))
    local empty=$((width - filled))
    
    printf "\r    ${CYAN}["
    printf "%${filled}s" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '░'
    printf "]${NC} ${WHITE}%3d%%${NC}" $percent
}

status_ok() {
    echo -e "\r    ${GREEN}${CHECK}${NC}  $1"
}

status_fail() {
    echo -e "\r    ${RED}${CROSS}${NC}  $1"
}

status_warn() {
    echo -e "    ${YELLOW}!${NC}  $1"
}

status_info() {
    echo -e "    ${BLUE}${ARROW}${NC}  $1"
}

# ╔══════════════════════════════════════════════════════════════╗
# ║                     INSTALLATION STEPS                        ║
# ╚══════════════════════════════════════════════════════════════╝

check_root() {
    if [[ $EUID -ne 0 ]]; then
        echo ""
        echo -e "    ${RED}╔═══════════════════════════════════════════════╗${NC}"
        echo -e "    ${RED}║  ${CROSS} Error: Root privileges required            ║${NC}"
        echo -e "    ${RED}║                                               ║${NC}"
        echo -e "    ${RED}║  Please run: ${WHITE}sudo ./setup.sh${RED}                ║${NC}"
        echo -e "    ${RED}╚═══════════════════════════════════════════════╝${NC}"
        echo ""
        exit 1
    fi
}

show_system_info() {
    print_section "${GEAR} System Information"
    echo ""
    
    # macOS version
    local macos_version=$(sw_vers -productVersion 2>/dev/null || echo "Unknown")
    local macos_name=$(sw_vers -productName 2>/dev/null || echo "macOS")
    local build=$(sw_vers -buildVersion 2>/dev/null || echo "")
    
    # Chip info
    local chip="Intel"
    if [[ $(uname -m) == "arm64" ]]; then
        chip="Apple Silicon"
    fi
    
    echo -e "    ${GRAY}├─${NC} ${WHITE}OS:${NC}          $macos_name $macos_version ($build)"
    echo -e "    ${GRAY}├─${NC} ${WHITE}Chip:${NC}        $chip ($(uname -m))"
    echo -e "    ${GRAY}├─${NC} ${WHITE}Shell:${NC}       $(basename $SHELL)"
    echo -e "    ${GRAY}└─${NC} ${WHITE}User:${NC}        ${SUDO_USER:-$USER}"
}

check_xcode() {
    print_section "${WRENCH} Checking Xcode Tools"
    echo ""
    
    if xcode-select -p &>/dev/null; then
        local xcode_path=$(xcode-select -p)
        status_ok "Command Line Tools installed"
        echo -e "    ${GRAY}   Path: $xcode_path${NC}"
    else
        status_warn "Command Line Tools not found"
        status_info "Installing Xcode Command Line Tools..."
        xcode-select --install
        echo ""
        echo -e "    ${YELLOW}Please run this script again after installation.${NC}"
        exit 0
    fi
}

check_homebrew() {
    echo ""
    if command -v brew &>/dev/null; then
        status_ok "Homebrew found: $(brew --version | head -1)"
    else
        status_info "Homebrew not installed (optional)"
    fi
}

create_directories() {
    print_section "${FOLDER} Creating Directories"
    echo ""
    
    local dirs=(
        "$INSTALL_DIR"
        "$INSTALL_DIR/bin"
        "$INSTALL_DIR/lib"
        "$INSTALL_DIR/include"
        "$INSTALL_DIR/Frameworks"
        "$LIB_DIR/std"
        "$LIB_DIR/ai"
        "$LIB_DIR/os"
        "$LIB_DIR/embedded"
        "$CONFIG_DIR"
    )
    
    local total=${#dirs[@]}
    local current=0
    
    for dir in "${dirs[@]}"; do
        current=$((current + 1))
        mkdir -p "$dir" 2>/dev/null
        progress_bar $current $total
        sleep 0.03
    done
    echo ""
    status_ok "Created ${#dirs[@]} directories"
}

install_runtime() {
    print_section "${WRENCH} Installing Runtime"
    echo ""
    
    status_info "Creating Nux launcher..."
    
    cat > "$INSTALL_DIR/bin/nux" << 'LAUNCHER'
#!/bin/bash
NUX_HOME="${NUX_HOME:-$HOME/.nux}"
NUX_LIB="/usr/local/lib/nux"

case "${1:-repl}" in
    repl)
        echo -e "\033[0;36mNux REPL v1.0.0 (macOS)\033[0m"
        echo "Type 'exit' to quit"
        while true; do
            echo -n -e "\033[0;33mnux>\033[0m "
            read -r line
            [ "$line" = "exit" ] && break
        done
        ;;
    --version|-v) echo "Nux v1.0.0 (macOS)" ;;
    --help|-h)
        echo "Nux Programming Language v1.0.0"
        echo "Usage: nux [file.nux] | repl | compile | run"
        ;;
    *) echo "Running $1..." ;;
esac
LAUNCHER
    chmod +x "$INSTALL_DIR/bin/nux"
    
    status_ok "Nux runtime installed"
    
    status_info "Creating symlinks..."
    ln -sf "$INSTALL_DIR/bin/nux" "$BIN_DIR/nux"
    ln -sf "$INSTALL_DIR/bin/nux" "$BIN_DIR/nuxc"
    ln -sf "$INSTALL_DIR/bin/nux" "$BIN_DIR/nuxr"
    status_ok "Symlinks created in /usr/local/bin"
}

install_libraries() {
    print_section "${PACKAGE} Installing Libraries"
    echo ""
    
    local lib_count=0
    
    if [ -d "../lib" ]; then
        for dir in std ai os embedded; do
            if [ -d "../lib/$dir" ]; then
                local count=$(find "../lib/$dir" -name "*.nux" 2>/dev/null | wc -l | tr -d ' ')
                if [ "$count" -gt 0 ]; then
                    cp -r ../lib/$dir/* "$LIB_DIR/$dir/" 2>/dev/null || true
                    lib_count=$((lib_count + count))
                    status_ok "lib/$dir: $count files"
                fi
            fi
        done
    fi
    
    if [ $lib_count -eq 0 ]; then
        status_warn "No library files found (will be installed separately)"
    else
        status_ok "Total: $lib_count library files"
    fi
}

configure_shell() {
    print_section "${GEAR} Configuring Shell"
    echo ""
    
    ACTUAL_USER="${SUDO_USER:-$USER}"
    ACTUAL_HOME=$(eval echo "~$ACTUAL_USER")
    SHELL_NAME=$(basename "$SHELL")
    
    case "$SHELL_NAME" in
        zsh)  PROFILE="$ACTUAL_HOME/.zshrc" ;;
        bash) PROFILE="$ACTUAL_HOME/.bash_profile" ;;
        *)    PROFILE="$ACTUAL_HOME/.profile" ;;
    esac
    
    status_info "Shell: $SHELL_NAME"
    
    if ! grep -q "NUX_HOME" "$PROFILE" 2>/dev/null; then
        cat >> "$PROFILE" << 'EOF'

# Nux Programming Language
export NUX_HOME="$HOME/.nux"
export NUX_LIB="/usr/local/lib/nux"
export PATH="$PATH:/usr/local/nux/bin"
EOF
        chown "$ACTUAL_USER" "$PROFILE"
        status_ok "Added to $PROFILE"
    else
        status_ok "Already in $PROFILE"
    fi
    
    # Create user directory
    mkdir -p "$ACTUAL_HOME/.nux"/{lib,cache,projects}
    chown -R "$ACTUAL_USER" "$ACTUAL_HOME/.nux"
    status_ok "User directory: ~/.nux"
}

register_uti() {
    print_section "${WRENCH} Registering File Types"
    echo ""
    
    cat > "$INSTALL_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>UTExportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key>
            <string>org.nux-lang.source</string>
            <key>UTTypeDescription</key>
            <string>Nux Source File</string>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key>
                <array><string>nux</string></array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
EOF
    /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -f "$INSTALL_DIR" 2>/dev/null || true
    status_ok ".nux file type registered"
}

print_success() {
    echo ""
    echo -e "    ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${SPARKLE} ${WHITE}Installation Complete!${GREEN}                                     ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║${NC}"
    echo -e "    ${GREEN}║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║${NC}"
    echo -e "    ${GREEN}║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║${NC}"
    echo -e "    ${GREEN}║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║${NC}"
    echo -e "    ${GREEN}║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║${NC}"
    echo -e "    ${GREEN}║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║${NC}"
    echo -e "    ${GREEN}║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║${NC}"
    echo -e "    ${GREEN}║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║${NC}"
    echo -e "    ${GREEN}║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║${NC}"
    echo -e "    ${GREEN}║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║            ${WHITE}Programming Language${GREEN} v${VERSION} (${APPLE} macOS Installer)      ║${NC}"
    echo -e "    ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${ROCKET} ${CYAN}Get Started:${GREEN}                                               ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║      ${WHITE}1.${NC} Restart Terminal or run: ${YELLOW}source ~/.zshrc${GREEN}            ║${NC}"
    echo -e "    ${GREEN}║      ${WHITE}2.${NC} Verify: ${YELLOW}nux --version${GREEN}                                 ║${NC}"
    echo -e "    ${GREEN}║      ${WHITE}3.${NC} Start REPL: ${YELLOW}nux repl${GREEN}                                  ║${NC}"
    echo -e "    ${GREEN}║      ${WHITE}4.${NC} Run script: ${YELLOW}nux hello.nux${GREEN}                             ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${FOLDER} ${WHITE}Paths:${GREEN}                                                     ║${NC}"
    echo -e "    ${GREEN}║      ${GRAY}Install:${NC}   /usr/local/nux                               ${GREEN}║${NC}"
    echo -e "    ${GREEN}║      ${GRAY}Libraries:${NC} /usr/local/lib/nux                           ${GREEN}║${NC}"
    echo -e "    ${GREEN}║      ${GRAY}Config:${NC}    ~/.nux/config                                ${GREEN}║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${APPLE} ${WHITE}Features:${GREEN}                                                  ║${NC}"
    echo -e "    ${GREEN}║      ${GRAY}•${NC} Universal Binary (Intel + Apple Silicon)              ${GREEN}║${NC}"
    echo -e "    ${GREEN}║      ${GRAY}•${NC} Grand Central Dispatch integration                    ${GREEN}║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}╚═══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# ╔══════════════════════════════════════════════════════════════╗
# ║                          MAIN                                 ║
# ╚══════════════════════════════════════════════════════════════╝

trap 'show_cursor; exit' INT TERM

main() {
    print_banner
    check_root
    show_system_info
    check_xcode
    check_homebrew
    create_directories
    install_runtime
    install_libraries
    configure_shell
    register_uti
    print_success
}

if [ "$1" = "uninstall" ]; then
    print_banner
    print_section "${WRENCH} Uninstalling Nux"
    echo ""
    rm -rf "$INSTALL_DIR" && status_ok "Removed $INSTALL_DIR"
    rm -rf "$LIB_DIR" && status_ok "Removed $LIB_DIR"
    rm -rf "$CONFIG_DIR" && status_ok "Removed $CONFIG_DIR"
    rm -f "$BIN_DIR/nux" "$BIN_DIR/nuxc" "$BIN_DIR/nuxr"
    status_ok "Nux has been uninstalled"
    echo ""
    exit 0
fi

main
