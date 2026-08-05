#!/usr/bin/env bash
set -e

INSTALL_DIR="/usr/local/lib/nux"
DOWNLOAD_URL="https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-mac.tar.gz"

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./update.sh)"
  exit 1
fi

echo "Updating Nux..."
TEMP_TAR=$(mktemp)
curl -L -o "$TEMP_TAR" "$DOWNLOAD_URL" || {
    echo "Failed to download update."
    exit 1
}

tar -xzf "$TEMP_TAR" -C "$INSTALL_DIR"
rm -f "$TEMP_TAR"

echo "Update successful!"
