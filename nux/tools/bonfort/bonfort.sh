#!/usr/bin/env bash
# bonfort — Nux Package Manager
# Installs this script to /usr/local/bin or ~/.local/bin

set -e

NUX_HOME="${HOME}/.nux"
ENVS_DIR="${NUX_HOME}/envs"
# If a venv is active, use its dirs; else use global
if [ -n "${NUX_VENV:-}" ] && [ -n "${NUX_PKG_PATH:-}" ]; then
    PKG_DIR="${NUX_PKG_PATH}"
    RUNTIME_DIR="${NUX_RUNTIME_PATH:-${NUX_HOME}/runtimes}"
else
    RUNTIME_DIR="${NUX_HOME}/runtimes"
    PKG_DIR="${NUX_HOME}/packages"
fi
CACHE_DIR="${NUX_HOME}/cache"
REGISTRY_URL="https://registry.nux-lang.org"
VERSION="1.0.0"

mkdir -p "${ENVS_DIR}" "${RUNTIME_DIR}" "${PKG_DIR}" "${CACHE_DIR}"

usage() {
    echo "Bonfort Package Manager v${VERSION}"
    echo ""
    echo "Usage: bonfort <command> [args]"
    echo ""
    echo "Package commands:"
    echo "  init [name] [pkg-id]     Create a new Nux project"
    echo "  install <pkg[@ver]>      Install a Nux package"
    echo "  remove  <pkg>            Remove a package"
    echo "  list                     List installed packages"
    echo "  search  <query>          Search the registry"
    echo "  update                   Update all packages"
    echo ""
    echo "Virtual environment (venv) commands:"
    echo "  venv create <name>           Create a new isolated venv"
    echo "  venv activate <name>         Print activation path (use with source)"
    echo "  venv deactivate              Deactivate current venv"
    echo "  venv list                    List all venvs"
    echo "  venv remove <name>           Delete a venv"
    echo "  venv info                    Show active venv details"
    echo "  venv clone <src> <dst>       Clone a venv"
    echo "  venv export <name> [out]     Export venv as .nxenv archive"
    echo "  venv import <file.nxenv>     Import a .nxenv archive"
    echo ""
    echo "Language runtime commands:"
    echo "  lang list                List installed LAG runtimes"
    echo "  lang add <lang[@ver]>    Install a language runtime"
    echo "  lang check <lang>        Check if a language is available"
    echo ""
    echo "Build commands:"
    echo "  build [file.nux]         Compile .nux to .nxb binary"
    echo "  build-all                Compile all .nux files in project"
    echo ""
    echo "Other:"
    echo "  self-update              Update bonfort itself"
    echo "  self-install             Install bonfort to PATH"
}

# ─── OS Package Manager Abstraction ──────────────────────────────────────────
os_install() {
    local pkg_apt="$1"
    local pkg_brew="${2:-$1}"
    local pkg_pkg="${3:-$1}"
    local pkg_choco="${4:-$1}"

    if command -v apt-get >/dev/null 2>&1; then
        echo "bonfort: using apt (Linux/ChromeOS)..."
        if [ -n "$TERMUX_VERSION" ]; then apt-get install -y $pkg_apt
        elif [ "$(id -u)" != "0" ]; then sudo apt-get install -y $pkg_apt
        else apt-get install -y $pkg_apt; fi
        return 0
    elif command -v brew >/dev/null 2>&1; then
        echo "bonfort: using brew (macOS/Linux)..."
        brew install $pkg_brew; return 0
    elif command -v pkg >/dev/null 2>&1; then
        echo "bonfort: using pkg (Android Termux / BSD)..."
        pkg install -y $pkg_pkg; return 0
    elif command -v choco >/dev/null 2>&1; then
        echo "bonfort: using choco (Windows)..."
        choco install -y $pkg_choco; return 0
    elif command -v winget >/dev/null 2>&1; then
        echo "bonfort: using winget (Windows)..."
        winget install -e --id $pkg_choco; return 0
    elif command -v dnf >/dev/null 2>&1; then
        echo "bonfort: using dnf (Fedora/RHEL)..."
        sudo dnf install -y $pkg_apt; return 0
    elif command -v pacman >/dev/null 2>&1; then
        echo "bonfort: using pacman (Arch)..."
        sudo pacman -S --noconfirm $pkg_apt; return 0
    fi
    echo "bonfort: ERROR no package manager found."
    return 1
}

