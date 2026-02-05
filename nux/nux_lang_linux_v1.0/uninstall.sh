#!/bin/bash
# Nux Uninstall Script

set -e

INSTALL_DIR="/usr/local/nux"
BIN_DIR="/usr/local/bin"
LIB_DIR="/usr/local/lib/nux"
CONFIG_DIR="/etc/nux"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${RED}Uninstalling Nux Programming Language...${NC}"

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

rm -rf "$INSTALL_DIR"
rm -rf "$LIB_DIR"
rm -rf "$CONFIG_DIR"
rm -f "$BIN_DIR/nux" "$BIN_DIR/nuxc" "$BIN_DIR/nuxr"
rm -f /etc/profile.d/nux.sh

echo -e "${GREEN}Successfully uninstalled Nux.${NC}"
