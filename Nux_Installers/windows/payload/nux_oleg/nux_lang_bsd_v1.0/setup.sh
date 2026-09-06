#!/bin/sh
# Nux Setup Manager (BSD)
# Central dispatcher for installing, repairing, and managing Nux

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Function to run sub-scripts
run_script() {
    script_name="$1"
    script_path="$SCRIPT_DIR/$script_name"
    
    if [ ! -f "$script_path" ]; then
        echo "${RED}Error: $script_name not found!${NC}"
        echo "You might need to repair your installer."
        exit 1
    fi
    
    # Check if script requires root
    if [ "$script_name" = "install.sh" ] || [ "$script_name" = "repair.sh" ] || [ "$script_name" = "uninstall.sh" ]; then
        if [ "$(id -u)" -ne 0 ]; then
            echo "${YELLOW}Administrator privileges required for this action.${NC}"
            sudo sh "$script_path"
            return
        fi
    fi
    
    sh "$script_path"
}

# 1. Check if Nux is already installed
if command -v nux >/dev/null 2>&1; then
    INSTALLED=true
else
    INSTALLED=false
fi

clear
echo ""
echo "${CYAN}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo "${CYAN}    ║                                                                   ║${NC}"
echo "${CYAN}    ║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║${NC}"
echo "${CYAN}    ║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║${NC}"
echo "${CYAN}    ║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║${NC}"
echo "${CYAN}    ║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║${NC}"
echo "${CYAN}    ║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║${NC}"
echo "${CYAN}    ║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║${NC}"
echo "${CYAN}    ║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║${NC}"
echo "${CYAN}    ║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║${NC}"
echo "${CYAN}    ║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║${NC}"
echo "${CYAN}    ║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║${NC}"
echo "${CYAN}    ║                                                                   ║${NC}"
echo "${CYAN}    ║               ${WHITE}Programming Language${CYAN} v1.0.0                          ║${NC}"
echo "${CYAN}    ║                                                                   ║${NC}"
echo "${CYAN}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ "$INSTALLED" = false ]; then
    echo "${YELLOW}Nux is not installed on this system.${NC}"
    echo ""
    printf "Do you want to install Nux now? [Y/n] "
    read choice
    choice=${choice:-Y}
    
    case "$choice" in
        [Yy]*) run_script "install.sh" ;;
        *) echo "Installation aborted."; exit 0 ;;
    esac
else
    echo "${GREEN}✓ Nux is currently installed.${NC}"
    echo ""
    echo "What would you like to do?"
    echo ""
    echo "   [1] ${YELLOW}Repair${NC} (Check for missing files & restore)"
    echo "   [2] ${RED}Uninstall${NC} (Remove Nux completely)"
    echo "   [3] ${CYAN}Check Health${NC} (Verify installation status)"
    echo "   [4] ${WHITE}Version Manager${NC} (Switch versions)"
    echo "   [5] Exit"
    echo ""
    printf "Select validation option [1-5]: "
    read choice
    
    case $choice in
        1) run_script "repair.sh" ;;
        2) run_script "uninstall.sh" ;;
        3) run_script "checker.sh" ;;
        4) run_script "version.sh" ;;
        *) echo "Exiting..."; exit 0 ;;
    esac
fi
