#!/usr/bin/env sh
# ─────────────────────────────────────────────────────────────
#  Nux Language — Universal Uninstaller
#  Supports: Linux, macOS, FreeBSD, OpenBSD, NetBSD
# ─────────────────────────────────────────────────────────────
set -e

if [ -t 1 ]; then
  GREEN='\033[1;32m'; RED='\033[1;31m'; CYAN='\033[1;36m'; RESET='\033[0m'
else
  GREEN=''; RED=''; CYAN=''; RESET=''
fi

ok()   { printf "  ${GREEN}✓${RESET}  %s\n" "$*"; }
info() { printf "  ${CYAN}→${RESET}  %s\n" "$*"; }
die()  { printf "  ${RED}✗${RESET}  %s\n" "$*" >&2; exit 1; }

printf "\n${CYAN}  Nux Language Uninstaller${RESET}\n\n"

# Find and remove binary
FOUND=0
for BIN_DIR in /usr/local/bin /usr/bin "$HOME/.local/bin" "$HOME/bin"; do
  if [ -f "$BIN_DIR/nux" ]; then
    info "Removing $BIN_DIR/nux ..."
    rm -f "$BIN_DIR/nux"
    ok "Removed $BIN_DIR/nux"
    FOUND=1
  fi
done

if [ "$FOUND" -eq 0 ]; then
  info "Nux binary not found in standard locations."
fi

# Remove PATH export lines added by installer
for PROFILE in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile" "$HOME/.config/fish/config.fish"; do
  if [ -f "$PROFILE" ]; then
    if grep -q "# Nux Language" "$PROFILE" 2>/dev/null; then
      # Use portable sed (works on BSD and GNU)
      sed -i.bak '/# Nux Language/,+1d' "$PROFILE" 2>/dev/null || \
      sed -i '' '/# Nux Language/,+1d' "$PROFILE" 2>/dev/null || true
      rm -f "${PROFILE}.bak"
      ok "Cleaned PATH entry from $PROFILE"
    fi
  fi
done

printf "\n  ${GREEN}Nux has been uninstalled.${RESET}\n\n"
