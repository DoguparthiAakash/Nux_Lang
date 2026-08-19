#!/usr/bin/env sh
# ─────────────────────────────────────────────────────────────
#  Nux — Local Package Builder
#  Run this on a Linux machine (or in WSL) to produce:
#    - nux_*.deb       (Debian/Ubuntu/Mint/Raspberry Pi OS)
#    - nux-*.rpm       (Fedora/RHEL/CentOS/openSUSE)
#    - nux-*.tar.gz    (Arch/Gentoo/Alpine/BSD generic)
#
#  Requirements: gcc, fpm
#  Install tools: gem install fpm
# ─────────────────────────────────────────────────────────────
set -e

NUX_VERSION="0.4.0"
SRC="nux/nux_oleg/nux_portable"
DIST="dist"

CYAN='\033[1;36m'; GREEN='\033[1;32m'; RED='\033[1;31m'; RESET='\033[0m'
ok()   { printf "  ${GREEN}✓${RESET}  %s\n" "$*"; }
info() { printf "  ${CYAN}→${RESET}  %s\n" "$*"; }
die()  { printf "  ${RED}✗${RESET}  %s\n" "$*" >&2; exit 1; }

printf "\n${CYAN}  Nux Package Builder v$NUX_VERSION${RESET}\n\n"

mkdir -p "$DIST"

# ── 1. Release Binary ──────────────────────────────────────────
info "Building Nux release binary..."
cd "$SRC/runtime_c"
gcc -O3 main.c vm.c vision/vision.c -lm -o nux
BINARY="nux"
[ -f "$BINARY" ] || die "Build failed — binary not found."
ok "Binary ready: $BINARY"

# ── 2. .deb and .rpm (Debian/Ubuntu/Fedora/RHEL) ────────────────
if command -v fpm > /dev/null 2>&1; then
  info "Building .deb and .rpm packages..."
  STAGING=$(mktemp -d)
  mkdir -p "$STAGING/usr/bin"
  cp "$BINARY" "$STAGING/usr/bin/nux"
  
  fpm -s dir -t deb -n nux -v "${NUX_VERSION}" -C "$STAGING" -p "../../../../$DIST/nux_${NUX_VERSION}_amd64.deb" usr/bin/nux
  ok "Created: $DIST/nux_${NUX_VERSION}_amd64.deb"
  
  fpm -s dir -t rpm -n nux -v "${NUX_VERSION}" -C "$STAGING" -p "../../../../$DIST/nux-${NUX_VERSION}-1.x86_64.rpm" usr/bin/nux
  ok "Created: $DIST/nux-${NUX_VERSION}-1.x86_64.rpm"
  rm -rf "$STAGING"
else
  info "Skipping .deb and .rpm (fpm not installed)"
  info "  Install: gem install fpm"
fi

cd ../../../../

# ── 4. Generic .tar.gz ────────────────────────────────────────
info "Building generic .tar.gz..."
STAGING=$(mktemp -d)
mkdir -p "$STAGING/usr/bin" "$STAGING/usr/share/doc/nux"
cp "$SRC/runtime_c/nux" "$STAGING/usr/bin/nux"
cp README.md "$STAGING/usr/share/doc/nux/" 2>/dev/null || true
cp install.sh uninstall.sh "$STAGING/" 2>/dev/null || true

# Detect current arch
case "$(uname -m)" in
  aarch64 | arm64) ARCH_LABEL="aarch64" ;;
  x86_64 | amd64)  ARCH_LABEL="x86_64"  ;;
  *)                ARCH_LABEL="$(uname -m)" ;;
esac

TARBALL="$DIST/nux-${NUX_VERSION}-linux-${ARCH_LABEL}.tar.gz"
tar czf "$TARBALL" -C "$STAGING" .
rm -rf "$STAGING"
ok "Created: $TARBALL"

# ── Summary ───────────────────────────────────────────────────
echo ""
printf "  ${GREEN}Packages ready in ./$DIST/${RESET}\n"
ls -lh "$DIST/" 2>/dev/null
echo ""
