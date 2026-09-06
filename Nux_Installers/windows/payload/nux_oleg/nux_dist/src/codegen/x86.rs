/// x86.rs — Nux x86-32 (i386) Machine Code Emitter
///
/// Emits raw x86 instruction bytes for the Nux native code generator.
/// This is the core of the "no GCC needed" backend for Navi OS (i686 bare-metal).
///
/// Reference: Intel Software Developer's Manual Vol 2.

use crate::elf_writer::{ElfObject, Symbol, Relocation, R_386_PC32, R_386_32};

// ─── Registers ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg32 {
    Eax = 0,
    Ecx = 1,
    Edx = 2,
    Ebx = 3,
    Esp = 4,
    Ebp = 5,
    Esi = 6,
    Edi = 7,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg8 {
    Al = 0,
    Cl = 1,
    Dl = 2,
    Bl = 3,
    Ah = 4,
    Ch = 5,
    Dh = 6,
    Bh = 7,
}

// ─── Instruction Emitter ─────────────────────────────────────────────────────

pub struct X86Emitter {
    pub code: Vec<u8>,
}

impl X86Emitter {
    pub fn new() -> Self {
        X86Emitter { code: Vec::new() }
    }

    pub fn pos(&self) -> u32 {
        self.code.len() as u32
    }

