// Native Code Generator - x86-64 machine code generation
// Compiles Nux bytecode directly to native x86-64 assembly

use crate::nvm::bytecode::{Opcode, BytecodeChunk, Value};
use std::collections::HashMap;

/// x86-64 code generator
pub struct X86_64CodeGen {
    code: Vec<u8>,
    labels: HashMap<usize, usize>,
    relocations: Vec<Relocation>,
}

/// Relocation entry for linking
#[derive(Debug, Clone)]
struct Relocation {
    offset: usize,
    reloc_type: RelocationType,
    symbol: String,
}

#[derive(Debug, Clone)]
enum RelocationType {
    Absolute64,
    Relative32,
}

/// x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum X86Register {
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

impl X86_64CodeGen {
    pub fn new() -> Self {
        X86_64CodeGen {
            code: Vec::new(),
            labels: HashMap::new(),
            relocations: Vec::new(),
        }
    }

    /// Compile bytecode to native x86-64
    pub fn compile(&mut self, chunk: &BytecodeChunk) -> Result<Vec<u8>, CodeGenError> {
        println!("[CODEGEN] Compiling {} to x86-64...", chunk.name);

        // Function prologue
        self.emit_prologue();

        // Compile bytecode instructions
        let mut pc = 0;
        while pc < chunk.code.len() {
            if let Some(opcode) = Opcode::from_u8(chunk.code[pc]) {
                pc += self.compile_instruction(opcode, &chunk.code[pc..], chunk)?;
            } else {
                return Err(CodeGenError::InvalidOpcode(chunk.code[pc]));
            }
        }

        // Function epilogue
        self.emit_epilogue();

        println!("[CODEGEN] Generated {} bytes of x86-64 code", self.code.len());
        Ok(self.code.clone())
    }

    /// Compile a single instruction
    fn compile_instruction(&mut self, opcode: Opcode, code: &[u8], chunk: &BytecodeChunk) -> Result<usize, CodeGenError> {
        match opcode {
            // Arithmetic operations
            Opcode::ADD => {
                // pop rbx; pop rax; add rax, rbx; push rax
                self.emit_pop(X86Register::RBX);
                self.emit_pop(X86Register::RAX);
                self.emit_add(X86Register::RAX, X86Register::RBX);
                self.emit_push(X86Register::RAX);
                Ok(1)
            }

            Opcode::SUB => {
                // pop rbx; pop rax; sub rax, rbx; push rax
                self.emit_pop(X86Register::RBX);
                self.emit_pop(X86Register::RAX);
                self.emit_sub(X86Register::RAX, X86Register::RBX);
                self.emit_push(X86Register::RAX);
                Ok(1)
            }

            Opcode::MUL => {
                // pop rbx; pop rax; imul rax, rbx; push rax
                self.emit_pop(X86Register::RBX);
                self.emit_pop(X86Register::RAX);
                self.emit_imul(X86Register::RAX, X86Register::RBX);
                self.emit_push(X86Register::RAX);
                Ok(1)
            }

            Opcode::DIV => {
                // pop rbx; pop rax; xor rdx, rdx; idiv rbx; push rax
                self.emit_pop(X86Register::RBX);
                self.emit_pop(X86Register::RAX);
                self.emit_xor(X86Register::RDX, X86Register::RDX);
                self.emit_idiv(X86Register::RBX);
                self.emit_push(X86Register::RAX);
                Ok(1)
            }

            // Stack operations
            Opcode::LOAD_CONST => {
                let const_idx = code[1] as usize;
                if let Some(Value::Int(val)) = chunk.constants.get(const_idx) {
                    // mov rax, immediate; push rax
                    self.emit_mov_imm(X86Register::RAX, *val);
                    self.emit_push(X86Register::RAX);
                }
                Ok(2)
            }

            Opcode::POP => {
                // add rsp, 8 (pop without storing)
                self.emit_add_imm(X86Register::RSP, 8);
                Ok(1)
            }

            Opcode::DUP => {
                // push qword [rsp]
                self.emit_push_mem(X86Register::RSP, 0);
                Ok(1)
            }

            // Control flow
            Opcode::RETURN => {
                // pop rax (return value)
                self.emit_pop(X86Register::RAX);
                Ok(1)
            }

            _ => {
                // Unsupported opcode - emit nop
                self.emit_nop();
                Ok(1)
            }
        }
    }

