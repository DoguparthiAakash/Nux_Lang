#!/bin/bash
set -e

# Define Source and Target
SOURCE="./target/release/nux"
TARGET_DIR="$HOME/.local/bin"
TARGET_BIN="$TARGET_DIR/nux"

# Ensure target directory exists
if [ ! -d "$TARGET_DIR" ]; then
    echo "Creating directory: $TARGET_DIR"
    mkdir -p "$TARGET_DIR"
fi

# Build release if not present
if [ ! -f "$SOURCE" ]; then
    echo "Building Nux..."
    cargo build --release
fi

# Copy binary
echo "Installing Nux to $TARGET_BIN..."
cp "$SOURCE" "$TARGET_BIN"
chmod +x "$TARGET_BIN"

# Check PATH
if [[ ":$PATH:" == *":$TARGET_DIR:"* ]]; then
    echo "Success! You can now use 'nux' from anywhere."
else
    echo "Success! But you need to add $TARGET_DIR to your PATH."
    echo "Run this command (or add to ~/.bashrc):"
    echo "export PATH=\"\$PATH:$TARGET_DIR\""
fi
