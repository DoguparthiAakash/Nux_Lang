#!/bin/sh
# Nux Health Checker (BSD)
# Validates installation integrity

INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'
CHECK="✓"
CROSS="✗"

# Banner
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
echo "    ${CYAN}Starting System Health Check (BSD)...${NC}"
echo ""

ERRORS=0

check_file() {
    if [ -f "$1" ]; then
        echo "    ${GREEN}${CHECK}${NC} Found: $1"
    else
        echo "    ${RED}${CROSS} MISSING:${NC} $1"
        ERRORS=$((ERRORS+1))
    fi
}

echo "    ${WHITE}Checking Core Binaries:${NC}"
check_file "$INSTALL_DIR/bin/nux"
check_file "$BIN_DIR/nux"

echo ""
echo "    ${WHITE}Checking Environment:${NC}"
if command -v nux >/dev/null 2>&1; then
    echo "    ${GREEN}${CHECK}${NC} 'nux' command is in PATH"
    VER=$(nux --version 2>/dev/null || echo "Unknown")
    echo "    ${BLUE}ℹ${NC}  Detected Version: $VER"
else
    echo "    ${RED}${CROSS} 'nux' command NOT found in PATH${NC}"
    ERRORS=$((ERRORS+1))
fi

echo ""
if [ $ERRORS -eq 0 ]; then
    echo "    ${GREEN}═══════════════════════════════════════════════════════════════════${NC}"
    echo "    ${GREEN}   ${CHECK} System Health: EXCELLENT${NC}"
    echo "    ${GREEN}═══════════════════════════════════════════════════════════════════${NC}"
else
    echo "    ${RED}═══════════════════════════════════════════════════════════════════${NC}"
    echo "    ${RED}   ${CROSS} System Health: BROKEN ($ERRORS issues found)${NC}"
    echo "       Recommended action: Run ${YELLOW}setup.sh${NC} and select ${YELLOW}Repair${NC}"
    echo "    ${RED}═══════════════════════════════════════════════════════════════════${NC}"
fi
echo ""
