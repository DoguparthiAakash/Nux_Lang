// Code Generation Module - Native code generation
pub mod x86_64;
pub mod x86;
pub mod nux_to_c;
pub mod nux_to_asm;
pub mod linker;
pub mod ptx_backend;

pub use x86_64::{X86_64CodeGen, X86Register, CodeGenError};
pub use nux_to_c::CTranspiler;
pub use nux_to_asm::AsmTranspiler;
pub use linker::Linker;
pub use ptx_backend::PtxTranspiler;

