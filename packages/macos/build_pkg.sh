#!/bin/bash
# Nux macOS Installer Builder
# Requires: pkgbuild, productbuild

set -e

VERSION="0.4.0"
OUTPUT_DIR="build/macos"
PAYLOAD_DIR="$OUTPUT_DIR/payload"
INSTALL_BIN_DIR="$PAYLOAD_DIR/usr/local/bin"
INSTALL_LIB_DIR="$PAYLOAD_DIR/usr/local/lib/nux"

echo "Building macOS Installer for Nux $VERSION..."

rm -rf "$OUTPUT_DIR"
mkdir -p "$INSTALL_BIN_DIR"
mkdir -p "$INSTALL_LIB_DIR/std"

# Build Nux binary
cd ../../nux/nux_oleg/nux_portable
cargo build --release --locked
cp target/release/nux "../../packages/macos/$INSTALL_BIN_DIR/"

# Copy standard library
cd ../../..
cp -r nux/nux_oleg/nux-installer-cpp/std/* "packages/macos/$INSTALL_LIB_DIR/std/"

cd packages/macos

echo "Packaging with pkgbuild..."
pkgbuild --root "$PAYLOAD_DIR" \
         --identifier "org.nuxlang.compiler" \
         --version "$VERSION" \
         --install-location "/" \
         "$OUTPUT_DIR/Nux-$VERSION.pkg"

echo "macOS Package built at $OUTPUT_DIR/Nux-$VERSION.pkg"
