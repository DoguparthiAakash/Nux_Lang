// Nux Binary Code Generator
// Direct machine code generation for maximum performance

use std::collections::HashMap;

// ===== X86-64 MACHINE CODE GENERATOR =====

pub struct X86CodeGen {
    code: Vec<u8>,
    labels: HashMap<String, usize>,
    relocations: Vec<(usize, String)>,
}

impl X86CodeGen {
    pub fn new() -> Self {
        X86CodeGen {
            code: Vec::new(),
            labels: HashMap::new(),
            relocations: Vec::new(),
        }
    }

    // ===== REGISTER ENCODING =====
    const RAX: u8 = 0;
    const RCX: u8 = 1;
    const RDX: u8 = 2;
    const RBX: u8 = 3;
    const RSP: u8 = 4;
    const RBP: u8 = 5;
    const RSI: u8 = 6;
    const RDI: u8 = 7;
    const R8: u8 = 8;
    const R9: u8 = 9;
    const R10: u8 = 10;
    const R11: u8 = 11;
    const R12: u8 = 12;
    const R13: u8 = 13;
    const R14: u8 = 14;
    const R15: u8 = 15;

    // ===== ARITHMETIC OPERATIONS =====

    pub fn add_reg_reg(&mut self, dst: u8, src: u8) {
        // add dst, src
        self.emit_rex(true, dst, src);
        self.code.push(0x01);
        self.emit_modrm(0b11, src, dst);
    }

    pub fn sub_reg_reg(&mut self, dst: u8, src: u8) {
        // sub dst, src
        self.emit_rex(true, dst, src);
        self.code.push(0x29);
        self.emit_modrm(0b11, src, dst);
    }

    pub fn imul_reg_reg(&mut self, dst: u8, src: u8) {
        // imul dst, src
        self.emit_rex(true, dst, src);
        self.code.push(0x0F);
        self.code.push(0xAF);
        self.emit_modrm(0b11, dst, src);
    }

    pub fn xor_reg_reg(&mut self, dst: u8, src: u8) {
        // xor dst, src (fastest way to zero a register)
        self.emit_rex(true, dst, src);
        self.code.push(0x31);
        self.emit_modrm(0b11, src, dst);
    }

    // ===== IMMEDIATE VALUES =====

