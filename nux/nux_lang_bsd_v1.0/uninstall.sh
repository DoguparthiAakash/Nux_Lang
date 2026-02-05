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
NC='\033[0m'

echo "${RED}Uninstalling Nux Programming Language...${NC}"

if [ "$(id -u)" -ne 0 ]; then
   echo "This script must be run as root" 
   exit 1
fi

rm -rf "$INSTALL_DIR"
rm -rf "$LIB_DIR"
rm -rf "$CONFIG_DIR"
rm -f "$BIN_DIR/nux" "$BIN_DIR/nuxc" "$BIN_DIR/nuxr"

# Remove shell integration if present
if [ -f "$HOME/.profile" ]; then
    # Simple grep removal (more robust solution would use sed/awk but BSD sed varies)
    echo "Warning: Assuming .profile was modified, please check manually."
fi

echo "${GREEN}Successfully uninstalled Nux.${NC}"
