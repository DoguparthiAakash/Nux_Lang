#!/bin/bash
# Nux Linux Installer Script
# Installs Nux and its standard library to /usr/local

set -e

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./install.sh)"
  exit 1
fi

VERSION="0.4.0"
INSTALL_BIN="/usr/local/bin"
INSTALL_LIB="/usr/local/lib/nux"

echo "Installing Nux $VERSION..."

# Ensure we have the built binary
if [ ! -f "../../nux/nux_oleg/nux_portable/target/release/nux" ]; then
    echo "Building Nux..."
    cd ../../nux/nux_oleg/nux_portable
    cargo build --release --locked
    cd ../../../../packages/linux
fi

echo "Copying executable..."
cp ../../nux/nux_oleg/nux_portable/target/release/nux "$INSTALL_BIN/nux"
chmod +x "$INSTALL_BIN/nux"

echo "Copying standard library..."
mkdir -p "$INSTALL_LIB/std"
cp -r ../../nux/nux_oleg/nux-installer-cpp/std/* "$INSTALL_LIB/std/"

echo "Nux has been successfully installed!"
echo "Run 'nux --help' to get started."
