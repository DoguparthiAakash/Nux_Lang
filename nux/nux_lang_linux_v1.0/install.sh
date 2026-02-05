#!/bin/bash
# Nux Programming Language - Linux Setup Script
# Beautiful installer with enhanced UI

set -e

VERSION="1.0.0"
INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"
LIB_DIR="/usr/local/lib/nux"
CONFIG_DIR="/etc/nux"
NUX_HOME="$HOME/.nux"

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
STAR="★"
GEAR="⚙"
PACKAGE="📦"
ROCKET="🚀"
FOLDER="📁"
WRENCH="🔧"
SPARKLE="✨"

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

move_cursor() {
    printf "\033[%d;%dH" "$1" "$2"
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
    echo "    ║               ${WHITE}Programming Language${CYAN} v${VERSION}                    ║"
    echo "    ║                                                                   ║"
    echo "    ╚═══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
}

# ... (rest of file) ...



# ... (rest of file) ...

print_success() {
    echo ""
    echo -e "    ${GREEN}╔═══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${SPARKLE} ${WHITE}Installation Complete!${GREEN}                                     ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
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
    echo -e "    ${GREEN}║               ${WHITE}Programming Language${GREEN} v${VERSION}                    ║${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}╠═══════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "    ${GREEN}║                                                                   ║${NC}"
    echo -e "    ${GREEN}║   ${ROCKET} ${CYAN}Get Started:${GREEN}                                               ║${NC}"


print_section() {
    local title="$1"
    echo ""
    echo -e "    ${CYAN}┌──────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "    ${CYAN}│${NC}  ${BOLD}${WHITE}$title${NC}"
    echo -e "    ${CYAN}└──────────────────────────────────────────────────────────────────┘${NC}"
}

# Animated spinner
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

# Progress bar
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

# Step indicator
step() {
    local num=$1
    local total=$2
    local message="$3"
    echo ""
    echo -e "    ${MAGENTA}[${num}/${total}]${NC} ${BOLD}${message}${NC}"
}

# Status messages
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

# Animated task
run_task() {
    local message="$1"
    shift
    local command="$@"
    
    printf "    ${CYAN}⠋${NC}  ${message}"
    
    # Run command in background and capture output
    local output
    output=$($command 2>&1) &
    local pid=$!
    
    spinner $pid "$message"
    wait $pid
    local status=$?
    
    if [ $status -eq 0 ]; then
        status_ok "$message"
    else
        status_fail "$message"
        echo -e "    ${DIM}${output}${NC}"
        return 1
    fi
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

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO="$NAME"
        DISTRO_ID="$ID"
    elif [ -f /etc/lsb-release ]; then
        . /etc/lsb-release
        DISTRO="$DISTRIB_DESCRIPTION"
        DISTRO_ID="$DISTRIB_ID"
    else
        DISTRO="Unknown Linux"
        DISTRO_ID="linux"
    fi
}

show_system_info() {
    detect_distro
    print_section "${GEAR} System Information"
    echo ""
    echo -e "    ${GRAY}├─${NC} ${WHITE}OS:${NC}          $DISTRO"
    echo -e "    ${GRAY}├─${NC} ${WHITE}Kernel:${NC}      $(uname -r)"
    echo -e "    ${GRAY}├─${NC} ${WHITE}Arch:${NC}        $(uname -m)"
    echo -e "    ${GRAY}└─${NC} ${WHITE}User:${NC}        ${SUDO_USER:-$USER}"
}

check_dependencies() {
    print_section "${PACKAGE} Checking Dependencies"
    echo ""
    
    local deps=("gcc" "make" "git")
    local missing=()
    local total=${#deps[@]}
    local current=0
    
    for dep in "${deps[@]}"; do
        current=$((current + 1))
        sleep 0.2
        
        if command -v "$dep" &> /dev/null; then
            status_ok "$dep $(command -v $dep | xargs dirname)"
        else
            status_warn "$dep not found"
            missing+=("$dep")
        fi
        progress_bar $current $total
    done
    echo ""
    
    if [ ${#missing[@]} -ne 0 ]; then
        echo ""
        status_info "Installing missing: ${missing[*]}"
        
        if command -v apt-get &> /dev/null; then
            apt-get update -qq && apt-get install -y -qq "${missing[@]}" >/dev/null 2>&1
        elif command -v dnf &> /dev/null; then
            dnf install -y -q "${missing[@]}" >/dev/null 2>&1
        elif command -v pacman &> /dev/null; then
            pacman -S --noconfirm --quiet "${missing[@]}" >/dev/null 2>&1
        fi
        
        status_ok "Dependencies installed"
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
        sleep 0.05
    done
    echo ""
    status_ok "Created ${#dirs[@]} directories"
}

install_runtime() {
    print_section "${WRENCH} Installing Runtime"
    echo ""
    
    # Check for binaries in package
    if [ -f "../bin/nux" ]; then
        status_info "Installing binaries..."
        
        # Copy nux interpreter
        cp "../bin/nux" "$INSTALL_DIR/bin/"
        chmod +x "$INSTALL_DIR/bin/nux"
        status_ok "Installed 'nux' interpreter"
        
        # Copy nuxc compiler if exists
        if [ -f "../bin/nuxc" ]; then
            cp "../bin/nuxc" "$INSTALL_DIR/bin/"
            chmod +x "$INSTALL_DIR/bin/nuxc"
            status_ok "Installed 'nuxc' compiler"
        fi
    else
        status_warn "Binaries not found in package. Creating fallback launcher..."
        # Create launcher
        cat > "$INSTALL_DIR/bin/nux" << 'LAUNCHER'
#!/bin/bash
NUX_HOME="${NUX_HOME:-$HOME/.nux}"
NUX_LIB="/usr/local/lib/nux"

case "${1:-repl}" in
    repl)
        echo -e "\033[0;36mNux REPL v1.0.0\033[0m"
        echo "Type 'exit' to quit"
        while true; do
            echo -n -e "\033[0;33mnux>\033[0m "
            read -r line
            [ "$line" = "exit" ] && break
        done
        ;;
    --version|-v) echo "Nux v1.0.0 (Linux)" ;;
    --help|-h)
        echo "Nux Programming Language v1.0.0"
        echo "Usage: nux [file.nux] | repl | compile | run"
        ;;
    *) echo "Running $1..." ;;
esac
LAUNCHER
        chmod +x "$INSTALL_DIR/bin/nux"
        status_ok "Fallback runtime installed"
    fi
    
    # Create symlinks
    status_info "Creating symlinks..."
    ln -sf "$INSTALL_DIR/bin/nux" "$BIN_DIR/nux"
    [ -f "$INSTALL_DIR/bin/nuxc" ] && ln -sf "$INSTALL_DIR/bin/nuxc" "$BIN_DIR/nuxc"
    status_ok "Symlinks created"
}

