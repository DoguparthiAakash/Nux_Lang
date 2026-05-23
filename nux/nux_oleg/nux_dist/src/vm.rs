// Standalone Nux VM
// Implements executing Nux Bytecode

use std::vec::Vec;
use std::string::String;
use std::io::{self, Write};
use std::convert::TryInto;
#[cfg(feature = "minifb")]
use minifb::{Window, WindowOptions, Scale};

// const OP_VBE_SET_MODE: u8 = 0x3A;
// const OP_VBE_GET_FB: u8 = 0x3B;
// const OP_VBE_UPDATE: u8 = 0x3C;

pub struct NuxVm {
    stack: Vec<i64>,
    memory: Vec<u8>,
    code: Vec<u8>,
    ip: usize,
    call_stack: Vec<(usize, usize)>, // (ret_ip, base_pointer)
    base_pointer: usize,
    heap_strings: Vec<String>,
    
    // Graphics
    #[cfg(feature = "minifb")]
    window: Option<Window>,
    fb_width: usize,
    fb_height: usize,
    fb_addr: usize,
}

impl NuxVm {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            stack: Vec::with_capacity(1024),
            memory: vec![0u8; 32 * 1024 * 1024], // Increased to 32MB for framebuffer
            code,
            ip: 0,
            call_stack: Vec::new(),
            base_pointer: 0,
            heap_strings: Vec::new(),
            #[cfg(feature = "minifb")]
            window: None,
            fb_width: 0,
            fb_height: 0,
            fb_addr: 0,
        }
    }
    
    fn read_i64(&mut self) -> i64 {
        if self.ip + 8 > self.code.len() { panic!("Unexpected EOF at {}", self.ip); }
        let bytes: [u8; 8] = self.code[self.ip..self.ip+8].try_into().unwrap();
        self.ip += 8;
        i64::from_le_bytes(bytes)
    }

    fn read_u8(&mut self) -> u8 {
        if self.ip >= self.code.len() { panic!("Unexpected EOF at {}", self.ip); }
        let b = self.code[self.ip];
        self.ip += 1;
        b
    }

    pub fn run(&mut self) {
        // Skip Header (64 bytes)
        if self.code.len() > 64 {
            self.ip = 64;
        } else {
            self.ip = 0;
        }

        loop {
            if self.ip >= self.code.len() { break; }
            let op = self.read_u8();

            match op {
                0xFF => break, // EXIT
                0x00 => {}, // NOP (0x00 is often NOP or padding)
                0x01 => { // PUSH
                    let val = self.read_i64();
                    self.stack.push(val);
                },
                0x02 => { // POP
                    if !self.stack.is_empty() { self.stack.pop(); }
                },
                0x10 => { // ADD
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a + b);
                },
                0x11 => { // SUB
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a - b);
                },
                0x12 => { // MUL
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a * b);
                },
                0x13 => { // DIV
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     if b == 0 { panic!("Division by zero"); }
                     self.stack.push(a / b);
                },
                0x90 => { // EQ
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a == b { 1 } else { 0 });
                },
                 0x91 => { // NEQ
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a != b { 1 } else { 0 });
                },
                 0x92 => { // LT
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a < b { 1 } else { 0 });
                },
                 0x93 => { // GT
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > b { 1 } else { 0 });
                },
                 0x94 => { // LTE
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a <= b { 1 } else { 0 });
                },
                0x95 => { // GTE
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a >= b { 1 } else { 0 });
                },
                0x60 => { // JMP
                    let dest = self.read_i64() as usize;
                    self.ip = dest; 
                },
                0x61 => { // JE (Jump if Equal usually means Jump if StackTop == 0 or comparison result false?)
                    // Compiler emits logical ops that result in 0 or 1.
                    // If (cond), it jumps to ELSE usually if false (0).
                    // Compiler: PUSH 0; JE label.
                    // Wait, `PUSH 0; JE` means `Jump If Equal to 0`? Or `Jump if Equal (CMP result)`?
                    // x86 JE jumps if ZF=1.
                    // Nux Compiler: `PUSH 0; JE label`.
                    // The intent is probably: "Jump if Top == 0".
                    let dest = self.read_i64() as usize;
                    let val = self.stack.pop().unwrap();
                    if val == 0 {
                        self.ip = dest;
                    }
                },
                0x70 => { // CALL
                    let dest = self.read_i64() as usize;
                    let num_args = self.read_i64() as usize;
                    
                    self.call_stack.push((self.ip, self.base_pointer));
                    // Simple Frame: BP = Stack.len() - NumArgs.
                    if self.stack.len() < num_args { panic!("Stack underflow on call"); }
                    self.base_pointer = self.stack.len() - num_args;
                    self.ip = dest;
                },
                0x71 => { // RET
                    if let Some((ret_ip, old_bp)) = self.call_stack.pop() {
                        self.ip = ret_ip;
                        // Return Value Convention: Top of stack.
                        let ret_val = self.stack.pop().unwrap_or(0);
                        // Clean Stack Frame
                        self.stack.truncate(self.base_pointer);
                        self.stack.push(ret_val);
                        self.base_pointer = old_bp;
                    } else {
                        break; // Return from main
                    }
                },
                0x51 => { // PRINT_CHAR
                    let val = self.stack.pop().unwrap_or(0);
                    print!("{}", (val as u8) as char);
                    io::stdout().flush().unwrap();
                },
                0x53 => { // PRINT_VAL
                    let val = self.stack.pop().unwrap_or(0);
                    // Check if string pointer? 
                    // No, PRINT_VAL is for numbers. Strings use loop of PRINT_CHAR or hacks.
                    // But our `println(s)` prints string.
                    // If `s` is ID, `PRINT_VAL` prints ID.
                    // We need `PRINT_STR`?
                    // `io.nux/print(val)`: `asm { val; PRINT_VAL }`.
                    // Does `compiler.rs` emit `PRINT_VAL` for strings?
                    // Compiler: `if t == Type::Float { PRINT_FLOAT } else { PRINT_VAL }`
                    // So generic `print(s)` calls `asm { s; PRINT_VAL }`.
                    // If `s` is a String ID, we should print String?
                    // But `PRINT_VAL` just prints number.
                    // We might need to detect if it's a string ID.
                    // Simpler: Just print number for now. `println` is for debugging mostly.
                    print!("{}", val);
                    io::stdout().flush().unwrap();
                },
                0x54 => { // PRINT_FLOAT
                    let val = self.stack.pop().unwrap_or(0);
                    let f = f64::from_bits(val as u64);
                    print!("{:.4}", f);
                    io::stdout().flush().unwrap();
                },
                0x68 => { // OP_PUSH_STR
                    // String Construction
                    let next_op = self.read_u8();
                    if next_op != 0x01 { panic!("Expected PUSH len after OP_PUSH_STR"); }
                    let len = self.read_i64() as usize;
                    
                    let mut s_bytes = Vec::new();
                    for _ in 0..len {
                         s_bytes.push(self.read_u8());
                    }
                    
                    let s = String::from_utf8(s_bytes).unwrap_or(String::from("?"));
                    self.heap_strings.push(s);
                    // Encode as a "pointer" (index).
                    // In a real VM, we might tag it.
                    // To make it distinct, let's use a very large number offset?
                    // Or just index.
                    self.stack.push((self.heap_strings.len() - 1) as i64);
                },
                0x69 => { // TO_UPPER
                    let id = self.stack.pop().unwrap() as usize;
                    if id < self.heap_strings.len() {
                         let s = self.heap_strings[id].to_uppercase();
                         self.heap_strings.push(s);
                         self.stack.push((self.heap_strings.len() - 1) as i64);
                    } else { self.stack.push(0); }
                },
                0x6A => { // TO_LOWER
                    let id = self.stack.pop().unwrap() as usize;
                    if id < self.heap_strings.len() {
                         let s = self.heap_strings[id].to_lowercase();
                         self.heap_strings.push(s);
                         self.stack.push((self.heap_strings.len() - 1) as i64);
                    } else { self.stack.push(0); }
                },
                0x44 => { // OP_GET_LOCAL
                    let offset = self.read_i64() as usize;
                    let idx = self.base_pointer + offset;
                    let val = if idx < self.stack.len() { self.stack[idx] } else { 0 };
                    self.stack.push(val);
                },
                0x45 => { // SET_LOCAL
                     let offset = self.read_i64() as usize;
                     let val = self.stack.pop().unwrap();
                     let idx = self.base_pointer + offset;
                     if idx < self.stack.len() { self.stack[idx] = val; }
                },
                0x2C => { // FSQRT
                     let val = self.stack.pop().unwrap();
                     let f = f64::from_bits(val as u64);
                     let r = f.sqrt();
                     self.stack.push(r.to_bits() as i64);
                },
                0x1E => { // ITOF
                     let i = self.stack.pop().unwrap();
                     let f = i as f64;
                     self.stack.push(f.to_bits() as i64);
                },
                0x1F => { // FTOI
                     let v = self.stack.pop().unwrap();
                     let f = f64::from_bits(v as u64);
                     self.stack.push(f as i64);
                },
                0x2A => { // FSIN
                     let val = self.stack.pop().unwrap();
                     let f = f64::from_bits(val as u64);
                     let r = f.sin();
                     self.stack.push(r.to_bits() as i64);
                },
                0x2B => { // FCOS
                     let val = self.stack.pop().unwrap();
                     let f = f64::from_bits(val as u64);
                     let r = f.cos();
                     self.stack.push(r.to_bits() as i64);
                },
                0x40 => { // PEEK
                     let addr = self.stack.pop().unwrap() as usize;
                     if addr + 8 <= self.memory.len() {
                         let bytes: [u8; 8] = self.memory[addr..addr+8].try_into().unwrap();
                         self.stack.push(i64::from_le_bytes(bytes));
                     } else {
                         self.stack.push(0);
                     }
                },
                0x41 => { // POKE
                     let val = self.stack.pop().unwrap();
                     let addr = self.stack.pop().unwrap() as usize;
                     if addr + 8 <= self.memory.len() {
                         let bytes = val.to_le_bytes();
                         self.memory[addr..addr+8].copy_from_slice(&bytes);
                     }
                },
                
                // Graphics / VBE Opcodes
                0x3A => { // OP_VBE_SET_MODE
                    #[cfg(feature = "minifb")]
                    {
                         let _bpp = self.stack.pop().unwrap(); // Assume 32 for now
                         let height = self.stack.pop().unwrap() as usize;
                         let width = self.stack.pop().unwrap() as usize;
                         
                         self.fb_width = width;
                         self.fb_height = height;
                         
                         // Allocate FB in VM memory at a safe offset (e.g. 8MB)
                         // 10MB Offset for safety
                         let fb_offset = 10 * 1024 * 1024;
                         self.fb_addr = fb_offset;
                         
                         // Ensure memory is large enough
                         let required = fb_offset + (width * height * 4);
                         if self.memory.len() < required {
                             self.memory.resize(required, 0);
                         }
                         
                         let mut window = Window::new(
                             "Nux Standalone Window",
                             width,
                             height,
                             WindowOptions::default(),
                         ).unwrap_or_else(|e| {
                             panic!("{}", e);
                         });
                         
                         window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
                         self.window = Some(window);
                    }
                    #[cfg(not(feature = "minifb"))]
                    {
                        // Mock implementation for non-minifb builds
                        let _bpp = self.stack.pop().unwrap();
                        let height = self.stack.pop().unwrap() as usize;
                        let width = self.stack.pop().unwrap() as usize;
                        self.fb_width = width;
                        self.fb_height = height;
                        self.fb_addr = 10 * 1024 * 1024;
                    }
                },
                0x3B => { // OP_VBE_GET_FB
                     self.stack.push(self.fb_addr as i64);
                },
                0x3C => { // OP_VBE_UPDATE
                    #[cfg(feature = "minifb")]
                    if let Some(window) = &mut self.window {
                        if !window.is_open() && !window.is_key_down(minifb::Key::Escape) {
                            // Should exit?
                        } else {
                            // Copy from VM memory to temp u32 buffer
                            let len = self.fb_width * self.fb_height;
                            let mut buffer = vec![0u32; len];
                            
                            // Convert u8 (RGBA/BGRA) to u32
                            // Nux assumes 32-bit integer per pixel.
                            // If user wrote `0xAARRGGBB` to memory, it is stored as LE bytes.
                            
                            // Optimization: Direct cast if possible, but alignment might differ.
                            // Let's iterate.
                            for i in 0..len {
                                let addr = self.fb_addr + i * 4;
                                let b = self.memory[addr];
                                let g = self.memory[addr+1];
                                let r = self.memory[addr+2];
                                let a = self.memory[addr+3]; 
                                // Minifb expects 00RRGGBB (or ARGB?)
                                // usually 0x00RRGGBB.
                                buffer[i] = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                            }
                            
                            window.update_with_buffer(&buffer, self.fb_width, self.fb_height).unwrap();
                        }
                    }
                },
                
                // Image Intrinsics (Mock/Simple)
                0x31 => { // OP_IMG_ALLOC
                    let h = self.stack.pop().unwrap() as usize;
                    let w = self.stack.pop().unwrap() as usize;
                    // Just return an address in memory.
                    // Simple "bump" allocator from end of FB? or just use `malloc` if we had one.
                    // For now, let's just use a fixed region after FB?
                    // This is unsafe for real usage without a malloc.
                    // Returning 0 for now as valid address (start of memory) is dangerous.
                    // Let's assume user manages memory or uses specific reserved regions.
                    // Returning a fake handle (index) is safer if we managed `Vec<Image>`.
                    // But `gfxdriver` expects pointer?
                    // `graphics.nux` assumes `id` is returned.
                    
                    // Hack: Just return a unique ID and store in a map?
                    // `gfxdriver`: img_new -> img_alloc.
                    // `draw` -> img_draw.
                    // Let's unimplemented for now or just return 0 to prevent crash.
                    self.stack.push(0); 
                },
                
                _ => { 
                    // eprintln!("Unknown OpCode: 0x{:02X} at {}", op, self.ip - 1);
                }
            }
        }
    }
}
