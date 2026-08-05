#!/usr/bin/env bash
set -e

# Scaffolds and builds a .deb package for Debian/Ubuntu distributions

VERSION="1.0.0"
ARCH="amd64"
PKG_DIR="nux_${VERSION}_${ARCH}"

echo "Building Debian Package: ${PKG_DIR}.deb"

mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/local/lib/nux"
mkdir -p "${PKG_DIR}/usr/local/bin"

# Create control file
cat <<EOF > "${PKG_DIR}/DEBIAN/control"
Package: nux
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${ARCH}
Maintainer: NuxLang Team
Description: Nux Programming Language
 An incredibly fast, lightweight, and extensible programming language.
EOF

# Note: You should copy the compiled nux binary and standard library into ${PKG_DIR}/usr/local/lib/nux/ here.

if command -v dpkg-deb &> /dev/null; then
    dpkg-deb --build "${PKG_DIR}"
    echo "Successfully built ${PKG_DIR}.deb"
else
    echo "dpkg-deb not found. Run this script on a Debian/Ubuntu system."
fi
