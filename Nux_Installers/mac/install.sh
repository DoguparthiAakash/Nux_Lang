#!/usr/bin/env bash
set -e

INSTALL_DIR="/usr/local/lib/nux"
BIN_DIR="/usr/local/bin"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-mac.tar.gz"

echo "===================================="
echo " Nux Programming Language Installer "
echo "===================================="

# Require sudo for /usr/local writes
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./install.sh)"
  exit 1
fi

echo "[1/3] Downloading latest Nux binaries..."
TEMP_TAR=$(mktemp)
curl -L -o "$TEMP_TAR" "$DOWNLOAD_URL" || {
    echo "Warning: Failed to download from $DOWNLOAD_URL. If you have a local build, copy it manually."
}

echo "[2/3] Extracting and installing..."
mkdir -p "$INSTALL_DIR"
if [ -s "$TEMP_TAR" ]; then
    tar -xzf "$TEMP_TAR" -C "$INSTALL_DIR"
fi
rm -f "$TEMP_TAR"

echo "[3/3] Linking executable..."
mkdir -p "$BIN_DIR"
ln -sf "$INSTALL_DIR/nux" "$BIN_DIR/nux"

echo "Installation Complete! You can now use the 'nux' command."
