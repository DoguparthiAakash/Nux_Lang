#!/usr/bin/env sh
# ─────────────────────────────────────────────────────────────
#  Nux Language — Universal Installer
#  Supports: Linux (Debian, Fedora, Arch, Alpine, Gentoo, Void)
#            macOS (Intel & Apple Silicon)
#            BSD   (FreeBSD, OpenBSD, NetBSD, DragonFlyBSD)
#            Legacy systems with old package managers
# ─────────────────────────────────────────────────────────────
set -e

NUX_VERSION="0.4.0"
NUX_REPO="https://github.com/DoguparthiAakash/Nux_Lang"
NUX_RELEASE="$NUX_REPO/releases/download/v$NUX_VERSION"
INSTALL_DIR=""

# ── Colours (safe fallback if term doesn't support) ──────────
if [ -t 1 ]; then
  CYAN='\033[1;36m'; GREEN='\033[1;32m'; RED='\033[1;31m'
  YELLOW='\033[1;33m'; BOLD='\033[1m'; RESET='\033[0m'
else
  CYAN=''; GREEN=''; RED=''; YELLOW=''; BOLD=''; RESET=''
fi

banner() {
  printf "\n${CYAN}  ╔══════════════════════════════════╗${RESET}\n"
  printf "${CYAN}  ║  Nux Language Installer v%s  ║${RESET}\n" "$NUX_VERSION"
  printf "${CYAN}  ╚══════════════════════════════════╝${RESET}\n\n"
}

ok()   { printf "  ${GREEN}✓${RESET}  %s\n" "$*"; }
info() { printf "  ${CYAN}→${RESET}  %s\n" "$*"; }
warn() { printf "  ${YELLOW}⚠${RESET}  %s\n" "$*"; }
die()  { printf "  ${RED}✗${RESET}  %s\n" "$*" >&2; exit 1; }

# ── Detect OS ────────────────────────────────────────────────
detect_os() {
  OS="unknown"
  ARCH=$(uname -m)

  case "$(uname -s)" in
    Linux*)   OS="linux" ;;
    Darwin*)  OS="macos" ;;
    FreeBSD*) OS="freebsd" ;;
    OpenBSD*) OS="openbsd" ;;
    NetBSD*)  OS="netbsd" ;;
    DragonFly*) OS="dragonflybsd" ;;
    *)        die "Unsupported OS: $(uname -s)" ;;
  esac

  case "$ARCH" in
    x86_64 | amd64)   ARCH="x86_64" ;;
    aarch64 | arm64)  ARCH="aarch64" ;;
    armv7*)            ARCH="armv7" ;;
    i386 | i686)       ARCH="x86" ;;
    *)                 ARCH="$ARCH" ;;  # pass through for exotic archs
  esac
}

# ── Detect Package Manager (for native install) ───────────────
detect_pkg_manager() {
  PKG_MANAGER="none"
  if   command -v apt-get  > /dev/null 2>&1; then PKG_MANAGER="apt"
  elif command -v dnf      > /dev/null 2>&1; then PKG_MANAGER="dnf"
  elif command -v yum      > /dev/null 2>&1; then PKG_MANAGER="yum"
  elif command -v pacman   > /dev/null 2>&1; then PKG_MANAGER="pacman"
  elif command -v zypper   > /dev/null 2>&1; then PKG_MANAGER="zypper"
  elif command -v apk      > /dev/null 2>&1; then PKG_MANAGER="apk"
  elif command -v pkg      > /dev/null 2>&1; then PKG_MANAGER="pkg"        # FreeBSD
  elif command -v pkg_add  > /dev/null 2>&1; then PKG_MANAGER="pkg_add"   # OpenBSD/NetBSD legacy
  elif command -v pkgin    > /dev/null 2>&1; then PKG_MANAGER="pkgin"     # NetBSD pkgin
  elif command -v brew     > /dev/null 2>&1; then PKG_MANAGER="brew"      # macOS Homebrew
  elif command -v port     > /dev/null 2>&1; then PKG_MANAGER="macports"  # macOS MacPorts
  fi
}

