/// elf_writer.rs — Nux Native ELF32 Object File Writer
///
/// Produces a valid ELF32 relocatable object (.o) file from raw machine code bytes
/// and a symbol table. This removes the GCC dependency for the Nux native backend.
///
/// ELF32 format reference: https://refspecs.linuxbase.org/elf/elf.pdf

use std::collections::HashMap;

// ─── ELF Constants ───────────────────────────────────────────────────────────

const ELFMAG: &[u8] = b"\x7FELF";
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1; // Little-endian
const ET_REL: u16 = 1; // Relocatable file
const EM_386: u16 = 3; // Intel 80386
const EV_CURRENT: u8 = 1;
const SHT_NULL: u32 = 0;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_REL: u32 = 9;
const SHF_ALLOC: u32 = 0x2;
const SHF_EXECINSTR: u32 = 0x4;
const SHF_WRITE: u32 = 0x1;
const STB_LOCAL: u8 = 0;
const STB_GLOBAL: u8 = 1;
const STT_NOTYPE: u8 = 0;
const STT_OBJECT: u8 = 1;
const STT_FUNC: u8 = 2;
const STT_SECTION: u8 = 3;
const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xFFF1;

// R_386 relocation types
pub const R_386_32: u8 = 1;
pub const R_386_PC32: u8 = 2;

// ─── Data Structures ─────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub value: u32,     // offset within section
    pub size: u32,
    pub sym_type: u8,   // STT_*
    pub binding: u8,    // STB_*
    pub section_idx: u16, // which section this symbol lives in (SHN_UNDEF if extern)
}

#[derive(Clone)]
pub struct Relocation {
    pub offset: u32,    // offset within .text where the relocation applies
    pub sym_name: String, // name of the symbol to relocate against
    pub rel_type: u8,   // R_386_*
}

pub struct ElfObject {
    pub text: Vec<u8>,
    pub data: Vec<u8>,
    pub bss_size: u32,
    pub symbols: Vec<Symbol>,
    pub relocations: Vec<Relocation>,
}

impl ElfObject {
    pub fn new() -> Self {
        ElfObject {
            text: Vec::new(),
            data: Vec::new(),
            bss_size: 0,
            symbols: Vec::new(),
            relocations: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, sym: Symbol) {
        self.symbols.push(sym);
    }

    pub fn add_reloc(&mut self, rel: Relocation) {
        self.relocations.push(rel);
    }
}

// ─── String Table Builder ─────────────────────────────────────────────────────

struct Strtab {
    data: Vec<u8>,
    map: HashMap<String, u32>,
}

impl Strtab {
    fn new() -> Self {
        // String tables always start with a null byte
        Strtab { data: vec![0], map: HashMap::new() }
    }

    fn add(&mut self, s: &str) -> u32 {
        if let Some(&off) = self.map.get(s) {
            return off;
        }
        let off = self.data.len() as u32;
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0);
        self.map.insert(s.to_string(), off);
        off
    }

    fn bytes(&self) -> &[u8] {
        &self.data
    }
}

// ─── ELF32 Header Serialization ──────────────────────────────────────────────

fn u16le(v: u16) -> [u8; 2] { v.to_le_bytes() }
fn u32le(v: u32) -> [u8; 4] { v.to_le_bytes() }

fn elf32_header(shoff: u32, shnum: u16, shstrndx: u16) -> Vec<u8> {
    let mut h = Vec::with_capacity(52);
    h.extend_from_slice(ELFMAG);      // e_ident[0..4]  magic
    h.push(ELFCLASS32);               // e_ident[4]     class
    h.push(ELFDATA2LSB);              // e_ident[5]     data encoding
    h.push(EV_CURRENT);              // e_ident[6]     version
    h.push(0);                        // e_ident[7]     OS/ABI (System V)
    h.extend_from_slice(&[0u8; 8]);   // e_ident[8..15] padding
    h.extend_from_slice(&u16le(ET_REL));       // e_type
    h.extend_from_slice(&u16le(EM_386));       // e_machine
    h.extend_from_slice(&u32le(1));            // e_version
    h.extend_from_slice(&u32le(0));            // e_entry (none for .o)
    h.extend_from_slice(&u32le(0));            // e_phoff (no program headers)
    h.extend_from_slice(&u32le(shoff));        // e_shoff
    h.extend_from_slice(&u32le(0));            // e_flags
    h.extend_from_slice(&u16le(52));           // e_ehsize
    h.extend_from_slice(&u16le(32));           // e_phentsize
    h.extend_from_slice(&u16le(0));            // e_phnum
    h.extend_from_slice(&u16le(40));           // e_shentsize
    h.extend_from_slice(&u16le(shnum));        // e_shnum
    h.extend_from_slice(&u16le(shstrndx));     // e_shstrndx
    h
}