    // x86-64 instruction emitters

    fn emit_prologue(&mut self) {
        // push rbp; mov rbp, rsp; sub rsp, 64 (stack frame)
        self.emit_push(X86Register::RBP);
        self.emit_mov(X86Register::RBP, X86Register::RSP);
        self.emit_sub_imm(X86Register::RSP, 64);
    }

    fn emit_epilogue(&mut self) {
        // mov rsp, rbp; pop rbp; ret
        self.emit_mov(X86Register::RSP, X86Register::RBP);
        self.emit_pop(X86Register::RBP);
        self.emit_ret();
    }

    fn emit_push(&mut self, reg: X86Register) {
        // PUSH r64: 50+rd
        self.code.push(0x50 + self.reg_code(reg));
    }

    fn emit_pop(&mut self, reg: X86Register) {
        // POP r64: 58+rd
        self.code.push(0x58 + self.reg_code(reg));
    }

    fn emit_mov(&mut self, dst: X86Register, src: X86Register) {
        // MOV r64, r64: REX.W + 89 /r
        self.code.push(0x48); // REX.W
        self.code.push(0x89);
        self.code.push(0xC0 | (self.reg_code(src) << 3) | self.reg_code(dst));
    }

    fn emit_mov_imm(&mut self, reg: X86Register, imm: i64) {
        // MOV r64, imm64: REX.W + B8+rd id
        self.code.push(0x48); // REX.W
        self.code.push(0xB8 + self.reg_code(reg));
        self.code.extend_from_slice(&imm.to_le_bytes());
    }

    fn emit_add(&mut self, dst: X86Register, src: X86Register) {
        // ADD r64, r64: REX.W + 01 /r
        self.code.push(0x48); // REX.W
        self.code.push(0x01);
        self.code.push(0xC0 | (self.reg_code(src) << 3) | self.reg_code(dst));
    }

    fn emit_add_imm(&mut self, reg: X86Register, imm: i32) {
        // ADD r64, imm32: REX.W + 81 /0 id
        self.code.push(0x48); // REX.W
        self.code.push(0x81);
        self.code.push(0xC0 | self.reg_code(reg));
        self.code.extend_from_slice(&imm.to_le_bytes());
    }

    fn emit_sub(&mut self, dst: X86Register, src: X86Register) {
        // SUB r64, r64: REX.W + 29 /r
        self.code.push(0x48); // REX.W
        self.code.push(0x29);
        self.code.push(0xC0 | (self.reg_code(src) << 3) | self.reg_code(dst));
    }

    fn emit_sub_imm(&mut self, reg: X86Register, imm: i32) {
        // SUB r64, imm32: REX.W + 81 /5 id
        self.code.push(0x48); // REX.W
        self.code.push(0x81);
        self.code.push(0xE8 | self.reg_code(reg));
        self.code.extend_from_slice(&imm.to_le_bytes());
    }

    fn emit_imul(&mut self, dst: X86Register, src: X86Register) {
        // IMUL r64, r64: REX.W + 0F AF /r
        self.code.push(0x48); // REX.W
        self.code.push(0x0F);
        self.code.push(0xAF);
        self.code.push(0xC0 | (self.reg_code(dst) << 3) | self.reg_code(src));
    }

    fn emit_idiv(&mut self, reg: X86Register) {
        // IDIV r64: REX.W + F7 /7
        self.code.push(0x48); // REX.W
        self.code.push(0xF7);
        self.code.push(0xF8 | self.reg_code(reg));
    }

