# Nux Distribution Structure

This directory contains two separate Nux implementations:

## 📦 nux_dist/ - Standalone Distribution
**Purpose:** Nux compiler and tools for Linux/macOS/Windows

**Contents:**
- Standalone Nux compiler (no kernel dependencies)
- Library API for embedding Nux in other projects
- CLI tool for compiling Nux scripts
- Minimal VM stub (for testing only)

**Usage:**
```bash
cd nux_dist
cargo build --release
./target/release/nux script.nux
```

**Target:** Developers who want to use Nux outside the kernel

---

## 🔧 nux_lang/ - Kernel Integration
**Purpose:** Nux language implementation for Ainux kernel

**Contents:**
- Full Nux compiler with kernel integration
- Complete VM with kernel APIs
- Security & Data Manager intrinsics
- File I/O, graphics, and system operations

**Usage:**
These files are integrated into the kernel at `src/nux/`

**Target:** Ainux kernel development

---

## Directory Structure

```
.
├── nux_dist/           # Standalone distribution
│   ├── src/
│   │   ├── lib.rs      # Public API
│   │   ├── main.rs     # CLI binary
│   │   ├── lexer.rs    # Tokenizer
│   │   ├── compiler.rs # High-level compiler
│   │   ├── assembler.rs# Bytecode assembler
│   │   └── vm.rs       # Minimal VM stub
│   ├── Cargo.toml
│   └── README.md
│
└── nux_lang/           # Kernel integration
    ├── lexer.rs        # Tokenizer (kernel version)
    ├── high_level.rs   # Compiler (kernel version)
    ├── compiler.rs     # Assembler (kernel version)
    ├── vm.rs           # Full VM with kernel APIs
    ├── mod.rs          # Module declarations
    └── README.md
```

---

## Key Differences

| Feature | nux_dist | nux_lang |
|---------|----------|----------|
| **Environment** | std Rust (Linux/macOS/Windows) | no_std (Ainux kernel) |
| **VM** | Stub only | Full implementation |
| **Kernel APIs** | None | VFS, drivers, security, etc. |
| **File I/O** | Disabled | Full VFS integration |
| **Graphics** | None | Kernel video driver |
| **Security** | None | UserManager integration |
| **Data Manager** | None | DataManager integration |
| **Use Case** | Compiler testing, development | Production kernel use |

---

## Development Workflow

1. **Test compiler changes** in `nux_dist/` (faster iteration)
2. **Port fixes** to `nux_lang/` for kernel integration
3. **Verify** in kernel with QEMU

---

## Building

### Standalone Distribution
```bash
cd nux_dist
cargo build --release
```

### Kernel Integration
```bash
# Automatically built with kernel
./build_iso.sh
```