fn elf32_shdr(name: u32, sh_type: u32, flags: u32, offset: u32, size: u32, link: u32, info: u32, addralign: u32, entsize: u32) -> Vec<u8> {
    let mut s = Vec::with_capacity(40);
    s.extend_from_slice(&u32le(name));
    s.extend_from_slice(&u32le(sh_type));
    s.extend_from_slice(&u32le(flags));
    s.extend_from_slice(&u32le(0));      // sh_addr (zero for relocatable)
    s.extend_from_slice(&u32le(offset));
    s.extend_from_slice(&u32le(size));
    s.extend_from_slice(&u32le(link));
    s.extend_from_slice(&u32le(info));
    s.extend_from_slice(&u32le(addralign));
    s.extend_from_slice(&u32le(entsize));
    s
}

fn elf32_sym(name: u32, value: u32, size: u32, info: u8, other: u8, shndx: u16) -> Vec<u8> {
    let mut s = Vec::with_capacity(16);
    s.extend_from_slice(&u32le(name));
    s.extend_from_slice(&u32le(value));
    s.extend_from_slice(&u32le(size));
    s.push(info);
    s.push(other);
    s.extend_from_slice(&u16le(shndx));
    s
}

fn elf32_rel(offset: u32, info: u32) -> Vec<u8> {
    let mut r = Vec::with_capacity(8);
    r.extend_from_slice(&u32le(offset));
    r.extend_from_slice(&u32le(info));
    r
}

// ─── Main Writer ─────────────────────────────────────────────────────────────

