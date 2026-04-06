#!/bin/sh
# Nux Uninstall Script (BSD)

set -e

INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"
LIB_DIR="/usr/local/lib/nux"
CONFIG_DIR="/usr/local/etc/nux"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'

clear
echo ""
echo "${RED}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo "${RED}    ║                  ${WHITE}UNINSTALL NUX PROGRAMMING LANGUAGE${RED}                 ║${NC}"
echo "${RED}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ "$(id -u)" -ne 0 ]; then
   echo "    ${RED}Error: This script must be run as root${NC}" 
   exit 1
fi

echo "    ${YELLOW}This will completely remove Nux from your system.${NC}"
printf "    Are you sure you want to continue? [y/N] "
read confirm
case "$confirm" in
    [Yy]*) ;;
    *) echo "    ${GREEN}Aborted.${NC}"; exit 0 ;;
esac
echo ""

echo "    ${CYAN}Removing files...${NC}"
rm -rf "$INSTALL_DIR" && echo "    ${GREEN}✓${NC} Removed installation directory"
rm -rf "$LIB_DIR" && echo "    ${GREEN}✓${NC} Removed libraries"
rm -rf "$CONFIG_DIR" && echo "    ${GREEN}✓${NC} Removed configuration"
rm -f "$BIN_DIR/nux" "$BIN_DIR/nuxc" "$BIN_DIR/nuxr" && echo "    ${GREEN}✓${NC} Removed executables"

echo ""
echo "${GREEN}    Successfully uninstalled Nux. We're sad to see you go!${NC}"
echo ""
