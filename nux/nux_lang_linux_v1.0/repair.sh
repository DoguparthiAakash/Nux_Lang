#!/bin/bash
# Nux Repair Script

set -e

INSTALL_DIR="/usr/local/nux"
REPO_URL="https://github.com/Nux-Lang/Nux_Linux.git"
TEMP_DIR=$(mktemp -d)

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}Repairing Nux Installation...${NC}"

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

echo "Downloading fresh files from GitHub..."
if ! command -v git &> /dev/null; then
    echo -e "${RED}Error: git is required for repair.${NC}"
    exit 1
fi

git clone --depth 1 "$REPO_URL" "$TEMP_DIR"

echo "Restoring core files..."
# Run the installer from the temp dir to overwrite/fix files
bash "$TEMP_DIR/nux_pack_linux_v1.0/setup.sh"

rm -rf "$TEMP_DIR"

echo -e "${GREEN}Repair complete. Files have been restored.${NC}"
