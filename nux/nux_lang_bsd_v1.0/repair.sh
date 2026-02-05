#!/bin/sh
# Nux Repair Script (BSD)

set -e

INSTALL_DIR="/usr/local/nux"
REPO_URL="https://github.com/Nux-Lang/Nux_BSD.git"
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
echo "${CYAN}    ╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo "${CYAN}    ║               ${WHITE}Nux Installation Repair Tool${CYAN}                            ║${NC}"
echo "${CYAN}    ╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ "$(id -u)" -ne 0 ]; then
   echo "    ${RED}Error: This script must be run as root${NC}" 
   exit 1
fi

echo "    ${YELLOW}${WRENCH} Beginning Repair Process...${NC}"
echo ""

echo "    ${CYAN}${ARROW}${NC} Downloading fresh files from GitHub..."
if ! command -v git >/dev/null 2>&1; then
    echo "    ${RED}Error: git is required for repair.${NC}"
    exit 1
fi

# Suppress git output for cleaner UI unless error
if git clone --depth 1 "$REPO_URL" "$TEMP_DIR" >/dev/null 2>&1; then
    echo "    ${GREEN}✓${NC} Download complete"
else
    echo "    ${RED}✗${NC} Download failed"
    exit 1
fi

echo "    ${CYAN}${ARROW}${NC} Restoring core files..."
# Patch the downloaded script to prevent infinite recursion loop
sed -i '' '2i\
export NUX_INSTALLER_RUNNING=1' "$TEMP_DIR/nux_pack_bsd_v1.0/setup.sh"

# Run the installer from the temp dir
sh "$TEMP_DIR/nux_pack_bsd_v1.0/setup.sh"

rm -rf "$TEMP_DIR"

echo ""
echo "    ${GREEN}✓ Repair complete. Your installation has been restored.${NC}"
echo ""