    fn emit(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    fn emit_i32(&mut self, v: i32) {
        self.code.extend_from_slice(&v.to_le_bytes());
    }

    fn emit_u32(&mut self, v: u32) {
        self.code.extend_from_slice(&v.to_le_bytes());
    }

    // ── Function prologue/epilogue ────────────────────────────────────────────

    /// push ebp; mov ebp, esp; sub esp, N  (N = local stack frame size in bytes)
    pub fn fn_prologue(&mut self, frame_size: u32) {
        self.emit(&[0x55]);                  // push ebp
        self.emit(&[0x89, 0xE5]);            // mov ebp, esp
        if frame_size > 0 {
            if frame_size <= 127 {
                self.emit(&[0x83, 0xEC, frame_size as u8]); // sub esp, imm8
            } else {
                self.emit(&[0x81, 0xEC]);    // sub esp, imm32
                self.emit_u32(frame_size);
            }
        }
    }

    /// mov esp, ebp; pop ebp; ret
    pub fn fn_epilogue(&mut self) {
        self.emit(&[0x89, 0xEC]);            // mov esp, ebp  (restore stack)
        self.emit(&[0x5D]);                  // pop ebp
        self.emit(&[0xC3]);                  // ret
    }

    // ── Data movement ────────────────────────────────────────────────────────

    /// mov reg, imm32
    pub fn mov_reg_imm32(&mut self, dst: Reg32, imm: u32) {
        self.emit(&[0xB8 + dst as u8]);
        self.emit_u32(imm);
    }

    /// mov reg, reg
    pub fn mov_reg_reg(&mut self, dst: Reg32, src: Reg32) {
        self.emit(&[0x89, 0xC0 | ((src as u8) << 3) | dst as u8]);
    }

    /// mov [ebp - offset], reg  (store local variable)
    pub fn mov_mem_ebp_neg(&mut self, offset: u8, src: Reg32) {
        self.emit(&[0x89, 0x45u8.wrapping_sub(0x45).wrapping_add(src as u8 * 8 + 0x45), offset.wrapping_neg()]);
        // Simplified: use ModRM with Mod=01 (disp8), rm=101 (ebp)
        // opcode: 89 /r   (MOV r/m32, r32)
        // ModRM: Mod=01, Reg=src, R/M=101 (ebp)
        // We already pushed incorrect bytes — fix:
        let len = self.code.len();
        self.code.truncate(len - 3);
        let modrm: u8 = 0x40 | ((src as u8) << 3) | 0x05; // Mod=01, Reg=src, RM=EBP
        self.emit(&[0x89, modrm, offset.wrapping_neg()]);
    }

    /// mov reg, [ebp - offset]  (load local variable)
    pub fn mov_reg_mem_ebp_neg(&mut self, dst: Reg32, offset: u8) {
        let modrm: u8 = 0x40 | ((dst as u8) << 3) | 0x05; // Mod=01, Reg=dst, RM=EBP
        self.emit(&[0x8B, modrm, offset.wrapping_neg()]);
    }

    /// mov [reg], reg  (store via pointer)
    pub fn mov_mem_reg(&mut self, ptr: Reg32, val: Reg32) {
        let modrm: u8 = (val as u8) << 3 | ptr as u8; // Mod=00
        self.emit(&[0x89, modrm]);
    }

    /// mov reg, [reg]  (load via pointer)
    pub fn mov_reg_mem(&mut self, dst: Reg32, ptr: Reg32) {
        let modrm: u8 = (dst as u8) << 3 | ptr as u8;
        self.emit(&[0x8B, modrm]);
    }

    // ── Arithmetic ───────────────────────────────────────────────────────────

    /// add reg, imm8
    pub fn add_reg_imm8(&mut self, dst: Reg32, imm: u8) {
        self.emit(&[0x83, 0xC0 | dst as u8, imm]);
    }

    /// add reg, reg
    pub fn add_reg_reg(&mut self, dst: Reg32, src: Reg32) {
        self.emit(&[0x01, 0xC0 | ((src as u8) << 3) | dst as u8]);
    }

    /// sub reg, imm8
    pub fn sub_reg_imm8(&mut self, dst: Reg32, imm: u8) {
        self.emit(&[0x83, 0xE8 | dst as u8, imm]);
    }

    /// sub reg, reg
    pub fn sub_reg_reg(&mut self, dst: Reg32, src: Reg32) {
        self.emit(&[0x29, 0xC0 | ((src as u8) << 3) | dst as u8]);
    }

    /// imul eax, reg  (signed multiply)
    pub fn imul_eax_reg(&mut self, src: Reg32) {
        self.emit(&[0xF7, 0xE8 | src as u8]);
    }

    // ── Comparison & branches ─────────────────────────────────────────────────

    /// cmp reg, imm8
    pub fn cmp_reg_imm8(&mut self, reg: Reg32, imm: u8) {
        self.emit(&[0x83, 0xF8 | reg as u8, imm]);
    }

    /// cmp reg, reg
    pub fn cmp_reg_reg(&mut self, lhs: Reg32, rhs: Reg32) {
        self.emit(&[0x39, 0xC0 | ((rhs as u8) << 3) | lhs as u8]);
    }

    /// Returns the position of the 4-byte relative offset for back-patching.
    /// Emits: jmp rel32  (0xE9 + 4-byte placeholder)
    pub fn jmp_rel32(&mut self) -> u32 {
        self.emit(&[0xE9, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4
    }

    /// je rel32
    pub fn je_rel32(&mut self) -> u32 {
        self.emit(&[0x0F, 0x84, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4
    }

    /// jne rel32
    pub fn jne_rel32(&mut self) -> u32 {
        self.emit(&[0x0F, 0x85, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4
    }

    /// jl rel32 (jump if less, signed)
    pub fn jl_rel32(&mut self) -> u32 {
        self.emit(&[0x0F, 0x8C, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4
    }

    /// jge rel32
    pub fn jge_rel32(&mut self) -> u32 {
        self.emit(&[0x0F, 0x8D, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4
    }

    /// Patch a previously emitted rel32 branch to jump to `target`.
    /// `patch_pos` is the offset returned by jmp_rel32/je_rel32/etc.
    pub fn patch_rel32(&mut self, patch_pos: u32, target: u32) {
        let rel = (target as i32) - (patch_pos as i32 + 4);
        let bytes = rel.to_le_bytes();
        let i = patch_pos as usize;
        self.code[i..i+4].copy_from_slice(&bytes);
    }

    // ── Stack ─────────────────────────────────────────────────────────────────

    /// push reg
    pub fn push_reg(&mut self, reg: Reg32) {
        self.emit(&[0x50 | reg as u8]);
    }

    /// push imm32
    pub fn push_imm32(&mut self, imm: u32) {
        self.emit(&[0x68]);
        self.emit_u32(imm);
    }

    /// pop reg
    pub fn pop_reg(&mut self, reg: Reg32) {
        self.emit(&[0x58 | reg as u8]);
    }

    // ── Function calls ────────────────────────────────────────────────────────

    /// call rel32  — emits a CALL with a 4-byte placeholder.
    /// Returns (patch_pos, reloc_offset) — both are the same value.
    /// The caller must add a relocation for the symbol.
    pub fn call_rel32(&mut self) -> u32 {
        self.emit(&[0xE8, 0x00, 0x00, 0x00, 0x00]);
        self.pos() - 4  // offset of the 4-byte operand
    }

    // ── I/O Port instructions ─────────────────────────────────────────────────

    /// outb: out dx, al  (write AL to port DX)
    pub fn outb_dx_al(&mut self) {
        self.emit(&[0xEE]);
    }

    /// inb: in al, dx  (read port DX into AL)
    pub fn inb_al_dx(&mut self) {
        self.emit(&[0xEC]);
    }

    // ── Interrupts / control ──────────────────────────────────────────────────

    /// cli — disable interrupts
    pub fn cli(&mut self) { self.emit(&[0xFA]); }

    /// sti — enable interrupts
    pub fn sti(&mut self) { self.emit(&[0xFB]); }

    /// hlt — halt
    pub fn hlt(&mut self) { self.emit(&[0xF4]); }

    /// nop
    pub fn nop(&mut self) { self.emit(&[0x90]); }

    /// int imm8 — software interrupt
    pub fn int_imm8(&mut self, n: u8) {
        self.emit(&[0xCD, n]);
    }

    // ── Logical ──────────────────────────────────────────────────────────────

    /// xor reg, reg  (zero register)
    pub fn xor_reg_reg(&mut self, dst: Reg32, src: Reg32) {
        self.emit(&[0x31, 0xC0 | ((src as u8) << 3) | dst as u8]);
    }

    /// and reg, imm8
    pub fn and_reg_imm8(&mut self, dst: Reg32, imm: u8) {
        self.emit(&[0x83, 0xE0 | dst as u8, imm]);
    }

    /// or reg, reg
    pub fn or_reg_reg(&mut self, dst: Reg32, src: Reg32) {
        self.emit(&[0x09, 0xC0 | ((src as u8) << 3) | dst as u8]);
    }

    // ── Returns ───────────────────────────────────────────────────────────────

    /// ret
    pub fn ret(&mut self) { self.emit(&[0xC3]); }

    /// Inline assembly passthrough — raw bytes embedded directly.
    /// Nux `asm("...")` blocks that GAS can't be called for are handled by
    /// a simple embedded assembler in the future; for now we allow raw hex blobs.
    pub fn raw_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }
}

// ─── Helper: compile a simple function stub ──────────────────────────────────
/// Emits a minimal function that just returns 0 (eax=0 then ret).
/// Used as placeholder while full IR→x86 lowering is implemented.
pub fn emit_stub_function(name: &str, obj: &mut ElfObject) {
    let mut em = X86Emitter::new();
    let start = em.pos();
    em.fn_prologue(0);
    em.xor_reg_reg(Reg32::Eax, Reg32::Eax); // return 0
    em.fn_epilogue();
    let end = em.pos();

    // Append machine code to .text
    let sym_offset = obj.text.len() as u32;
    obj.text.extend_from_slice(&em.code);

    // Add the function symbol
    obj.add_symbol(Symbol {
        name: name.to_string(),
        value: sym_offset,
        size: end - start,
        sym_type: 2, // STT_FUNC
        binding: 1,  // STB_GLOBAL
        section_idx: 1, // .text is section 1
    });
}
