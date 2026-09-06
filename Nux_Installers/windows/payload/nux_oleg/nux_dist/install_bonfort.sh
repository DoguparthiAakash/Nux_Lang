#!/bin/bash
set -e

echo "Installing Bonfort Package Manager globally..."

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
  echo "Please run with sudo: sudo ./install_bonfort.sh"
  exit 1
fi

# Find bonfort binary
BONFORT_BIN=""
if [ -f "target/x86_64-unknown-linux-gnu/debug/bonfort" ]; then
    BONFORT_BIN="target/x86_64-unknown-linux-gnu/debug/bonfort"
elif [ -f "target/debug/bonfort" ]; then
    BONFORT_BIN="target/debug/bonfort"
elif [ -f "target/x86_64-unknown-linux-gnu/release/bonfort" ]; then
    BONFORT_BIN="target/x86_64-unknown-linux-gnu/release/bonfort"
elif [ -f "target/release/bonfort" ]; then
    BONFORT_BIN="target/release/bonfort"
else
    echo "Error: bonfort binary not found. Please build it first with 'cargo build'"
    exit 1
fi

# Find nux binary
NUX_BIN=""
if [ -f "target/x86_64-unknown-linux-gnu/debug/nux" ]; then
    NUX_BIN="target/x86_64-unknown-linux-gnu/debug/nux"
elif [ -f "target/debug/nux" ]; then
    NUX_BIN="target/debug/nux"
elif [ -f "target/x86_64-unknown-linux-gnu/release/nux" ]; then
    NUX_BIN="target/x86_64-unknown-linux-gnu/release/nux"
elif [ -f "target/release/nux" ]; then
    NUX_BIN="target/release/nux"
else
    echo "Error: nux binary not found. Please build it first with 'cargo build'"
    exit 1
fi

echo "Installing bonfort to /usr/local/bin/..."
cp "$BONFORT_BIN" /usr/local/bin/bonfort
chmod +x /usr/local/bin/bonfort

echo "Installing nux to /usr/local/bin/..."
cp "$NUX_BIN" /usr/local/bin/nux
chmod +x /usr/local/bin/nux

# Set up global library path
echo "Setting up global library directory..."
mkdir -p /usr/local/lib/nux
export NUX_LIB=/usr/local/lib/nux

echo ""
echo "✓ Installation complete!"
echo ""
echo "Installed:"
echo "  - bonfort -> /usr/local/bin/bonfort"
echo "  - nux -> /usr/local/bin/nux"
echo "  - library directory -> /usr/local/lib/nux"
echo ""
echo "You can now use 'bonfort' and 'nux' from anywhere!"
echo ""
echo "Try:"
echo "  bonfort --version"
echo "  bonfort init my-project"
echo "  nux --version"
