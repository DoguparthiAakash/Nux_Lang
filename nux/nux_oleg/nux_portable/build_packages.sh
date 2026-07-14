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
cargo build --release

echo -e "${CYAN}Building Debian package (.deb)...${RESET}"
if ! command -v cargo-deb &> /dev/null; then
    echo "Installing cargo-deb..."
    cargo install cargo-deb
fi
cargo deb
echo -e "${GREEN}✓ Debian package created in target/debian/${RESET}"

echo -e "${CYAN}Building RPM package (.rpm)...${RESET}"
if ! command -v cargo-generate-rpm &> /dev/null; then
    echo "Installing cargo-generate-rpm..."
    cargo install cargo-generate-rpm
fi
cargo generate-rpm
echo -e "${GREEN}✓ RPM package created in target/generate-rpm/${RESET}"

echo -e "${CYAN}Building macOS/Linux Tarballs...${RESET}"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
esac

TAR_NAME="nux-0.4.0-${OS}-${ARCH}.tar.gz"
mkdir -p target/dist
tar -czf "target/dist/${TAR_NAME}" -C target/release nux
echo -e "${GREEN}✓ Tarball created in target/dist/${TAR_NAME}${RESET}"

echo -e "${GREEN}All packages built successfully!${RESET}"
