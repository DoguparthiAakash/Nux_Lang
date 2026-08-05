import os
from pathlib import Path

BASE_DIR = Path(r"e:\nux\Nux_Lang\Nux_Installers")
PLATFORMS = {
    "windows": "Windows",
    "mac": "macOS",
    "linux": "Linux",
    "bsd": "BSD"
}

README_TEMPLATE = """# Nux Programming Language ({platform})

Welcome to the Nux Programming Language distribution for {platform}!

## What is Nux?
Nux is an incredibly fast, lightweight, and extensible programming language. It is designed to run everywhere while remaining powerful enough for advanced ML, 3D engines, Quantum computing interfaces, and OS-level interactions.

## Contents
{contents}

## Quick Start
Check `SetupGuide.md` for installation instructions and `UserGuide.md` on how to start writing your first Nux program!
"""

SETUP_TEMPLATE = """# Nux Setup Guide ({platform})

This guide explains how to install and update Nux on {platform}.

## Installation

{install_steps}

## Updating Nux

Run the included update script to fetch the latest versions without reinstalling manually.
{update_steps}

## Uninstallation

If you need to remove Nux:
{uninstall_steps}
"""

USER_TEMPLATE = """# Nux User Guide

Welcome to Nux! Because Nux is designed as a "Write Once, Run Anywhere" language, the syntax and core concepts are exactly the same whether you are on Windows, Mac, Linux, or BSD.

## Writing Your First Program

Create a file called `hello.nux`:

```nux
func main() {
    println("Hello, World from Nux!");
}
```

## Running Nux

To execute a `.nux` file, use the Nux compiler/runner from your terminal:

```sh
nux run hello.nux
```

## Compiling to Executable

To compile your code into a standalone binary:

```sh
nux build hello.nux
```

## Core Libraries

Nux comes with a powerful standard library. Simply import what you need:

```nux
import "std/math";
import "std/io";

func main() {
    var x = math_sqrt(16.0);
    println(x);
}
```

Enjoy building with Nux!
"""

CONTENTS = {
    "windows": "- **nux-setup.exe**: The Windows Installer (NSIS-based).\n- **install.ps1**: PowerShell automated installer.\n- **update.ps1**: Update manager for Windows.\n- **uninstall.ps1**: Completely cleans Nux from your system.\n- **nux_source_only.zip**: The standard library and source code for manual installs.",
    "mac": "- **install.sh**: Shell automated installer.\n- **build_pkg.sh**: Native macOS PKG builder.\n- **update.sh**: Update manager.\n- **uninstall.sh**: Uninstaller.\n- **nux_source_only.tar.gz**: The standard library and source code.",
    "linux": "- **install.sh**: Shell automated installer.\n- **build_deb.sh / build_rpm.sh**: Builders for Debian and RHEL packages.\n- **update.sh**: Update manager.\n- **uninstall.sh**: Uninstaller.\n- **nux_source_only.tar.gz**: The standard library and source code.",
    "bsd": "- **install.sh**: Shell automated installer.\n- **build_pkg.sh**: FreeBSD package builder.\n- **update.sh**: Update manager.\n- **uninstall.sh**: Uninstaller.\n- **nux_source_only.tar.gz**: The standard library and source code."
}

INSTALL_STEPS = {
    "windows": "1. Run `nux-setup.exe` and follow the prompts.\n2. Alternatively, right-click `install.ps1` and select 'Run with PowerShell'.\n3. The installer will extract files and add `nux` to your System PATH.",
    "mac": "1. Open your terminal.\n2. Run `./install.sh`.\n3. The installer will copy `nux` to `/usr/local/bin` and libraries to `/usr/local/lib/nux`.",
    "linux": "1. Open your terminal.\n2. Run `./install.sh` for a generic installation.\n3. (Optional) Run `./build_deb.sh` or `./build_rpm.sh` to create a native package for your package manager, then install it via `dpkg -i nux.deb` or `rpm -i nux.rpm`.",
    "bsd": "1. Open your terminal.\n2. Run `./install.sh` for a generic installation.\n3. (Optional) Run `./build_pkg.sh` to generate a native FreeBSD package, then install it using `pkg install`."
}

UPDATE_STEPS = {
    "windows": "```powershell\n.\\update.ps1\n```",
    "mac": "```sh\n./update.sh\n```",
    "linux": "```sh\n./update.sh\n```",
    "bsd": "```sh\n./update.sh\n```"
}

UNINSTALL_STEPS = {
    "windows": "```powershell\n.\\uninstall.ps1\n```\nOr use the Windows 'Add or Remove Programs' menu.",
    "mac": "```sh\n./uninstall.sh\n```",
    "linux": "```sh\n./uninstall.sh\n```",
    "bsd": "```sh\n./uninstall.sh\n```"
}

def generate_docs():
    for plat, plat_name in PLATFORMS.items():
        plat_dir = BASE_DIR / plat
        plat_dir.mkdir(parents=True, exist_ok=True)
        
        # README
        with open(plat_dir / "README.md", "w") as f:
            f.write(README_TEMPLATE.format(platform=plat_name, contents=CONTENTS[plat]))
            
        # Setup Guide
        with open(plat_dir / "SetupGuide.md", "w") as f:
            f.write(SETUP_TEMPLATE.format(
                platform=plat_name,
                install_steps=INSTALL_STEPS[plat],
                update_steps=UPDATE_STEPS[plat],
                uninstall_steps=UNINSTALL_STEPS[plat]
            ))
            
        # User Guide
        with open(plat_dir / "UserGuide.md", "w") as f:
            f.write(USER_TEMPLATE)

    print("Documentation generated successfully.")

if __name__ == "__main__":
    generate_docs()
