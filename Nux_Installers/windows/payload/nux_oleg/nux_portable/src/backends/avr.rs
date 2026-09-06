use super::NativeEmitter;

pub struct AvrEmitter;

impl NativeEmitter for AvrEmitter {
    fn emit_header(&self) -> Vec<u8> {
        // AVR reset vector
        vec![0x00, 0x00]
    }
    
    fn emit_instruction(&self, _opcode: u8, _operands: &[i64]) -> Vec<u8> {
        vec![]
    }
    
    fn emit_footer(&self) -> Vec<u8> {
        vec![0x00]
    }
}
