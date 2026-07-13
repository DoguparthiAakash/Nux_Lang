use super::NativeEmitter;

pub struct RiscvEmitter;

impl NativeEmitter for RiscvEmitter {
    fn emit_header(&self) -> Vec<u8> {
        // RISC-V boot stub
        vec![0x00, 0x00]
    }
    
    fn emit_instruction(&self, _opcode: u8, _operands: &[i64]) -> Vec<u8> {
        vec![]
    }
    
    fn emit_footer(&self) -> Vec<u8> {
        vec![0x00]
    }
}
