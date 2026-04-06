#!/bin/bash
# Nux Library Deployment Script
# Updates system-wide installation with new libraries

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

# Check root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}Error: This script must be run as root.${NC}" 
   echo "Please run: sudo ./deploy_libs.sh"
   exit 1
fi

LIB_SRC="../lib/std"
LIB_DEST="/usr/local/lib/nux/std"

echo "Deploying new libraries to $LIB_DEST..."

if [ ! -d "$LIB_DEST" ]; then
    echo "Creating directory $LIB_DEST..."
    mkdir -p "$LIB_DEST"
fi

# Copy files
cp "$LIB_SRC"/gui.nux "$LIB_DEST/"
cp "$LIB_SRC"/sql.nux "$LIB_DEST/"
cp "$LIB_SRC"/markdown.nux "$LIB_DEST/"
# Also copy new dependencies like json if we modified/added them, but json was existing.

echo -e "${GREEN}Libraries deployed successfully!${NC}"
echo "Installed: gui.nux, sql.nux, markdown.nux"
