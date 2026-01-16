#!/bin/bash
set -e

if [ "$EUID" -ne 0 ]; then 
  echo "Please run as root"
  exit 1
fi

echo "Installing Nux Engine..."

# 1. Install Binary
echo "Installing binary to /usr/local/bin/nux..."
cp bin/nux /usr/local/bin/nux
chmod +x /usr/local/bin/nux

# 2. Install Libraries
LIB_DIR="/usr/local/lib/nux"
echo "Installing libraries to $LIB_DIR..."
mkdir -p "$LIB_DIR"
cp lib/*.nux "$LIB_DIR/"

# 3. Message
echo "Installation Complete!"
echo "You can now run 'nux' from anywhere."
echo "Libraries in $LIB_DIR will be automatically found."
