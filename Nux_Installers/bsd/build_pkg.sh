#!/usr/bin/env bash
set -e

# This script packages Nux into a FreeBSD native .pkg installer
# It must be run on a FreeBSD environment.

VERSION="1.0.0"
echo "Building FreeBSD Package..."

mkdir -p build_stage/usr/local/lib/nux
mkdir -p build_stage/usr/local/bin

cat <<EOF > build_stage/+MANIFEST
name: "nux"
version: "${VERSION}"
origin: "lang/nux"
prefix: "/usr/local"
comment: "Nux Programming Language"
desc: "An incredibly fast, lightweight, and extensible programming language."
maintainer: "nuxlang@example.com"
www: "https://github.com/DoguparthiAakash/Nux_Lang"
EOF

# Copy binaries into build_stage/usr/local/lib/nux here

if command -v pkg &> /dev/null; then
    pkg create -M build_stage/+MANIFEST -r build_stage -o .
    echo "Successfully built FreeBSD pkg."
else
    echo "pkg command not found. Run this script on a FreeBSD system."
fi
