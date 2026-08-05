#!/usr/bin/env bash
set -e

INSTALL_DIR="/usr/local/lib/nux"
BIN_DIR="/usr/local/bin"

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./uninstall.sh)"
  exit 1
fi

echo "Uninstalling Nux..."
rm -rf "$INSTALL_DIR"
rm -f "$BIN_DIR/nux"
echo "Nux uninstalled successfully."