# ── Decide install directory ──────────────────────────────────
decide_install_dir() {
  if [ "$INSTALL_DIR" != "" ]; then
    return
  fi

  case "$OS" in
    macos)
      if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
      else
        INSTALL_DIR="$HOME/.local/bin"
      fi
      ;;
    freebsd | openbsd | netbsd | dragonflybsd)
      if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
      else
        INSTALL_DIR="$HOME/bin"
      fi
      ;;
    *)
      if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
      else
        INSTALL_DIR="$HOME/.local/bin"
      fi
      ;;
  esac

  mkdir -p "$INSTALL_DIR"
}

# ── Add to PATH if needed ─────────────────────────────────────
ensure_path() {
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) return ;;  # already in PATH
  esac

  SHELL_NAME=$(basename "$SHELL")
  PROFILE=""
  case "$SHELL_NAME" in
    zsh)  PROFILE="$HOME/.zshrc" ;;
    fish) PROFILE="$HOME/.config/fish/config.fish"
          mkdir -p "$(dirname "$PROFILE")" ;;
    *)    PROFILE="$HOME/.bashrc"
          [ -f "$HOME/.profile" ] && PROFILE="$HOME/.profile" ;;
  esac

  if [ -n "$PROFILE" ]; then
    if [ "$SHELL_NAME" = "fish" ]; then
      echo "fish_add_path $INSTALL_DIR" >> "$PROFILE"
    else
      printf '\n# Nux Language\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$PROFILE"
    fi
    warn "Added $INSTALL_DIR to PATH in $PROFILE"
    warn "Run: source $PROFILE  (or open a new terminal)"
  fi
}

# ── Download helper (curl or wget) ───────────────────────────
download() {
  URL="$1"; DEST="$2"
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL --retry 3 "$URL" -o "$DEST"
  elif command -v wget > /dev/null 2>&1; then
    wget -q --tries=3 "$URL" -O "$DEST"
  elif command -v fetch > /dev/null 2>&1; then   # BSD fetch
    fetch -q -o "$DEST" "$URL"
  else
    die "No download tool found. Install curl, wget, or fetch."
  fi
}

# ── Install from native package manager ──────────────────────
install_via_package_manager() {
  info "Detected package manager: $PKG_MANAGER"

  case "$PKG_MANAGER" in
    apt)
      DEB_URL="$NUX_RELEASE/nux_${NUX_VERSION}_amd64.deb"
      TMP_DEB=$(mktemp /tmp/nux.XXXXXX.deb)
      info "Downloading .deb package..."
      download "$DEB_URL" "$TMP_DEB"
      info "Installing .deb (requires sudo)..."
      sudo dpkg -i "$TMP_DEB" || sudo apt-get install -f -y
      rm -f "$TMP_DEB"
      ok "Nux installed via dpkg"
      return 0
      ;;
    dnf | yum)
      RPM_URL="$NUX_RELEASE/nux-${NUX_VERSION}-1.x86_64.rpm"
      TMP_RPM=$(mktemp /tmp/nux.XXXXXX.rpm)
      info "Downloading .rpm package..."
      download "$RPM_URL" "$TMP_RPM"
      info "Installing .rpm (requires sudo)..."
      if command -v dnf > /dev/null 2>&1; then
        sudo dnf install -y "$TMP_RPM"
      else
        sudo yum install -y "$TMP_RPM"
      fi
      rm -f "$TMP_RPM"
      ok "Nux installed via rpm"
      return 0
      ;;
    pacman)
      # Arch doesn't use .deb/.rpm; fall through to tarball install
      info "Arch Linux detected — installing from source tarball..."
      return 1
      ;;
    brew)
      info "Installing via Homebrew..."
      brew install --formula "$NUX_REPO/raw/main/nux.rb"
      ok "Nux installed via brew"
      return 0
      ;;
    pkg)
      # FreeBSD - install from tarball
      info "FreeBSD pkg detected — installing from binary tarball..."
      return 1
      ;;
    pkg_add)
      # OpenBSD/NetBSD legacy
      info "BSD legacy pkg_add detected — installing from binary tarball..."
      return 1
      ;;
  esac

  return 1  # fallback to tarball
}

