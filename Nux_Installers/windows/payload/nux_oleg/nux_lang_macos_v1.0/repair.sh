#!/bin/bash
# Nux Repair Script (macOS)

set -e

INSTALL_DIR="/usr/local/nux"
REPO_URL="https://github.com/Nux-Lang/Nux_Mac.git"
TEMP_DIR=$(mktemp -d)

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
WHITE='\033[1;37m'
NC='\033[0m'
WRENCH="🔧"
ARROW="➜"

clear
echo ""
echo -e "${CYAN}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}    ║               ${WHITE}Nux Installation Repair Tool${CYAN}                         ║${NC}"
echo -e "${CYAN}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [[ $EUID -ne 0 ]]; then
   echo -e "    ${RED}Error: This script must be run as root${NC}" 
   exit 1
fi

echo -e "    ${YELLOW}${WRENCH} Beginning Repair Process...${NC}"
echo ""

echo -e "    ${CYAN}${ARROW}${NC} Downloading fresh files from GitHub..."
if ! command -v git &> /dev/null; then
    echo -e "    ${RED}Error: git is required for repair.${NC}"
    exit 1
fi

# Suppress git output for cleaner UI unless error
# Suppress git output for cleaner UI unless error
if mkdir -p "$TEMP_DIR" && cd "$TEMP_DIR" && \
   git clone --no-checkout --depth 1 --filter=blob:none "$REPO_URL" . >/dev/null 2>&1 && \
   git sparse-checkout init --cone >/dev/null 2>&1 && \
   git sparse-checkout set nux_pack_macos_v1.0 >/dev/null 2>&1 && \
   git checkout >/dev/null 2>&1; then
    echo -e "    ${GREEN}✓${NC} Download complete"
else
    echo -e "    ${RED}✗${NC} Download failed"
    exit 1
fi

echo -e "    ${CYAN}${ARROW}${NC} Restoring core files..."
# Patch the downloaded script to prevent infinite recursion loop
# macOS sed requires empty extension for in-place edit
sed -i '' '2i\
export NUX_INSTALLER_RUNNING=1' "$TEMP_DIR/nux_pack_macos_v1.0/setup.sh"

# Run the installer from the temp dir
bash "$TEMP_DIR/nux_pack_macos_v1.0/setup.sh"

rm -rf "$TEMP_DIR"

echo ""
echo -e "    ${GREEN}✓ Repair complete. Your installation has been restored.${NC}"
echo ""