    pub fn mov_reg_imm64(&mut self, reg: u8, value: i64) {
        // mov reg, imm64
        self.emit_rex(true, reg, 0);
        self.code.push(0xB8 + (reg & 0x7));
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    pub fn add_reg_imm32(&mut self, reg: u8, value: i32) {
        // add reg, imm32
        self.emit_rex(true, reg, 0);
        if reg == Self::RAX {
            self.code.push(0x05);
        } else {
            self.code.push(0x81);
            self.emit_modrm(0b11, 0, reg);
        }
        self.code.extend_from_slice(&value.to_le_bytes());
    }

    // ===== MEMORY OPERATIONS =====

    pub fn mov_reg_mem(&mut self, dst: u8, base: u8, offset: i32) {
        // mov dst, [base + offset]
        self.emit_rex(true, dst, base);
        self.code.push(0x8B);
        
        if offset == 0 && base != Self::RBP {
            self.emit_modrm(0b00, dst, base);
        } else if offset >= -128 && offset <= 127 {
            self.emit_modrm(0b01, dst, base);
            self.code.push(offset as u8);
        } else {
            self.emit_modrm(0b10, dst, base);
            self.code.extend_from_slice(&offset.to_le_bytes());
        }
    }

    pub fn mov_mem_reg(&mut self, base: u8, offset: i32, src: u8) {
        // mov [base + offset], src
        self.emit_rex(true, src, base);
        self.code.push(0x89);
        
        if offset == 0 && base != Self::RBP {
            self.emit_modrm(0b00, src, base);
        } else if offset >= -128 && offset <= 127 {
            self.emit_modrm(0b01, src, base);
            self.code.push(offset as u8);
        } else {
            self.emit_modrm(0b10, src, base);
            self.code.extend_from_slice(&offset.to_le_bytes());
        }
    }

    // ===== CONTROL FLOW =====

    pub fn jmp_label(&mut self, label: &str) {
        // jmp label (near)
        self.code.push(0xE9);
        self.relocations.push((self.code.len(), label.to_string()));
        self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder
    }

    pub fn je_label(&mut self, label: &str) {
        // je label (jump if equal)
        self.code.push(0x0F);
        self.code.push(0x84);
        self.relocations.push((self.code.len(), label.to_string()));
        self.code.extend_from_slice(&[0, 0, 0, 0]);
    }

    pub fn jne_label(&mut self, label: &str) {
        // jne label (jump if not equal)
        self.code.push(0x0F);
        self.code.push(0x85);
        self.relocations.push((self.code.len(), label.to_string()));
        self.code.extend_from_slice(&[0, 0, 0, 0]);
    }

    pub fn call_label(&mut self, label: &str) {
        // call label
        self.code.push(0xE8);
        self.relocations.push((self.code.len(), label.to_string()));
        self.code.extend_from_slice(&[0, 0, 0, 0]);
    }

    pub fn ret(&mut self) {
        // ret
        self.code.push(0xC3);
    }

    // ===== COMPARISON =====

    pub fn cmp_reg_reg(&mut self, left: u8, right: u8) {
        // cmp left, right
        self.emit_rex(true, left, right);
        self.code.push(0x39);
        self.emit_modrm(0b11, right, left);
    }

    pub fn test_reg_reg(&mut self, left: u8, right: u8) {
        // test left, right
        self.emit_rex(true, left, right);
        self.code.push(0x85);
        self.emit_modrm(0b11, right, left);
    }

    // ===== FUNCTION PROLOGUE/EPILOGUE =====

    pub fn prologue(&mut self) {
        // push rbp
        self.code.push(0x55);
        // mov rbp, rsp
        self.emit_rex(true, Self::RBP, Self::RSP);
        self.code.push(0x89);
        self.emit_modrm(0b11, Self::RSP, Self::RBP);
    }

    pub fn epilogue(&mut self) {
        // mov rsp, rbp
        self.emit_rex(true, Self::RSP, Self::RBP);
        self.code.push(0x89);
        self.emit_modrm(0b11, Self::RBP, Self::RSP);
        // pop rbp
        self.code.push(0x5D);
        // ret
        self.ret();
    }

    // ===== SIMD OPERATIONS (AVX2) =====

    pub fn vaddpd_ymm(&mut self, dst: u8, src1: u8, src2: u8) {
        // vaddpd ymm_dst, ymm_src1, ymm_src2
        self.code.push(0xC5); // VEX prefix
        self.code.push(0xFD); // VEX.vvvv
        self.code.push(0x58); // opcode
        self.emit_modrm(0b11, dst, src2);
    }

    pub fn vmulpd_ymm(&mut self, dst: u8, src1: u8, src2: u8) {
        // vmulpd ymm_dst, ymm_src1, ymm_src2
        self.code.push(0xC5);
        self.code.push(0xFD);
        self.code.push(0x59);
        self.emit_modrm(0b11, dst, src2);
    }

    // ===== HELPER FUNCTIONS =====

    fn emit_rex(&mut self, w: bool, reg: u8, rm: u8) {
        let mut rex = 0x40;
        if w { rex |= 0x08; }
        if reg >= 8 { rex |= 0x04; }
        if rm >= 8 { rex |= 0x01; }
        if rex != 0x40 {
            self.code.push(rex);
        }
    }

    fn emit_modrm(&mut self, mode: u8, reg: u8, rm: u8) {
        self.code.push((mode << 6) | ((reg & 0x7) << 3) | (rm & 0x7));
    }

    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.code.len());
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        // Resolve relocations
        for (offset, label) in &self.relocations {
            if let Some(&target) = self.labels.get(label) {
                let rel = (target as i32) - (*offset as i32) - 4;
                self.code[*offset..*offset + 4].copy_from_slice(&rel.to_le_bytes());
            }
        }
        
        self.code.clone()
    }
}

// ===== INLINE ASSEMBLY SUPPORT =====

#[macro_export]
macro_rules! asm_inline {
    ($($asm:tt)*) => {
        unsafe {
            std::arch::asm!($($asm)*);
        }
    };
}

// ===== FAST SYSTEM CALLS =====

pub struct FastSyscall;

impl FastSyscall {
    #[inline(always)]
    pub fn syscall0(n: u64) -> u64 {
        let ret: u64;
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") n,
                lateout("rax") ret,
                options(nostack, preserves_flags)
            );
        }
        ret
    }

    #[inline(always)]
    pub fn syscall1(n: u64, arg1: u64) -> u64 {
        let ret: u64;
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") n,
                in("rdi") arg1,
                lateout("rax") ret,
                options(nostack, preserves_flags)
            );
        }
        ret
    }

    #[inline(always)]
    pub fn syscall3(n: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
        let ret: u64;
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") n,
                in("rdi") arg1,
                in("rsi") arg2,
                in("rdx") arg3,
                lateout("rax") ret,
                options(nostack, preserves_flags)
            );
        }
        ret
    }
}

// ===== EXAMPLE: COMPILE NUX TO MACHINE CODE =====

pub fn compile_nux_function(name: &str, nux_code: &str) -> Vec<u8> {
    let mut gen = X86CodeGen::new();
    
    // Function prologue
    gen.label(name);
    gen.prologue();
    
    // Example: compile "return a + b"
    // Assume a in RDI, b in RSI (System V ABI)
    gen.mov_reg_imm64(X86CodeGen::RAX, 0);  // Clear RAX
    gen.add_reg_reg(X86CodeGen::RAX, X86CodeGen::RDI);  // RAX = a
    gen.add_reg_reg(X86CodeGen::RAX, X86CodeGen::RSI);  // RAX += b
    
    // Function epilogue
    gen.epilogue();
    
    gen.finalize()
}
