#!/bin/bash
# Nux Uninstall Script (macOS)

set -e

INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"
LIB_DIR="/usr/local/lib/nux"
CONFIG_DIR="/etc/nux"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'

clear
echo ""
echo -e "${RED}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${RED}    ║                  ${WHITE}UNINSTALL NUX PROGRAMMING LANGUAGE${RED}                 ║${NC}"
echo -e "${RED}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [[ $EUID -ne 0 ]]; then
   echo -e "    ${RED}Error: This script must be run as root${NC}" 
   exit 1
fi

echo -e "    ${YELLOW}This will completely remove Nux from your system.${NC}"
read -p "    Are you sure you want to continue? [y/N] " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo -e "    ${GREEN}Aborted.${NC}"
    exit 0
fi
echo ""

echo -e "    ${CYAN}Removing files...${NC}"
rm -rf "$INSTALL_DIR" && echo -e "    ${GREEN}✓${NC} Removed installation directory"
rm -rf "$LIB_DIR" && echo -e "    ${GREEN}✓${NC} Removed libraries"
rm -rf "$CONFIG_DIR" && echo -e "    ${GREEN}✓${NC} Removed configuration"
rm -f "$BIN_DIR/nux" "$BIN_DIR/nuxc" "$BIN_DIR/nuxr" && echo -e "    ${GREEN}✓${NC} Removed executables"

# Remove shell integration if present
if [ -f "$HOME/.zshrc" ]; then
    sed -i '' '/Nux Programming Language/d' "$HOME/.zshrc"
    sed -i '' '/export NUX_HOME/d' "$HOME/.zshrc"
    sed -i '' '/export NUX_LIB/d' "$HOME/.zshrc"
    sed -i '' '/export PATH.*\/usr\/local\/nux\/bin/d' "$HOME/.zshrc"
    echo -e "    ${GREEN}✓${NC} Cleaned .zshrc"
fi

echo ""
echo -e "${GREEN}    Successfully uninstalled Nux. We're sad to see you go!${NC}"
echo ""