/// Serializes an ElfObject into a valid ELF32 relocatable object file.
pub fn write_elf32(obj: &ElfObject) -> Vec<u8> {
    // Section indices
    // 0 = SHN_UNDEF (null)
    // 1 = .text
    // 2 = .data       (only if non-empty)
    // 3 = .bss        (only if non-zero)
    // 4 = .rel.text   (only if relocations exist)
    // 5 = .symtab
    // 6 = .strtab
    // 7 = .shstrtab

    let has_data = !obj.data.is_empty();
    let has_bss = obj.bss_size > 0;
    let has_rel = !obj.relocations.is_empty();

    // ── Build section name string table (.shstrtab) ───────────────────────────
    let mut shstrtab = Strtab::new();
    let nm_null = shstrtab.add("");
    let nm_text = shstrtab.add(".text");
    let nm_data = if has_data { shstrtab.add(".data") } else { 0 };
    let nm_bss  = if has_bss  { shstrtab.add(".bss")  } else { 0 };
    let nm_rel  = if has_rel  { shstrtab.add(".rel.text") } else { 0 };
    let nm_sym  = shstrtab.add(".symtab");
    let nm_str  = shstrtab.add(".strtab");
    let nm_shstr = shstrtab.add(".shstrtab");

    // ── Build symbol string table (.strtab) + symbol table ───────────────────
    let mut strtab = Strtab::new();
    strtab.add(""); // index 0 = empty string

    // Collect symbol name offsets
    let sym_names: Vec<u32> = obj.symbols.iter().map(|s| strtab.add(&s.name)).collect();

    // Assign section indices for relocation symbol lookup
    let mut sym_index_map: HashMap<String, u32> = HashMap::new();
    let local_syms_count = obj.symbols.iter().filter(|s| s.binding == STB_LOCAL).count() as u32 + 1; // +1 for null sym

    // Build symbol table bytes
    // ELF requires: null sym first, then all LOCAL syms, then GLOBAL syms
    // .symtab info = index of first global symbol
    let mut symtab_bytes: Vec<u8> = Vec::new();

    // Null symbol
    symtab_bytes.extend_from_slice(&elf32_sym(0, 0, 0, 0, 0, SHN_UNDEF));

    // Determine section index assignments
    let mut sect_idx_text: u16 = 1;
    let mut next_sect: u16 = 2;
    let sect_idx_data: u16 = if has_data { let i = next_sect; next_sect += 1; i } else { 0 };
    let sect_idx_bss:  u16 = if has_bss  { let i = next_sect; next_sect += 1; i } else { 0 };

    // Sort: locals first, globals last
    let mut local_syms: Vec<(usize, &Symbol)> = obj.symbols.iter().enumerate()
        .filter(|(_, s)| s.binding == STB_LOCAL).collect();
    let mut global_syms: Vec<(usize, &Symbol)> = obj.symbols.iter().enumerate()
        .filter(|(_, s)| s.binding == STB_GLOBAL).collect();

    let first_global_idx = 1 + local_syms.len() as u32;

    for (orig_i, sym) in local_syms.iter().chain(global_syms.iter()) {
        let name_off = sym_names[*orig_i];
        let shndx = if sym.section_idx == SHN_UNDEF { SHN_UNDEF } else { sym.section_idx };
        let info = (sym.binding << 4) | (sym.sym_type & 0xF);
        symtab_bytes.extend_from_slice(&elf32_sym(name_off, sym.value, sym.size, info, 0, shndx));
        // Track name → symtab index for relocations
        let my_idx = symtab_bytes.len() as u32 / 16 - 1;
        sym_index_map.insert(sym.name.clone(), my_idx);
    }

    // ── Build .rel.text ───────────────────────────────────────────────────────
    let mut rel_bytes: Vec<u8> = Vec::new();
    for rel in &obj.relocations {
        let sym_idx = sym_index_map.get(&rel.sym_name).copied().unwrap_or(0);
        let info = (sym_idx << 8) | (rel.rel_type as u32);
        rel_bytes.extend_from_slice(&elf32_rel(rel.offset, info));
    }

    // ── Layout: compute section offsets ──────────────────────────────────────
    let elf_hdr_size: u32 = 52;
    let mut offset = elf_hdr_size;

    let text_off = offset;
    let text_size = obj.text.len() as u32;
    offset += text_size;

    let data_off = if has_data { let o = offset; offset += obj.data.len() as u32; o } else { 0 };
    let data_size = obj.data.len() as u32;

    // .bss has no content bytes — size tracked separately
    let bss_off = 0u32; // doesn't consume file space

    let rel_off  = if has_rel  { let o = offset; offset += rel_bytes.len() as u32; o } else { 0 };
    let rel_size = rel_bytes.len() as u32;

    let sym_off  = offset; offset += symtab_bytes.len() as u32;
    let sym_size = symtab_bytes.len() as u32;

    let str_off  = offset; offset += strtab.bytes().len() as u32;
    let str_size = strtab.bytes().len() as u32;

    let shstr_off  = offset; offset += shstrtab.bytes().len() as u32;
    let shstr_size = shstrtab.bytes().len() as u32;

    // Align section header table to 4 bytes
    let align4 = (4 - (offset % 4)) % 4;
    offset += align4;
    let shdr_off = offset;

    // Count sections
    let shnum: u16 = {
        let mut n: u16 = 3; // null + .text + .symtab + .strtab + .shstrtab = 5
        n += 2; // .strtab, .shstrtab
        if has_data { n += 1; }
        if has_bss  { n += 1; }
        if has_rel  { n += 1; }
        n
    };

    // .shstrtab index = last section
    let shstrndx = shnum - 1;

    // .symtab link = .strtab idx, info = first global sym
    // symtab section index
    let symtab_sect_idx: u32 = {
        let mut i = 2u32; // null=0, text=1
        if has_data { i += 1; }
        if has_bss  { i += 1; }
        if has_rel  { i += 1; }
        i // symtab
    };
    let strtab_sect_idx = symtab_sect_idx + 1;
    let text_sect_idx_for_rel: u32 = 1;

    // ── Assemble the binary ───────────────────────────────────────────────────
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(&elf32_header(shdr_off, shnum, shstrndx));
    out.extend_from_slice(&obj.text);
    if has_data { out.extend_from_slice(&obj.data); }
    if has_rel  { out.extend_from_slice(&rel_bytes); }
    out.extend_from_slice(&symtab_bytes);
    out.extend_from_slice(strtab.bytes());
    out.extend_from_slice(shstrtab.bytes());
    // Padding to align shdr table
    out.extend_from_slice(&vec![0u8; align4 as usize]);

    // ── Section headers ───────────────────────────────────────────────────────
    // [0] NULL
    out.extend_from_slice(&elf32_shdr(nm_null, SHT_NULL, 0, 0, 0, 0, 0, 0, 0));
    // [1] .text
    out.extend_from_slice(&elf32_shdr(nm_text, SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
        text_off, text_size, 0, 0, 16, 0));
    // [2?] .data
    if has_data {
        out.extend_from_slice(&elf32_shdr(nm_data, SHT_PROGBITS, SHF_ALLOC | SHF_WRITE,
            data_off, data_size, 0, 0, 4, 0));
    }
    // .bss
    if has_bss {
        out.extend_from_slice(&elf32_shdr(nm_bss, 8 /*SHT_NOBITS*/, SHF_ALLOC | SHF_WRITE,
            0, obj.bss_size, 0, 0, 16, 0));
    }
    // .rel.text  (link=symtab idx, info=text section idx)
    if has_rel {
        out.extend_from_slice(&elf32_shdr(nm_rel, SHT_REL, 0,
            rel_off, rel_size, symtab_sect_idx, text_sect_idx_for_rel, 4, 8));
    }
    // .symtab  (link=strtab idx, info=first global sym)
    out.extend_from_slice(&elf32_shdr(nm_sym, SHT_SYMTAB, 0,
        sym_off, sym_size, strtab_sect_idx, first_global_idx, 4, 16));
    // .strtab
    out.extend_from_slice(&elf32_shdr(nm_str, SHT_STRTAB, 0,
        str_off, str_size, 0, 0, 1, 0));
    // .shstrtab
    out.extend_from_slice(&elf32_shdr(nm_shstr, SHT_STRTAB, 0,
        shstr_off, shstr_size, 0, 0, 1, 0));

    out
}