# ─── Lang: resolve runtime path ─────────────────────────────────────────────
lang_resolve() {
    local lang="$1"
    case "$lang" in
        python|python3) which python3 2>/dev/null || which python 2>/dev/null ;;
        rust|rustc)     which rustc 2>/dev/null ;;
        c|cc|gcc)       which gcc 2>/dev/null || which clang 2>/dev/null || which cc 2>/dev/null ;;
        c++|g++|cpp)    which g++ 2>/dev/null || which clang++ 2>/dev/null || which c++ 2>/dev/null ;;
        java)           which java 2>/dev/null ;;
        zig)            which zig 2>/dev/null ;;
        go)             which go 2>/dev/null ;;
        ruby)           which ruby 2>/dev/null ;;
        js|node|nodejs) which node 2>/dev/null ;;
        lua)            which lua 2>/dev/null || which lua5.4 2>/dev/null ;;
        julia)          which julia 2>/dev/null ;;
        r|R)            which Rscript 2>/dev/null ;;
        *)              echo "" ;;
    esac
}

lang_version() {
    local lang="$1"
    local rt
    rt="$(lang_resolve "$lang")"
    [ -z "$rt" ] && echo "not installed" && return
    case "$lang" in
        python|python3) "$rt" --version 2>&1 | head -1 ;;
        rust|rustc)     "$rt" --version 2>&1 | head -1 ;;
        c|cc|gcc)       "$rt" --version 2>&1 | head -1 ;;
        c++|g++|cpp)    "$rt" --version 2>&1 | head -1 ;;
        java)           "$rt" -version 2>&1 | head -1 ;;
        zig)            "$rt" version 2>&1 | head -1 ;;
        go)             "$rt" version 2>&1 | head -1 ;;
        ruby)           "$rt" --version 2>&1 | head -1 ;;
        js|node|nodejs) "$rt" --version 2>&1 | head -1 ;;
        lua)            "$rt" -v 2>&1 | head -1 ;;
        julia)          "$rt" --version 2>&1 | head -1 ;;
        r|R)            "$rt" --version 2>&1 | head -1 ;;
        *)              echo "unknown" ;;
    esac
}

# ─── lang add ────────────────────────────────────────────────────────────────
lang_add() {
    local spec="$1"
    local lang="${spec%@*}"
    local ver="${spec#*@}"
    [ "$ver" = "$spec" ] && ver="latest"

    echo "bonfort: checking for ${lang} runtime..."
    local rt
    rt="$(lang_resolve "$lang")"
    if [ -n "$rt" ]; then
        echo "bonfort: ${lang} already available → $rt"
        return 0
    fi

    echo "bonfort: installing ${lang}@${ver}..."
    case "$lang" in
        python|python3) os_install "python3 python3-pip" "python@3" "python" "python3" ;;
        rust|rustc)
            if command -v rustup >/dev/null 2>&1; then
                rustup toolchain install "${ver:-stable}"
            else
                echo "bonfort: installing rustup..."
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source "${HOME}/.cargo/env"
            fi ;;
        c|cc|gcc)       os_install "gcc" "gcc" "clang" "mingw" ;;
        c++|g++|cpp)    os_install "g++ clang" "gcc" "clang" "mingw" ;;
        java)           os_install "default-jdk" "openjdk" "openjdk-17" "openjdk" ;;
        zig)
            echo "bonfort: downloading zig ${ver}..."
            local zig_ver="${ver:-0.12.0}"
            local platform="linux-x86_64"
            if [ "$(uname)" = "Darwin" ]; then platform="macos-x86_64"; fi
            local zig_url="https://ziglang.org/download/${zig_ver}/zig-${platform}-${zig_ver}.tar.xz"
            local zig_dest="${RUNTIME_DIR}/zig/${zig_ver}"
            mkdir -p "$zig_dest"
            curl -L "$zig_url" | tar -xJ -C "$zig_dest" --strip-components=1
            ln -sf "${zig_dest}/zig" "${HOME}/.local/bin/zig"
            echo "bonfort: zig installed to ${zig_dest}" ;;
        go)             os_install "golang" "go" "go" "go" ;;
        js|node|nodejs) os_install "nodejs npm" "node" "nodejs" "nodejs" ;;
        lua)            os_install "lua5.4" "lua" "lua54" "lua" ;;
        julia)
            echo "bonfort: downloading julia..."
            curl -fsSL https://install.julialang.org | sh ;;
        r|R)            os_install "r-base" "r" "R" "R.Project" ;;
        *)              echo "bonfort: ERROR unknown language: $lang"; return 1 ;;
    esac

    rt="$(lang_resolve "$lang")"
    if [ -n "$rt" ]; then
        echo "bonfort: SUCCESS ${lang} installed → $rt"
    fi
}

lang_list() {
    echo "bonfort: LAG runtime status:"
    local langs=("python" "rust" "c" "c++" "java" "zig" "go" "ruby" "js" "lua" "julia" "r")
    for lang in "${langs[@]}"; do
        local rt
        rt="$(lang_resolve "$lang" 2>/dev/null)" || true
        if [ -n "$rt" ]; then
            printf "  %-8s ✓  %s\n" "$lang" "$(lang_version "$lang" | head -1)"
        else
            printf "  %-8s ✗  not found\n" "$lang"
        fi
    done
}

