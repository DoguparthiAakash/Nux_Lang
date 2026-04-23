<p align="center">
  <img src="nux_oleg/vscode_extension/icons/logo.png" width="200" alt="Nux Logo" />
</p>

# Nux OS/AI Language

Nux is a high-performance, memory-safe systems programming language designed for AI, ML, quantum computing, and OS development. It features a tier-based JIT compiler ensuring assembly-level runtime speeds, automatic garbage collection, and seamless interoperability.

## Features

- **Memory Safe & Lightning Fast**: Competes with Rust and C/C++ in performance securely.
- **LAG (Language Agnostic Gateway)**: Native execution and frictionless importing of Python, Rust, C++, Java, Zig, Go, and more using `@imports` and standard variable mapping without FFI overhead.
- **Micro-Binary Format (.nxb)**: Compiles down to highly compressed natively interpretable bytecode saving storage space and deployment times.
- **Universal Isolated Environments**: `bonfort venv` brings Python-like isolated virtual environments to Nux, allowing project-specific dependencies and lag-runtimes.
- **Bonfort Package Manager**: Simple dependency scaffolding and execution with `bonfort init`, `build`, `run`, and declarative `Bonfort.toml` & `imports.xml`.
- **Hybrid Runtime System**: Powered by the Overall Virtual Machine (OVM), transitioning dynamically from Interpreter → Baseline JIT → Optimized RISC-V Native Code.

## Quick Start

Initialize a project:

```bash
bonfort init my-project
cd my-project
```

Compile and run:

```bash
bonfort build main.nux
nux main.nux
```

Create an isolated virtual environment:

```bash
bonfort venv create env
source $(bonfort venv activate env)
bonfort lang add python@3.13  # installs isolated runtime into env
deactivate
```
