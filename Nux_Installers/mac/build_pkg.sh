#!/usr/bin/env bash
set -e

# This script packages Nux into a macOS native .pkg installer
# It must be run on a macOS environment.

VERSION="1.0.0"
IDENTIFIER="com.nuxlang.pkg"
PAYLOAD_DIR="payload"
OUTPUT_PKG="nux-${VERSION}-mac.pkg"

echo "Building macOS Package: $OUTPUT_PKG"

# 1. Prepare Payload Directory (this matches /)
rm -rf "$PAYLOAD_DIR"
mkdir -p "$PAYLOAD_DIR/usr/local/lib/nux"
mkdir -p "$PAYLOAD_DIR/usr/local/bin"

# 2. Copy binaries and libraries into payload
# Note: Ensure nux binary and lib folder are copied here before running pkgbuild
# cp -r ../release_archives/* "$PAYLOAD_DIR/usr/local/lib/nux/"
# ln -sf /usr/local/lib/nux/nux "$PAYLOAD_DIR/usr/local/bin/nux"

# 3. Build PKG
if command -v pkgbuild &> /dev/null; then
    pkgbuild --root "$PAYLOAD_DIR" --identifier "$IDENTIFIER" --version "$VERSION" "$OUTPUT_PKG"
    echo "Successfully built $OUTPUT_PKG"
else
    echo "pkgbuild not found. This script must be run on macOS."
    exit 1
fi
