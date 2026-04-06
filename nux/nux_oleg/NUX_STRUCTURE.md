# Nux Project Structure

## 📦 nux_dist/ - Standalone Distribution
**Purpose:** Nux compiler and tools for Linux/macOS/Windows (Standard Rust)

**Contents:**
- `src/compiler.rs`: High-level Nux compiler with Import support and OO features
- `src/vm.rs`: Standalone Nux VM implementation
- `src/main.rs`: CLI (compile/run/asm)

---

## 📚 lib/ - Standard Libraries
**Purpose:** Official Nux libraries written in Nux

### `lib/nux/` (Core Standard Library)
The essential building blocks of the language.
- `io.nux`, `math.nux`, `string.nux`, `collections.nux`
- `memory.nux`, `system.nux`, `thread.nux`
- `vm.nux`, `gc.nux`

### `lib/external/` (Extensions)
Domain-specific libraries for advanced development.
- `ai/` - Machine learning, Neural Networks, Agents
- `game/` - Game engines, graphics, physics
- `os/` - Kernel, drivers, bootloaders
- `web/` - HTTP, servers, cloud
- `data/` - Databases, SQL, Blockchain
- `science/` - Scientific computing, Quantum
- `security/` - Cryptography, Auth
- `gui/` - GUI frameworks
- `tools/` - Compiler tools, CLI frameworks
- `embedded/` - IoT and embedded systems
- `advanced/` - Advanced language theory concepts

---

## 📂 examples/ - Example Scripts
**Purpose:** Sample Nux programs and demos

- `std_demo.nux`: Demonstrates standard library usage
- `class_demo.nux`: Object-oriented programming demo
- `loop.nux`: Loop benchmarks

---

## 🔧 nux_lang/ - Kernel Integration
**Purpose:** Nux language implementation for Ainux kernel

---

## Directory Structure
```
.
├── nux_dist/           # Standalone compiler & VM
├── lib/
│   ├── nux/            # Core Standard Libs
│   │   ├── io.nux
│   │   ├── math.nux
│   │   └── ...
│   └── external/       # Domain-Specific Libs
│       ├── ai/
│       ├── game/
│       ├── os/
│       ├── web/
│       └── ...
├── examples/           # Example Scripts
└── nux_lang/           # Kernel Source
```
