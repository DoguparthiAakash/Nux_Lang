#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────
#  Nux Language — Package Builder (.deb, .rpm, mac)
# ─────────────────────────────────────────────────────────────
set -e

CYAN='\033[1;36m'
GREEN='\033[1;32m'
RED='\033[1;31m'
RESET='\033[0m'

echo -e "${CYAN}Building Nux release binary...${RESET}"
cd runtime_c
gcc -O3 main.c vm.c vision/vision.c -lm -o nux
cd ..

echo -e "${CYAN}Building Debian and RPM packages...${RESET}"
if command -v fpm &> /dev/null; then
    mkdir -p target/pkg/usr/bin
    cp runtime_c/nux target/pkg/usr/bin/nux
    
    mkdir -p target/debian target/generate-rpm
    
    fpm -s dir -t deb -n nux -v "0.4.0" -C target/pkg -p target/debian/nux_0.4.0_amd64.deb usr/bin/nux
    echo -e "${GREEN}✓ Debian package created in target/debian/${RESET}"
    
    fpm -s dir -t rpm -n nux -v "0.4.0" -C target/pkg -p target/generate-rpm/nux-0.4.0-1.x86_64.rpm usr/bin/nux
    echo -e "${GREEN}✓ RPM package created in target/generate-rpm/${RESET}"
else
    echo -e "${RED}Skipping .deb and .rpm (fpm not installed). Install with: gem install fpm${RESET}"
fi

echo -e "${CYAN}Building macOS/Linux Tarballs...${RESET}"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
esac

TAR_NAME="nux-0.4.0-${OS}-${ARCH}.tar.gz"
mkdir -p target/dist
tar -czf "target/dist/${TAR_NAME}" -C runtime_c nux
echo -e "${GREEN}✓ Tarball created in target/dist/${TAR_NAME}${RESET}"

echo -e "${GREEN}All packages built successfully!${RESET}"
