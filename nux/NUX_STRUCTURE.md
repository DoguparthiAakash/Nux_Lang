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

### `lib/std/` (Standard)

**Core Libraries:**
- `io.nux`: Input/Output wrappers
- `math.nux`: Mathematical functions
- `string.nux`: String manipulation
- `file.nux`: File System operations
- `graphics.nux`: Image & Vision wrappers

**Object-Oriented Libraries:**
- `oo_file.nux`: OO File I/O (File class)
- `oo_graphics.nux`: OO Graphics (Image class)
- `random.nux`: Random number generator (Random class)

**Data & Collections:**
- `collections.nux`: List and Map classes
- `json.nux`: JSON parsing and serialization
- `datetime.nux`: Date and time utilities (DateTime class)

**Networking & System:**
- `network.nux`: Socket operations and HTTP
- `system.nux`: OS interaction, environment, processes
- `crypto.nux`: Hashing, encryption, Base64
- `regex.nux`: Regular expressions (Regex class)

### `lib/embedded/` (Embedded/IoT)
- `gpio.nux`: GPIO control
- `time.nux`: Timing functions
- `analog.nux`: Analog I/O

---

## 📂 examples/ - Example Scripts
**Purpose:** Sample Nux programs and demos

- `std_demo.nux`: Demonstrates standard library usage
- `class_demo.nux`: Object-oriented programming demo
- `loop.nux`: Loop benchmarks
- `print.nux`: Basic printing tests
- `mem_limit.nux`: Memory management tests
- `sec.nux`: Security features demo

---

## 🔧 nux_lang/ - Kernel Integration
**Purpose:** Nux language implementation for Ainux kernel

**Target:** Ainux kernel development

---

## Directory Structure
```
.
├── nux_dist/           # Standalone compiler & VM
├── lib/
│   ├── std/            # Standard Libs
│   │   ├── io.nux
│   │   ├── math.nux
│   │   ├── string.nux
│   │   ├── file.nux
│   │   ├── graphics.nux
│   │   ├── oo_file.nux
│   │   ├── oo_graphics.nux
│   │   ├── random.nux
│   │   ├── collections.nux
│   │   ├── json.nux
│   │   ├── datetime.nux
│   │   ├── network.nux
│   │   ├── system.nux
│   │   ├── crypto.nux
│   │   └── regex.nux
│   └── embedded/       # Embedded Libs
│       ├── gpio.nux
│       ├── time.nux
│       └── analog.nux
├── examples/           # Example Scripts
│   ├── std_demo.nux
│   ├── class_demo.nux
│   ├── loop.nux
│   └── ...
└── nux_lang/           # Kernel Source
```

## Library Categories

### 🎯 Core (Essential)
Basic I/O, math, strings, files

### 🎨 Object-Oriented
Classes for cleaner API design

### 📊 Data Structures
Collections, JSON, DateTime

### 🌐 Networking & System
HTTP, sockets, OS interaction

### 🔐 Security
Crypto, hashing, encoding
