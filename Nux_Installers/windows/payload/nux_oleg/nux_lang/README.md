# Nux Language - Kernel Integration

This directory contains the Nux programming language implementation for the Ainux kernel.

## Files

- `lexer.rs` - Tokenizer/lexer for Nux syntax
- `high_level.rs` - High-level compiler (Nux → Assembly)
- `compiler.rs` - Low-level compiler/assembler (Assembly → Bytecode)
- `vm.rs` - Nux Virtual Machine with kernel integration
- `mod.rs` - Module declarations

## Integration

These files are integrated into the Ainux kernel at `src/nux/` and provide:
- Nux script compilation
- VM execution with kernel APIs
- Security intrinsics (sec_login, sec_whoami)
- Data Manager intrinsics (dm_get, dm_set)
- File I/O, graphics, and system operations

## Usage in Kernel

```rust
use crate::nux::{compile_high_level, NuxVm};

// Compile Nux source
let bytecode = compile_high_level(source)?;

// Execute in VM
let mut vm = NuxVm::new(bytecode);
vm.run();
```

## See Also

- `../nux_dist/` - Standalone Nux distribution for Linux
