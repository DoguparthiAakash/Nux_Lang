# Nux Programming Language - macOS Distribution

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
**Platform:** macOS (Intel + Apple Silicon)

## Quick Start

Install Nux with a single command:

```bash
sudo ./setup.sh
```

## System Requirements

- **OS:** macOS 11.0 (Big Sur) or later
- **Xcode:** Command Line Tools required
- **Disk Space:** ~100 MB
- **RAM:** 512 MB minimum

## Installation

### 1. Extract the Package

```bash
cd nux_pack_macos
```

### 2. Install Xcode Command Line Tools (if needed)

```bash
xcode-select --install
```

### 3. Run the Installer

```bash
sudo ./setup.sh
```

The installer will:
- ✓ Verify Xcode Command Line Tools
- ✓ Create installation directories
- ✓ Install universal binaries (Intel + Apple Silicon)
- ✓ Configure shell profile (zsh/bash)
- ✓ Register .nux file type

### 4. Verify Installation

Open a new Terminal and run:

```bash
nux --version
```

You should see: `Nux v1.0.0 (macOS)`

## Getting Started

### Start the REPL

```bash
nux repl
```

### Run Example Programs

```bash
# Hello World
nux examples/hello.nux

# Web Server
nux examples/web_server.nux

# AI Demo
nux examples/ai_demo.nux
```

### Create Your First Program

```bash
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
  - `nux` - Runtime and REPL (Universal Binary)
  - `nuxc` - Compiler
  - `nuxr` - Script runner

## macOS-Specific Features

- **Universal Binary:** Runs natively on both Intel and Apple Silicon
- **File Type Registration:** Double-click .nux files to run
- **Grand Central Dispatch:** Optimized threading for macOS
- **Metal Integration:** GPU acceleration support

## Troubleshooting

### Permission Denied

Make sure to run the installer with `sudo`:
```bash
sudo ./setup.sh
```

### Command Not Found

Restart Terminal or manually source your profile:
```bash
# For zsh (default on macOS)
source ~/.zshrc

# For bash
source ~/.bash_profile
```

### Xcode Tools Missing

Install Command Line Tools:
```bash
xcode-select --install
```

### Homebrew Conflicts

If you have Homebrew installed, ensure `/usr/local/bin` is in your PATH before Homebrew paths.

## Uninstallation

```bash
sudo ./setup.sh uninstall
```

## Documentation

- **Language Guide:** https://nux-lang.org/docs
- **API Reference:** https://nux-lang.org/api
- **Examples:** https://github.com/nux-lang/examples

## Support

- **Issues:** https://github.com/nux-lang/nux/issues
- **Community:** https://discord.gg/nux-lang
- **Email:** support@nux-lang.org

## License

Nux Programming Language is released under the MIT License.  
See LICENSE file for details.

---

**Happy Coding!** 🍎🚀
