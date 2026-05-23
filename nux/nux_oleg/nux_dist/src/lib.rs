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
    compile_high_level(source)
}

/// Compile Nux source code to assembly
pub fn compile_to_asm(source: &str) -> Result<String, Vec<CompileError>> {
    compile_to_asm_source(source)
}

/// Get Nux version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
