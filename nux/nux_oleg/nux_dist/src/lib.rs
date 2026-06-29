// Nux Programming Language - Library Interface
// Exports core modules and functionality

// Core compilation
pub mod lexer;
pub mod compiler;
pub mod assembler;
pub mod vm;

// Advanced runtime features (existing)
pub mod nvm;
pub mod jit;
pub mod polyglot_parser;
pub mod ffi_manager;
pub mod cux;
pub mod type_marshaller;
pub mod venv_manager;
pub mod bonfort_config;
pub mod package_manager;

// New standalone features (commented out until fully integrated)
// pub mod gc;
// pub mod security;
// pub mod profiler;
// pub mod runtime;
// pub mod package;
// pub mod codegen;
// pub mod debugger;
// pub mod lsp;
// pub mod build;
// pub mod wasm;
// pub mod distributed;

// Re-exports for convenience
pub use lexer::{Lexer, Token};
pub use compiler::{compile_to_asm_source, compile_high_level, CompileError};
pub use assembler::compile as assemble;
pub use vm::NuxVm;

/// Compile Nux source code to bytecode
pub fn compile(source: &str) -> Result<Vec<u8>, Vec<CompileError>> {
    let mut bytecode = compile_high_level(source)?;
    
    // Security: Encrypt the bytecode with a basic XOR cipher
    for b in bytecode.iter_mut() {
        *b ^= 0x5A;
    }
    
    // Security: Compute FNV-1a checksum
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in &bytecode {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    
    // Append magic bytes 'NUX!' and the 8-byte checksum
    bytecode.extend_from_slice(b"NUX!");
    bytecode.extend_from_slice(&hash.to_le_bytes());
    
    Ok(bytecode)
}

/// Compile Nux source code to assembly
pub fn compile_to_asm(source: &str) -> Result<String, Vec<CompileError>> {
    compile_to_asm_source(source)
}

/// Get Nux version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
