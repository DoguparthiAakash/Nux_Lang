pub mod xtensa;
pub mod riscv;
pub mod avr;

pub trait NativeEmitter {
    /// Emit initialization/bootloader code
    fn emit_header(&self) -> Vec<u8>;
    
    /// Translate a standard Nux opcode (or sequence) to native machine code
    fn emit_instruction(&self, opcode: u8, operands: &[i64]) -> Vec<u8>;
    
    /// Emit the footer/halt logic
    fn emit_footer(&self) -> Vec<u8>;
}
