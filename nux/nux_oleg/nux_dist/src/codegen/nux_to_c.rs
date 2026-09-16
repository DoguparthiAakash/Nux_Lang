use crate::nvm::bytecode::{Opcode, BytecodeChunk, Value as NvmValue};
use std::fmt::Write;

pub struct CTranspiler {
    out: String,
}

impl CTranspiler {
    pub fn new() -> Self {
        let mut out = String::new();
        // Setup freestanding environment
        out.push_str("/* Freestanding Nux to C Transpilation */\n");
        out.push_str("#include <stdint.h>\n\n");
        Self { out }
    }

    pub fn transpile(&mut self, function_name: &str, chunk: &BytecodeChunk) -> Result<String, String> {
        writeln!(&mut self.out, "int64_t {}() {{", function_name).unwrap();
        
        // Simulating the Nux stack in C
        writeln!(&mut self.out, "    int64_t stack[256];").unwrap();
        writeln!(&mut self.out, "    int sp = 0;").unwrap();

        let mut offset = 0;
        while offset < chunk.code.len() {
            let opcode_byte = chunk.code[offset];
            let opcode = Opcode::from_u8(opcode_byte).ok_or(format!("Unknown opcode: {}", opcode_byte))?;

            match opcode {
                Opcode::LOAD_CONST => {
                    let const_idx = chunk.code[offset + 1] as usize;
                    let val = &chunk.constants[const_idx];
                    match val {
                        NvmValue::Int(i) => writeln!(&mut self.out, "    stack[sp++] = {}LL;", i).unwrap(),
                        _ => return Err("Unsupported constant type for C transpilation".to_string()),
                    }
                    offset += 2;
                }
                Opcode::ADD => {
                    writeln!(&mut self.out, "    sp -= 2;").unwrap();
                    writeln!(&mut self.out, "    stack[sp] = stack[sp] + stack[sp+1];").unwrap();
                    writeln!(&mut self.out, "    sp += 1;").unwrap();
                    offset += 1;
                }
                Opcode::SUB => {
                    writeln!(&mut self.out, "    sp -= 2;").unwrap();
                    writeln!(&mut self.out, "    stack[sp] = stack[sp] - stack[sp+1];").unwrap();
                    writeln!(&mut self.out, "    sp += 1;").unwrap();
                    offset += 1;
                }
                Opcode::MUL => {
                    writeln!(&mut self.out, "    sp -= 2;").unwrap();
                    writeln!(&mut self.out, "    stack[sp] = stack[sp] * stack[sp+1];").unwrap();
                    writeln!(&mut self.out, "    sp += 1;").unwrap();
                    offset += 1;
                }
                Opcode::DIV => {
                    writeln!(&mut self.out, "    sp -= 2;").unwrap();
                    writeln!(&mut self.out, "    stack[sp] = stack[sp] / stack[sp+1];").unwrap();
                    writeln!(&mut self.out, "    sp += 1;").unwrap();
                    offset += 1;
                }
                Opcode::RETURN => {
                    writeln!(&mut self.out, "    return stack[--sp];").unwrap();
                    offset += 1;
                }
                _ => return Err(format!("Opcode {:?} not supported in C transpiler", opcode)),
            }
        }
        
        writeln!(&mut self.out, "}}\n").unwrap();
        Ok(self.out.clone())
    }
}
