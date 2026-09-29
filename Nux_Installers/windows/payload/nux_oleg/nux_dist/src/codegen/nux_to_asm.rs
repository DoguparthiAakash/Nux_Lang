use crate::nvm::bytecode::{Opcode, BytecodeChunk, Value as NvmValue};
use std::fmt::Write;

pub struct AsmTranspiler {
    out: String,
}

impl AsmTranspiler {
    pub fn new() -> Self {
        let mut out = String::new();
        out.push_str("; Bare-metal Nux Assembly Transpilation (x86_64)\n");
        out.push_str("global _start\n\n");
        out.push_str("section .text\n");
        Self { out }
    }

    pub fn transpile(&mut self, function_name: &str, chunk: &BytecodeChunk) -> Result<String, String> {
        writeln!(&mut self.out, "{}:", function_name).unwrap();
        
        let mut offset = 0;
        while offset < chunk.code.len() {
            let opcode_byte = chunk.code[offset];
            let opcode = Opcode::from_u8(opcode_byte).ok_or(format!("Unknown opcode: {}", opcode_byte))?;

            match opcode {
                Opcode::LOAD_CONST => {
                    let const_idx = chunk.code[offset + 1] as usize;
                    let val = &chunk.constants[const_idx];
                    match val {
                        NvmValue::Int(i) => writeln!(&mut self.out, "    push {}", i).unwrap(),
                        _ => return Err("Unsupported constant type for ASM transpilation".to_string()),
                    }
                    offset += 2;
                }
                Opcode::ADD => {
                    writeln!(&mut self.out, "    pop rbx").unwrap();
                    writeln!(&mut self.out, "    pop rax").unwrap();
                    writeln!(&mut self.out, "    add rax, rbx").unwrap();
                    writeln!(&mut self.out, "    push rax").unwrap();
                    offset += 1;
                }
                Opcode::SUB => {
                    writeln!(&mut self.out, "    pop rbx").unwrap();
                    writeln!(&mut self.out, "    pop rax").unwrap();
                    writeln!(&mut self.out, "    sub rax, rbx").unwrap();
                    writeln!(&mut self.out, "    push rax").unwrap();
                    offset += 1;
                }
                Opcode::RETURN => {
                    writeln!(&mut self.out, "    pop rax").unwrap();
                    writeln!(&mut self.out, "    ret").unwrap();
                    offset += 1;
                }
                _ => return Err(format!("Opcode {:?} not supported in ASM transpiler", opcode)),
            }
        }
        
        Ok(self.out.clone())
    }
}
