// Nux Programming Language - Library Interface
pub mod lexer;
pub mod compiler;
pub mod assembler;
pub mod vm;

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
