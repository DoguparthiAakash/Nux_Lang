# Nux Programming Language - BSD Distribution

```
████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗
████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝
████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝ 
████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝  
██████████████████████       ██║ ╚██╗██║██║    ██║   ███║   
██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗  
         ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗ 
         ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗
█████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║
█████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝
```

**Version:** 1.0.0  
**Platform:** BSD (FreeBSD, OpenBSD, NetBSD, DragonFly BSD)

## Quick Start

Install Nux with a single command:

```sh
doas ./setup.sh
# or
sudo ./setup.sh
```

## System Requirements

- **OS:** FreeBSD 13.0+, OpenBSD 7.0+, NetBSD 9.0+, or DragonFly BSD 6.0+
- **Dependencies:** gcc, make, git (auto-installed if missing)
- **Disk Space:** ~100 MB
- **RAM:** 512 MB minimum

## Installation

### 1. Extract the Package

```sh
cd nux_pack_bsd
```

### 2. Run the Installer

```sh
# FreeBSD/NetBSD
sudo ./setup.sh

# OpenBSD (use doas)
doas ./setup.sh
```

The installer will:
- ✓ Detect BSD variant (FreeBSD/OpenBSD/NetBSD/DragonFly)
- ✓ Check system dependencies
- ✓ Install missing packages via pkg/pkg_add/pkgin
- ✓ Create installation directories
- ✓ Install Nux runtime and libraries
- ✓ Configure environment variables

### 3. Verify Installation

Open a new terminal and run:

```sh
nux --version
```

You should see: `Nux v1.0.0 (BSD)`

## Getting Started

### Start the REPL

```sh
nux repl
```

### Run Example Programs

```sh
# Hello World
nux examples/hello.nux

# Web Server
nux examples/web_server.nux

# AI Demo
nux examples/ai_demo.nux
```

### Create Your First Program

```sh
echo 'import "std.io"; println("Hello, Nux!");' > hello.nux
nux hello.nux
```

## What's Included

- **Standard Libraries** (79 files)
  - `lib/std/` - Core utilities, I/O, networking, graphics
  - `lib/ai/` - Neural networks, transformers, GANs, RL
  - `lib/os/` - Kernel, scheduler, memory management
  - `lib/embedded/` - Hardware control, GPIO, sensors

- **Examples**
  - `hello.nux` - Basic syntax and I/O
  - `web_server.nux` - HTTP server
  - `ai_demo.nux` - Neural network training

- **Tools**
  - `nux` - Runtime and REPL
  - `nuxc` - Compiler
  - `nuxr` - Script runner

## BSD-Specific Features

- **Multi-BSD Support:** Automatically detects FreeBSD, OpenBSD, NetBSD, DragonFly BSD
- **Package Manager Integration:** Uses pkg (FreeBSD), pkg_add (OpenBSD), or pkgin (NetBSD)
- **Standard Paths:** Follows BSD filesystem hierarchy (/usr/local)
- **Shell Integration:** Works with sh, bash, zsh, ksh

## Troubleshooting

### Permission Denied

Make sure to run the installer with appropriate privileges:

```sh
# FreeBSD/NetBSD
sudo ./setup.sh

# OpenBSD
doas ./setup.sh
```

### Command Not Found

Restart your terminal or manually source your profile:

```sh
source ~/.profile
# or for bash
source ~/.bashrc
# or for zsh
source ~/.zshrc
```

### Missing Dependencies

The installer auto-installs gcc, make, and git. If you encounter issues:

**FreeBSD:**
```sh
sudo pkg install gcc make git
```

**OpenBSD:**
```sh
doas pkg_add gcc make git
```

**NetBSD:**
```sh
sudo pkgin install gcc make git
```

### Package Manager Not Found

Ensure your BSD system has the appropriate package manager installed:
- FreeBSD: `pkg` (should be pre-installed)
- OpenBSD: `pkg_add` (built-in)
- NetBSD: `pkgin` (install via: `pkg_add pkgin`)

## Uninstallation

```sh
sudo ./setup.sh uninstall
# or on OpenBSD
doas ./setup.sh uninstall
```

## Platform-Specific Notes

### FreeBSD
- Uses `pkg` package manager
- Installs to `/usr/local/nux`
- Compatible with FreeBSD 13.0 and later

### OpenBSD
- Uses `pkg_add` package manager
- Requires `doas` for privilege escalation
- Compatible with OpenBSD 7.0 and later

### NetBSD
- Uses `pkgin` package manager (install if needed)
- Compatible with NetBSD 9.0 and later

### DragonFly BSD
- Uses `pkg` package manager
- Compatible with DragonFly BSD 6.0 and later

## Documentation

- **Language Guide:** https://nux-lang.org/docs
- **API Reference:** https://nux-lang.org/api
- **Examples:** https://github.com/nux-lang/examples
- **BSD Notes:** https://nux-lang.org/docs/bsd

## Support

- **Issues:** https://github.com/nux-lang/nux/issues
- **Community:** https://discord.gg/nux-lang
- **Email:** support@nux-lang.org
- **BSD Forum:** https://forums.nux-lang.org/bsd

## License

Nux Programming Language is released under the MIT License.  
See LICENSE file for details.

---

**Happy Coding on BSD!** 🚀🐡