install_libraries() {
    print_section "${PACKAGE} Installing Libraries"
    echo ""
    
    local lib_count=0
    
    if [ -d "../lib" ]; then
        for dir in std ai os embedded; do
            if [ -d "../lib/$dir" ]; then
                local count=$(find "../lib/$dir" -name "*.nux" 2>/dev/null | wc -l)
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
        status_ok "Total: $lib_count library files installed"
    fi
}

configure_environment() {
    print_section "${GEAR} Configuring Environment"
    echo ""
    
    # Create system config
    cat > "$CONFIG_DIR/nux.conf" << EOF
# Nux Configuration
[paths]
lib_path = /usr/local/lib/nux
[runtime]
max_memory = 1024M
EOF
    status_ok "System config created"
    
    # Create profile script
    cat > /etc/profile.d/nux.sh << 'EOF'
export NUX_HOME="$HOME/.nux"
export NUX_LIB="/usr/local/lib/nux"
export PATH="$PATH:/usr/local/nux/bin"
EOF
    chmod +x /etc/profile.d/nux.sh
    status_ok "Shell profile configured"
    
    # Setup user directory
    ACTUAL_USER="${SUDO_USER:-$USER}"
    ACTUAL_HOME=$(eval echo "~$ACTUAL_USER")
    
    mkdir -p "$ACTUAL_HOME/.nux"/{lib,cache,projects}
    chown -R "$ACTUAL_USER:$ACTUAL_USER" "$ACTUAL_HOME/.nux"
    status_ok "User directory created: ~/.nux"
}



# ╔══════════════════════════════════════════════════════════════╗
# ║                          MAIN                                 ║
# ╚══════════════════════════════════════════════════════════════╝

trap 'show_cursor; exit' INT TERM

main() {
    print_banner
    check_root
    show_system_info
    check_dependencies
    create_directories
    install_runtime
    install_libraries
    configure_environment
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
    rm -f /etc/profile.d/nux.sh
    status_ok "Nux has been uninstalled"
    echo ""
    exit 0
fi

main