# ─── VENV ────────────────────────────────────────────────────────────────────
cmd_venv() {
    local sub="${1:-list}"; shift || true
    case "$sub" in
    create)
        local name="${1:-nux-env}"
        local vdir="${ENVS_DIR}/${name}"
        if [ -d "$vdir" ]; then echo "bonfort venv: exists"; return 1; fi
        mkdir -p "${vdir}/packages" "${vdir}/runtimes" "${vdir}/cache" "${vdir}/bin"
        
        # Windows BAT and PS1 Generators
        cat > "${vdir}/activate.bat" << BAT
@echo off
set "NUX_VENV=${name}"
set "NUX_VENV_DIR=${vdir}"
set "NUX_PKG_PATH=${vdir}\packages"
set "NUX_RUNTIME_PATH=${vdir}\runtimes"
set "PATH=${vdir}\bin;%PATH%"
set "PROMPT=(nux:${name}) %PROMPT%"
echo [nux] Activated venv '${name}' (Windows CMD)
BAT
        cat > "${vdir}/activate.ps1" << PS1
\$env:NUX_VENV="${name}"
\$env:NUX_VENV_DIR="${vdir}"
\$env:NUX_PKG_PATH="${vdir}\packages"
\$env:NUX_RUNTIME_PATH="${vdir}\runtimes"
\$env:PATH="${vdir}\bin;" + \$env:PATH
function prompt { "(nux:${name}) " + (Get-Location) + "> " }
Write-Output "[nux] Activated venv '${name}' (PowerShell)"
PS1

        cat > "${vdir}/activate" << ACTIVATE
export NUX_VENV="${name}"
export NUX_VENV_DIR="${vdir}"
export NUX_PKG_PATH="${vdir}/packages"
export NUX_RUNTIME_PATH="${vdir}/runtimes"
export NUX_CACHE_DIR="${vdir}/cache"
export PATH="${vdir}/bin:\$PATH"
_NUX_OLD_PS1="\$PS1"
export PS1="(nux:${name}) \$PS1"
deactivate() {
    unset NUX_VENV NUX_VENV_DIR NUX_PKG_PATH NUX_RUNTIME_PATH NUX_CACHE_DIR
    PATH="\${PATH//\${vdir}\/bin:/}"
    export PATH
    export PS1="\$_NUX_OLD_PS1"
    unset _NUX_OLD_PS1
    unset -f deactivate 2>/dev/null || true
    echo "[nux] Deactivated venv '${name}'"
}
echo "[nux] Activated venv '${name}'"
ACTIVATE
        echo "name=\"${name}\"\ncreated=\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"" > "${vdir}/nux-env.toml"
        echo "<?xml version=\"1.0\"?><nux-imports><project><name>${name}</name></project></nux-imports>" > "${vdir}/imports.xml"
        echo "# Bonfort.lock version=1" > "${vdir}/Bonfort.lock"
        echo "bonfort: venv '${name}' created at ${vdir}"
        ;;
    activate)
        local name="${1:-}"
        if [ -z "$name" ] && [ -n "${NUX_VENV:-}" ]; then echo "${NUX_VENV_DIR}/activate"; exit 0; fi
        local vdir="${ENVS_DIR}/${name}"
        if [ ! -f "${vdir}/activate" ]; then echo "not found"; exit 1; fi
        echo "${vdir}/activate"
        ;;
    deactivate)
        echo "run 'deactivate' or close shell" ;;
    list)
        ls "${ENVS_DIR}" 2>/dev/null || echo "(none)" ;;
    remove)
        rm -rf "${ENVS_DIR}/${1}" ;;
    esac
}

# ─── BUILD ───────────────────────────────────────────────────────────────────
cmd_build() {
    local src="${1:-main.nux}"
    local out="${src%.nux}.nxb"
    if [ -x "./nuxc" ]; then
        ./nuxc compile "$src" -o "$out"
    elif command -v zstd >/dev/null 2>&1; then
        zstd -19 -q "$src" -o "$out"
        echo "bonfort: $src -> $out (zstd)"
    else
        gzip -9 -c "$src" > "$out"
    fi
}

cmd_init() {
    local name="${1:-my-project}"
    mkdir -p src
    echo "[package]\nname=\"${name}\"\nimports=\"imports.xml\"" > Bonfort.toml
    echo "<?xml version=\"1.0\"?><nux-imports></nux-imports>" > imports.xml
    echo "func main() { println(\"Hello from ${name}\"); }\nmain();" > src/main.nux
    echo "bonfort: project ready!"
}

CMD="${1:-}"
shift || true
case "$CMD" in
    ""|help) usage ;;
    init) cmd_init "$@" ;;
    build) cmd_build "$@" ;;
    venv) cmd_venv "$@" ;;
    lang)
        SUB="${1:-list}"; shift || true
        case "$SUB" in
            list) lang_list ;;
            add) lang_add "$@" ;;
        esac ;;
esac
