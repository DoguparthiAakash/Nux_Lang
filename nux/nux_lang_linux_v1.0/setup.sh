#!/bin/bash
# Nux Setup Manager
# Central dispatcher for installing, repairing, and managing Nux

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Function to run sub-scripts
run_script() {
    local script_name="$1"
    local script_path="$SCRIPT_DIR/$script_name"
    
    if [ ! -f "$script_path" ]; then
        echo -e "${RED}Error: $script_name not found!${NC}"
        echo "You might need to repair your installer."
        exit 1
    fi
    
    bash "$script_path"
}

# 1. Check if Nux is already installed
if command -v nux &> /dev/null; then
    INSTALLED=true
else
    INSTALLED=false
fi

clear
echo ""
echo -e "${CYAN}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}    ║                                                                   ║${NC}"
echo -e "${CYAN}    ║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║${NC}"
echo -e "${CYAN}    ║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║${NC}"
echo -e "${CYAN}    ║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║${NC}"
echo -e "${CYAN}    ║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║${NC}"
echo -e "${CYAN}    ║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║${NC}"
echo -e "${CYAN}    ║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║${NC}"
echo -e "${CYAN}    ║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║${NC}"
echo -e "${CYAN}    ║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║${NC}"
echo -e "${CYAN}    ║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║${NC}"
echo -e "${CYAN}    ║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║${NC}"
echo -e "${CYAN}    ║                                                                   ║${NC}"
echo -e "${CYAN}    ║               ${WHITE}Programming Language${CYAN} v1.0.0                          ║${NC}"
echo -e "${CYAN}    ║                                                                   ║${NC}"
echo -e "${CYAN}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ "$INSTALLED" = false ]; then
    echo -e "${YELLOW}Nux is not installed on this system.${NC}"
    echo ""
    read -p "Do you want to install Nux now? [Y/n] " choice
    choice=${choice:-Y}
    
    if [[ "$choice" =~ ^[Yy]$ ]]; then
        run_script "install.sh"
    else
        echo "Installation aborted."
        exit 0
    fi
else
    echo -e "${GREEN}✓ Nux is currently installed.${NC}"
    echo ""
    echo "What would you like to do?"
    echo ""
    echo -e "   [1] ${YELLOW}Repair${NC} (Check for missing files & restore)"
    echo -e "   [2] ${RED}Uninstall${NC} (Remove Nux completely)"
    echo -e "   [3] ${CYAN}Check Health${NC} (Verify installation status)"
    echo -e "   [4] ${WHITE}Version Manager${NC} (Switch versions)"
    echo -e "   [5] Exit"
    echo ""
    read -p "Select validation option [1-5]: " choice
    
    case $choice in
        1) run_script "repair.sh" ;;
        2) run_script "uninstall.sh" ;;
        3) run_script "checker.sh" ;;
        4) run_script "version.sh" ;;
        *) echo "Exiting..."; exit 0 ;;
    esac
fi