# ── Install from pre-built binary tarball ────────────────────
install_from_tarball() {
  case "$OS" in
    macos)   TARBALL="nux-${NUX_VERSION}-macos-universal.tar.gz" ;;
    linux)
      if [ "$ARCH" = "aarch64" ]; then
        TARBALL="nux-${NUX_VERSION}-linux-aarch64.tar.gz"
      else
        TARBALL="nux-${NUX_VERSION}-linux-x86_64.tar.gz"
      fi
      ;;
    freebsd | openbsd | netbsd | dragonflybsd)
      # BSD uses x86_64 Linux binary (via Linux compatibility layer on FreeBSD)
      # or we fall back to building from source
      TARBALL="nux-${NUX_VERSION}-linux-x86_64.tar.gz"
      ;;
    *)
      TARBALL="nux-${NUX_VERSION}-linux-x86_64.tar.gz"
      ;;
  esac

  TMP_DIR=$(mktemp -d /tmp/nux_install.XXXXXX)
  TARBALL_PATH="$TMP_DIR/nux.tar.gz"
  TARBALL_URL="$NUX_RELEASE/$TARBALL"

  info "Downloading $TARBALL..."
  download "$TARBALL_URL" "$TARBALL_PATH" || {
    warn "Pre-built binary not available for $OS/$ARCH. Building from source..."
    build_from_source
    return
  }

  info "Extracting..."
  tar xzf "$TARBALL_PATH" -C "$TMP_DIR"

  BINARY=$(find "$TMP_DIR" -name "nux" -type f | head -1)
  if [ -z "$BINARY" ]; then
    warn "Binary not found in tarball. Building from source..."
    rm -rf "$TMP_DIR"
    build_from_source
    return
  fi

  chmod +x "$BINARY"
  cp "$BINARY" "$INSTALL_DIR/nux"
  rm -rf "$TMP_DIR"
  ok "Nux binary installed to $INSTALL_DIR/nux"
}

# ── Build from source using Cargo ────────────────────────────
build_from_source() {
  info "Building Nux from source..."

  if ! command -v cargo > /dev/null 2>&1; then
    warn "Cargo (Rust) not found. Installing Rust via rustup..."
    TMPRUST=$(mktemp /tmp/rustup.XXXXXX.sh)
    download "https://sh.rustup.rs" "$TMPRUST"
    sh "$TMPRUST" -y --default-toolchain stable --profile minimal
    rm -f "$TMPRUST"
    # shellcheck disable=SC1091
    . "$HOME/.cargo/env"
  fi

  # Clone or use current repo
  if [ -d "nux/nux_oleg/nux_portable" ]; then
    BUILD_DIR="nux/nux_oleg/nux_portable"
  else
    CLONE_DIR=$(mktemp -d /tmp/nux_src.XXXXXX)
    info "Cloning Nux source..."
    if command -v git > /dev/null 2>&1; then
      git clone --depth=1 "$NUX_REPO.git" "$CLONE_DIR"
    else
      die "git is required to build from source. Install git and retry."
    fi
    BUILD_DIR="$CLONE_DIR/nux/nux_oleg/nux_portable"
  fi

  info "Compiling (this may take 1-2 minutes)..."
  cd "$BUILD_DIR"
  cargo build --release --locked 2>&1 || cargo build --release

  cp "target/release/nux" "$INSTALL_DIR/nux"
  chmod +x "$INSTALL_DIR/nux"
  ok "Built and installed Nux from source"
}

# ─────────────────────────────────────────────────────────────
#  MAIN
# ─────────────────────────────────────────────────────────────
banner

detect_os
info "OS: $OS  |  Architecture: $ARCH"

detect_pkg_manager
decide_install_dir
info "Install directory: $INSTALL_DIR"

echo ""

# Try native package manager first, then tarball, then source
install_via_package_manager || install_from_tarball

ensure_path

echo ""
printf "  ${GREEN}${BOLD}Nux has been installed!${RESET}\n"
printf "  Run: ${CYAN}nux version${RESET}\n\n"
