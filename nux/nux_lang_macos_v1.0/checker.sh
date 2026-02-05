#!/bin/bash
# Nux Health Checker (macOS)

INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}Checking Nux Installation Health...${NC}"

ERRORS=0

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}OK${NC}: $1 found."
    else
        echo -e "${RED}ALARM${NC}: $1 MISSING!"
        ERRORS=$((ERRORS+1))
    fi
}

check_file "$INSTALL_DIR/bin/nux"
check_file "$BIN_DIR/nux"

if command -v nux &> /dev/null; then
    echo -e "${GREEN}OK${NC}: 'nux' command is in PATH."
    nux --version
else
    echo -e "${RED}ALARM${NC}: 'nux' command NOT found in PATH."
    ERRORS=$((ERRORS+1))
fi

# Check macOS specific Frameworks
if [ -d "$INSTALL_DIR/Frameworks" ]; then
    echo -e "${GREEN}OK${NC}: Frameworks directory exists."
else
    echo -e "${YELLOW}WARN${NC}: Frameworks directory missing."
fi

if [ $ERRORS -eq 0 ]; then
    echo -e "\n${GREEN}System Health: EXCELLENT${NC}"
else
    echo -e "\n${RED}System Health: BROKEN ($ERRORS issues found)${NC}"
    echo "Recommended action: Run Repair."
fi