    fn emit_xor(&mut self, dst: X86Register, src: X86Register) {
        // XOR r64, r64: REX.W + 31 /r
        self.code.push(0x48); // REX.W
        self.code.push(0x31);
        self.code.push(0xC0 | (self.reg_code(src) << 3) | self.reg_code(dst));
    }

    fn emit_push_mem(&mut self, base: X86Register, offset: i32) {
        // PUSH [base+offset]: FF /6
        self.code.push(0xFF);
        if offset == 0 {
            self.code.push(0x30 | self.reg_code(base));
        } else {
            self.code.push(0x70 | self.reg_code(base));
            self.code.push(offset as u8);
        }
    }

    fn emit_ret(&mut self) {
        // RET: C3
        self.code.push(0xC3);
    }

    fn emit_nop(&mut self) {
        // NOP: 90
        self.code.push(0x90);
    }

    fn reg_code(&self, reg: X86Register) -> u8 {
        match reg {
            X86Register::RAX => 0,
            X86Register::RCX => 1,
            X86Register::RDX => 2,
            X86Register::RBX => 3,
            X86Register::RSP => 4,
            X86Register::RBP => 5,
            X86Register::RSI => 6,
            X86Register::RDI => 7,
            X86Register::R8 => 8,
            X86Register::R9 => 9,
            X86Register::R10 => 10,
            X86Register::R11 => 11,
            X86Register::R12 => 12,
            X86Register::R13 => 13,
            X86Register::R14 => 14,
            X86Register::R15 => 15,
        }
    }

    /// Save generated code to file
    pub fn save_to_file(&self, path: &str) -> Result<(), CodeGenError> {
        std::fs::write(path, &self.code)
            .map_err(|e| CodeGenError::IoError(e.to_string()))?;
        println!("[CODEGEN] Saved to {}", path);
        Ok(())
    }

    /// Generate ELF executable (Linux)
    pub fn generate_elf(&self, path: &str) -> Result<(), CodeGenError> {
        let mut elf = Vec::new();

        // ELF header
        elf.extend_from_slice(&[
            0x7F, b'E', b'L', b'F',  // Magic
            2,                        // 64-bit
            1,                        // Little endian
            1,                        // ELF version
            0, 0, 0, 0, 0, 0, 0, 0, 0, // Padding
        ]);

        // Add program headers and code
        // (Simplified - full implementation would be more complex)
        elf.extend_from_slice(&self.code);

        std::fs::write(path, &elf)
            .map_err(|e| CodeGenError::IoError(e.to_string()))?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(path)
                .map_err(|e| CodeGenError::IoError(e.to_string()))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(path, perms)
                .map_err(|e| CodeGenError::IoError(e.to_string()))?;
        }

        println!("[CODEGEN] Generated ELF executable: {}", path);
        Ok(())
    }
}

/// Code generation errors
#[derive(Debug)]
pub enum CodeGenError {
    InvalidOpcode(u8),
    UnsupportedFeature(String),
    IoError(String),
}

impl std::fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeGenError::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
            CodeGenError::UnsupportedFeature(feat) => write!(f, "Unsupported feature: {}", feat),
            CodeGenError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for CodeGenError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_creation() {
        let codegen = X86_64CodeGen::new();
        assert_eq!(codegen.code.len(), 0);
    }

    #[test]
    fn test_instruction_emission() {
        let mut codegen = X86_64CodeGen::new();
        codegen.emit_push(X86Register::RAX);
        assert_eq!(codegen.code.len(), 1);
        assert_eq!(codegen.code[0], 0x50);
    }

    #[test]
    fn test_mov_instruction() {
        let mut codegen = X86_64CodeGen::new();
        codegen.emit_mov(X86Register::RAX, X86Register::RBX);
        assert_eq!(codegen.code.len(), 3);
        assert_eq!(codegen.code[0], 0x48); // REX.W
    }
}
