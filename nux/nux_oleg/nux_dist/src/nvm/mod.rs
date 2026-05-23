// NVM (Nux Virtual Machine) - Module root
// Bytecode-based virtual machine for Nux

pub mod bytecode;
pub mod vm;

pub use bytecode::{Opcode, Value, BytecodeChunk, BytecodeCompiler};
pub use vm::{NuxVM, VMError};
