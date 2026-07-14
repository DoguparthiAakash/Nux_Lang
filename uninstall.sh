#!/usr/bin/env sh
# ─────────────────────────────────────────────────────────────
#  Nux Language — Universal Uninstaller
# ─────────────────────────────────────────────────────────────
set -e

# ── Colours ──────────────────────────────────────────────────
if [ -t 1 ]; then
  CYAN='\033[1;36m'; GREEN='\033[1;32m'; RED='\033[1;31m'
  YELLOW='\033[1;33m'; BOLD='\033[1m'; RESET='\033[0m'
else
  CYAN=''; GREEN=''; RED=''; YELLOW=''; BOLD=''; RESET=''
fi

ok()   { printf "  ${GREEN}✓${RESET}  %s\n" "$*"; }
info() { printf "  ${CYAN}→${RESET}  %s\n" "$*"; }
warn() { printf "  ${YELLOW}⚠${RESET}  %s\n" "$*"; }
die()  { printf "  ${RED}✗${RESET}  %s\n" "$*" >&2; exit 1; }

printf "\n${CYAN}  ╔══════════════════════════════════╗${RESET}\n"
printf "${CYAN}  ║  Nux Language Uninstaller        ║${RESET}\n"
printf "${CYAN}  ╚══════════════════════════════════╝${RESET}\n\n"

# Remove ~/.nuxenv
if [ -d "$HOME/.nuxenv" ]; then
  info "Removing virtual environments directory (~/.nuxenv)..."
  rm -rf "$HOME/.nuxenv"
  ok "Removed ~/.nuxenv"
fi

# Remove ~/.nux
if [ -d "$HOME/.nux" ]; then
  info "Removing standard libraries and configurations (~/.nux)..."
  rm -rf "$HOME/.nux"
  ok "Removed ~/.nux"
fi

# Find and remove nux executable
NUX_PATH=$(command -v nux || true)
if [ -n "$NUX_PATH" ]; then
  info "Found nux binary at $NUX_PATH. Removing..."
  if [ -w "$NUX_PATH" ]; then
    rm -f "$NUX_PATH"
    ok "Removed $NUX_PATH"
  else
    warn "Need elevated permissions to remove $NUX_PATH"
    if command -v sudo > /dev/null 2>&1; then
      sudo rm -f "$NUX_PATH"
      ok "Removed $NUX_PATH via sudo"
    else
      warn "Please manually delete $NUX_PATH"
    fi
  fi
else
  info "Nux binary not found in PATH."
fi

echo ""
printf "  ${GREEN}${BOLD}Nux has been successfully uninstalled from your system!${RESET}\n\n"
