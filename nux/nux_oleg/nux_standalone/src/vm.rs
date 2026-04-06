use std::vec::{Vec, self};
use std::collections::BTreeMap;
use std::string::String;
use std::sync::{Arc, Mutex, MutexGuard};
use std::cell::UnsafeCell;
// use crate::fs::vfs::{FileHandle, Inode}; // Mock VFS for now?

// Mock VFS Trait until we port FS
pub trait FileHandle: Send + Sync {
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, ()>;
    fn write(&self, buf: &[u8], offset: usize) -> Result<usize, ()>;
}

macro_rules! kprint {
    ($($arg:tt)*) => ({
        print!($($arg)*);
    });
}

macro_rules! kprintln {
    () => (println!(""));
    ($($arg:tt)*) => ({
        println!($($arg)*);
    });
}

macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => ({
        kprint!($($arg)*);
        kprint!("\n");
    });
}

// ... constants ...
const OP_PUSH: u8 = 0x01;
const OP_POP: u8 = 0x02;
// ... arithmetic ...
const OP_ADD: u8 = 0x10;
const OP_SUB: u8 = 0x11;
const OP_MUL: u8 = 0x12;
const OP_DIV: u8 = 0x13;
const OP_MOD: u8 = 0x14;
const OP_POW: u8 = 0x15;
const OP_FLOORDIV: u8 = 0x16;
const OP_AND: u8 = 0x18;
const OP_OR:  u8 = 0x19; 
const OP_XOR: u8 = 0x22;
const OP_XAND: u8 = 0x23; // XNOR
const OP_XNOT: u8 = 0x24; // Bitwise NOT 
const OP_EQ: u8 = 0x90;
const OP_NEQ: u8 = 0x91;
const OP_LT: u8 = 0x92;
const OP_GT: u8 = 0x93;
const OP_LTE: u8 = 0x94;
const OP_GTE: u8 = 0x95;

const OP_DRAW_RECT: u8 = 0x20;
const OP_DRAW_IMG: u8 = 0x21; // Unused
const OP_SLEEP: u8 = 0x30;

const OP_DEBUG_PRINT: u8 = 0x50;
const OP_PRINT_CHAR: u8 = 0x51;
const OP_INPUT: u8 = 0x52;
const OP_PRINT_VAL: u8 = 0x53; // Prints i64
const OP_PRINT_FLOAT: u8 = 0x54; // Prints f64

// Float Ops
const OP_FADD: u8 = 0x1A;
const OP_FSUB: u8 = 0x1B;
const OP_FMUL: u8 = 0x1C;
const OP_FDIV: u8 = 0x1D;
const OP_ITOF: u8 = 0x1E; // Int to Float
const OP_FTOI: u8 = 0x1F; // Float to Int

const OP_PEEK: u8 = 0x40;
const OP_POKE: u8 = 0x41;
// Legacy GFX (Direct)
const OP_GFX_TEXT: u8 = 0x3C;
const OP_GFX_RECT: u8 = 0x3D;

// 42/43 PEEK8/POKE8 unused
const OP_GET_LOCAL: u8 = 0x44;
const OP_SET_LOCAL: u8 = 0x45;
const OP_FPOW: u8 = 0x46;
const OP_FFLOORDIV: u8 = 0x47;

const OP_JMP: u8 = 0x60;
const OP_JE: u8 = 0x61;
// const OP_JNE: u8 = 0x62; // Future?

const OP_CALL: u8 = 0x70;
const OP_RET: u8 = 0x71;
const OP_SPAWN: u8 = 0x72; // NEW: Spawn Thread
const OP_LOCK: u8 = 0x73;  // NEW: Acquire Lock (Simple Global Lock or ID?)
const OP_UNLOCK: u8 = 0x74; // NEW: Release Lock

const OP_KERNEL_OP: u8 = 0x80;
const OP_SYSTEM: u8 = 0x81; // NEW: Execute System Command
const OP_EXIT: u8 = 0xFF;

// File I/O Ops
const OP_FILE_OPEN: u8 = 0x55;
const OP_FILE_CLOSE: u8 = 0x56;
const OP_FILE_READ: u8 = 0x57;
const OP_FILE_WRITE: u8 = 0x58;
const OP_FILE_EXISTS: u8 = 0x59;
const OP_FILE_MKDIR: u8 = 0x5A;
const OP_FILE_DELETE: u8 = 0x5C;

// Time & Random Ops
const OP_TIME: u8 = 0x75;
const OP_RANDOM: u8 = 0x76;

// DataManager & Security Opcodes
const OP_DM_GET: u8 = 0x64;
const OP_DM_SET: u8 = 0x65;
const OP_SEC_LOGIN: u8 = 0x66;
const OP_SEC_WHOAMI: u8 = 0x67;
const OP_PUSH_STR: u8 = 0x68; // Push string literal

// Wrapper for Mutex to mimic SpinLock API (no Result)
pub struct SpinLock<T> {
    inner: Mutex<T>,
}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self { inner: Mutex::new(data) }
    }
    
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }
}


// Shared State for all threads
struct SharedState {
    memory: Vec<u8>, // Global Virtual Memory (Heap/Globals)
    locks: BTreeMap<u64, Arc<SpinLock<()>>>,
    files: BTreeMap<u64, Arc<dyn FileHandle>>, // Open File Handles
    next_fd: u64,
    rng_state: u64,
}

#[derive(Clone)]
pub struct NuxVm {
    // Thread-Local State
    stack: Vec<i64>,
    ip: usize,
    fp: usize, // Frame Pointer
    call_stack: Vec<(usize, usize)>, // (ret_ip, ret_fp)
    running: bool,
    
    // Shared State
    code: Arc<Vec<u8>>,
    shared: Arc<SpinLock<SharedState>>,
}

impl NuxVm {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            stack: Vec::with_capacity(256),
            ip: 0,
            fp: 0,
            call_stack: Vec::with_capacity(32),
            running: false,
            code: Arc::new(code),
            shared: Arc::new(SpinLock::new(SharedState {
                memory: vec![0u8; 1024 * 64], // 64KB Shared Memory
                locks: BTreeMap::new(),
                files: BTreeMap::new(),
                next_fd: 1,
                rng_state: 0xCAFEBABE, // Initial Seed
            })),
        }
    }
    
    // New internal constructor for cloning shared state
    fn fork(&self, start_ip: usize) -> Self {
        Self {
            stack: Vec::with_capacity(256),
            ip: start_ip,
            fp: 0, 
            call_stack: Vec::with_capacity(32),
            running: true,
            code: self.code.clone(),
            shared: self.shared.clone(),
        }
    }

    pub fn push(&mut self, val: i64) {
        if self.stack.len() >= 1024 {
            kprintln!("Runtime Error: Stack Overflow");
            self.running = false;
            return;
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> i64 {
        if self.stack.is_empty() {
             kprintln!("Runtime Error: Stack Underflow");
             self.running = false;
             return 0;
        }
        self.stack.pop().unwrap_or(0)
    }
    
    fn read_i64_code(&mut self) -> i64 {
         if self.ip + 8 > self.code.len() { return 0; }
         let bytes = &self.code[self.ip..self.ip+8];
         let val = i64::from_le_bytes(bytes.try_into().unwrap());
         self.ip += 8;
         val
    }

    pub fn run(&mut self) {
        // Only check header if starting from 0 (main thread)
        // Sub-threads start at specific function.
        if self.ip == 0 {
             if self.code.len() < 64 || &self.code[0..4] != b"ANUX" {
                 kprintln!("NuxVM: Invalid Binary");
                 return;
             }
             self.ip = 64; 
        }
        
        self.running = true;

        while self.running && self.ip < self.code.len() {
            let op = self.code[self.ip];
            self.ip += 1;

            match op {
                OP_PUSH => {
                    let val = self.read_i64_code();
                    self.push(val);
                },
                OP_PUSH_STR => {
                    // Read string length (i64)
                    let len = self.read_i64_code() as usize;
                    
                    // Read string bytes from bytecode
                    let mut str_bytes = Vec::new();
                    for _ in 0..len {
                        if self.ip < self.code.len() {
                            str_bytes.push(self.code[self.ip]);
                            self.ip += 1;
                        }
                    }
                    
                    // Allocate in shared memory (use a simple bump allocator at end of memory)
                    let mut shared = self.shared.lock();
                    let dest_addr = shared.memory.len() - 2048; // Reserve 2KB at end for string pool
                    
                    // Write string to shared memory
                    let max_copy = core::cmp::min(len, 1024); // Limit to 1KB per string
                    for i in 0..max_copy {
                        shared.memory[dest_addr + i] = str_bytes[i];
                    }
                    shared.memory[dest_addr + max_copy] = 0; // Null terminator
                    drop(shared);
                    
                    // Push address
                    self.push(dest_addr as i64);
                },
                OP_POP => { self.pop(); },
                OP_ADD => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_add(b)); },
                OP_SUB => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_sub(b)); },
                OP_MUL => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_mul(b)); },
                OP_DIV => { 
                    let b = self.pop(); let a = self.pop(); 
                    if b == 0 { kprintln!("Runtime Error: DivZero"); self.running = false; }
                    else { self.push(a.wrapping_div(b)); }
                },
                OP_MOD => { let b = self.pop(); let a = self.pop(); if b!=0 { self.push(a%b); } else { self.push(0); } },
                OP_POW => {
                    let b = self.pop();
                    let a = self.pop();
                    // Use i64::pow for positive exponents, handle negative separately
                    if b >= 0 && b <= u32::MAX as i64 {
                        self.push(a.pow(b as u32));
                    } else if b < 0 {
                        // Negative exponent: convert to float
                        // let result = (a as f64).powf(b as f64);
                        // self.push(result as i64);
                        kprintln!("Runtime Warning: Negative POW not supported in kernel yet");
                        self.push(0);
                    } else {
                        self.push(0); // Overflow protection
                    }
                },
                OP_FLOORDIV => {
                    let b = self.pop();
                    let a = self.pop();
                    if b == 0 {
                        kprintln!("Runtime Error: DivZero");
                        self.running = false;
                    } else {
                        // Floor division: a // b = floor(a / b)
                        self.push(a.div_euclid(b));
                    }
                },
                
                // Float Ops
                OP_FADD => { 
                    let b = f64::from_bits(self.pop() as u64); 
                    let a = f64::from_bits(self.pop() as u64); 
                    self.push((a + b).to_bits() as i64); 
                },
                OP_FSUB => { 
                    let b = f64::from_bits(self.pop() as u64); 
                    let a = f64::from_bits(self.pop() as u64); 
                    self.push((a - b).to_bits() as i64); 
                },
                OP_FMUL => { 
                    let b = f64::from_bits(self.pop() as u64); 
                    let a = f64::from_bits(self.pop() as u64); 
                    self.push((a * b).to_bits() as i64); 
                },
                OP_FDIV => { 
                    let b = f64::from_bits(self.pop() as u64); 
                    let a = f64::from_bits(self.pop() as u64); 
                    self.push((a / b).to_bits() as i64); 
                },
                OP_FPOW => {
                    let _b = f64::from_bits(self.pop() as u64);
                    let _a = f64::from_bits(self.pop() as u64);
                    // self.push(a.powf(b).to_bits() as i64);
                    kprintln!("Runtime Warning: FPOW not supported");
                    self.push(0);
                },
                OP_FFLOORDIV => {
                    let b = f64::from_bits(self.pop() as u64);
                    let a = f64::from_bits(self.pop() as u64);
                    // self.push((a / b).floor().to_bits() as i64);
                    self.push(((a / b) as i64 as f64).to_bits() as i64); // Hack for no_std floor
                },
                OP_ITOF => {
                    let a = self.pop();
                    self.push((a as f64).to_bits() as i64);
                },
                OP_FTOI => {
                    let a = f64::from_bits(self.pop() as u64);
                    self.push(a as i64);
                },
                OP_PRINT_FLOAT => {
                    let val = f64::from_bits(self.pop() as u64);
                    // kprint!("{}", val);
                    // io::stdout().flush().unwrap();
                    kprintln!("FLOAT: {}", val as i64); // Todo: proper float print
                },
                
                OP_EQ => { let b = self.pop(); let a = self.pop(); self.push(if a == b {1} else {0}); },
                OP_NEQ => { let b = self.pop(); let a = self.pop(); self.push(if a != b {1} else {0}); },
                OP_LT => { let b = self.pop(); let a = self.pop(); self.push(if a < b {1} else {0}); },
                OP_GT => { let b = self.pop(); let a = self.pop(); self.push(if a > b {1} else {0}); },
                OP_LTE => { let b = self.pop(); let a = self.pop(); self.push(if a <= b {1} else {0}); },
                OP_GTE => { let b = self.pop(); let a = self.pop(); self.push(if a >= b {1} else {0}); },
                
                OP_AND => { let b = self.pop(); let a = self.pop(); self.push(if a!=0 && b!=0 {1} else {0}); },
                OP_OR => { let b = self.pop(); let a = self.pop(); self.push(if a!=0 || b!=0 {1} else {0}); },

                // GC Ops
                0x5B => { // OP_VM_STACK_COPY
                    let dest_ptr = self.pop() as usize;
                    let count = self.stack.len();
                    
                     let mut shared = self.shared.lock();
                     let mem = &mut shared.memory;
                     let max_len = mem.len();
                     
                     if dest_ptr + (count * 8) <= max_len {
                         for (i, &val) in self.stack.iter().enumerate() {
                             let addr = dest_ptr + (i * 8);
                             // Write i64 as LE bytes
                             let bytes = val.to_le_bytes();
                             for j in 0..8 {
                                 mem[addr + j] = bytes[j];
                             }
                         }
                         drop(shared);
                         self.push(count as i64); // Return count
                     } else {
                         drop(shared);
                         kprintln!("VM Stack Copy Overflow: Dest {}, Count {}", dest_ptr, count);
                         self.push(-1); 
                     }
                },

                OP_SLEEP => {
                    let ms = self.pop();
                    if ms > 0 {
                        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                    }
                },
                OP_DEBUG_PRINT => { let val = self.pop(); kprintln!("DEBUG: {}", val); },
                OP_PRINT_CHAR => { 
                    let val = self.pop(); 
                    kprint!("{}", val as u8 as char);  
                },
                OP_PRINT_VAL => { let val = self.pop(); kprint!("{}", val); },
                
                OP_INPUT => {
                   let mut _buffer = String::new();
                   // Kernel Input TODO
                   self.push(0); 
                },

                OP_JMP => { let t = self.read_i64_code(); self.ip = t as usize; },
                OP_JE => { 
                    let t = self.read_i64_code(); 
                    let b = self.pop(); let a = self.pop(); 
                    if a == b { self.ip = t as usize; }
                },
                
                OP_CALL => {
                    let t = self.read_i64_code();
                    let num_args = self.read_i64_code(); // New generic arg
                    
                    if self.call_stack.len() >= 256 {
                        kprintln!("Runtime Error: Call Stack Overflow (Recursion too deep)");
                        self.running = false;
                    } else {
                        self.call_stack.push((self.ip, self.fp));
                        // Frame starts at the first argument
                        // Stack: [..., Arg0, Arg1] < Top
                        // FP = Len - 2
                        if (self.stack.len() as i64) < num_args {
                             kprintln!("Runtime Error: Stack Underflow on Call");
                             self.running = false;
                        } else {
                             self.fp = self.stack.len() - (num_args as usize);
                             self.ip = t as usize;
                        }
                    }
                },
                OP_RET => {
                    if let Some((ret_ip, ret_fp)) = self.call_stack.pop() { 
                        // Preserve return value
                        let ret_val = self.pop();
                        // Restore stack (discard locals)
                        if self.stack.len() > self.fp {
                            self.stack.truncate(self.fp);
                        }
                        self.push(ret_val);
                        
                        self.ip = ret_ip; 
                        self.fp = ret_fp;
                    }
                    else { self.running = false; }
                },

                OP_GET_LOCAL => {
                     let offset = self.read_i64_code();
                     let idx = (self.fp as i64 + offset) as usize;
                     if idx < self.stack.len() {
                         self.push(self.stack[idx]);
                     } else {
                         kprintln!("Runtime Error: Stack Invalid Access Local {}", offset);
                         self.running = false;
                     }
                },
                OP_SET_LOCAL => {
                     let offset = self.read_i64_code();
                     let idx = (self.fp as i64 + offset) as usize;
                     if idx < self.stack.len() {
                         let val = self.pop();
                         self.stack[idx] = val;
                     } else {
                         // If we are setting a local that hasn't been pushed yet (e.g. init), 
                         // compiler should have emitted PUSH 0.
                         // But if we are setting an ARG (negative offset), it must exist.
                         kprintln!("Runtime Error: Stack Invalid Write Local {}", offset);
                         self.running = false;
                     }
                },
                
                // --- THREADING_OPS ---
                OP_SPAWN => {
                    let target = self.read_i64_code(); // Function address
                    // Fork a VM instance
                    let mut child_vm = self.fork(target as usize);
                    
                    // Spawn OS Thread
                    // thread::spawn(move || {
                    //     child_vm.run();
                    // });
                    kprintln!("DEBUG: Spawning thread at {} (Not Implemented)", target);
                    child_vm.run(); // Run sync for now to avoid hang
                },
                // Locking ops (TODO: Implement proper ID-based locks if needed)
                OP_LOCK => { /* Placeholder */ },
                OP_UNLOCK => { /* Placeholder */ },

                OP_TIME => {
                    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
                    self.push(now as i64);
                },

                OP_GFX_RECT => {
                    let _col = self.pop() as u32;
                    let _h = self.pop() as usize;
                    let _w = self.pop() as usize;
                    let _y = self.pop() as usize;
                    let _x = self.pop() as usize;
                    // kprintln!("GFX_RECT: x={}, y={}, w={}, h={}, col={:X}", x, y, w, h, col);
                },

                OP_GFX_TEXT => {
                    let _col = self.pop() as u32;
                    let ptr = self.pop();
                    let _y = self.pop() as usize;
                    let _x = self.pop() as usize;
                    
                    // Read string
                    let shared = self.shared.lock(); // .unwrap()
                    let mut s = String::new();
                    let mem = &shared.memory; // simplified unwrap
                    let mut addr = ptr as usize;
                     while addr < mem.len() && mem[addr] != 0 {
                        s.push(mem[addr] as char);
                        addr += 1;
                    }
                    // kprintln!("GFX_TEXT: '{}' at {},{}", s, x, y);
                },

                OP_RANDOM => {
                     let mut shared = self.shared.lock();
                     let mut x = shared.rng_state;
                     // Xorshift64*
                     x ^= x << 12;
                     x ^= x >> 25;
                     x ^= x << 27;
                     shared.rng_state = x;
                     let res = x.wrapping_mul(0x2545F4914F6CDD1D);
                     drop(shared);
                     self.push(res as i64);
                },

                OP_KERNEL_OP => {
                    let op_id = self.pop();
                    match op_id {
                        1 => kprint!("\x1B[2J\x1B[1;1H"),
                        2 => kprintln!("NuxVM Kernel-Mode v0.5"),
                        _ => {},
                    }
                },
                
                OP_SYSTEM => {
                    let ptr = self.pop();
                    // Read string from memory
                    let mut cmd = String::new();
                    let shared = self.shared.lock();
                    let mem = &shared.memory;
                    let mut addr = ptr as usize;
                    while addr < mem.len() && mem[addr] != 0 {
                        cmd.push(mem[addr] as char);
                        addr += 1;
                    }
                    drop(shared); // Unlock
                    
                    kprintln!("System Command: {}", cmd);
                    // In kernel, this might spawn a task. In portable, use std::process.
                    {
                        // Stub
                         kprintln!("System command (Stub): {}", cmd);
                    }
                    self.push(0); // Return Success
                },
                
                // Opcode 0xB5: Vision Detect (Mock)
                0xB5 => {
                    let _handle = self.pop();
                    // Simulate processing time
                    // Simulation delay removed for no_std compatibility
                    // unsafe { let mut x = 0; for _ in 0..1000000 { x += 1; core::ptr::read_volatile(&x); } }
                    
                    self.push(1); // Found 1 object
                },
                
                OP_GFX_RECT => {
                    let col = self.pop() as u32;
                    let h = self.pop() as usize;
                    let w = self.pop() as usize;
                    let y = self.pop() as usize;
                    let _x = self.pop() as usize;
                    // crate::drivers::video::draw_rect(x, y, w, h, col);
                },

                OP_GFX_TEXT => {
                    let col = self.pop() as u32;
                    let ptr = self.pop();
                    let y = self.pop() as usize;
                    let mut x = self.pop() as usize;
                    
                    // Read string from shared memory
                    let shared = self.shared.lock();
                    let mem = &shared.memory;
                    let mut addr = ptr as usize;
                    
                    while addr < mem.len() && mem[addr] != 0 {
                        let c = mem[addr] as char;
                        // crate::drivers::video::draw_char_raw(x, y, c, col);
                        x += 8; // Advance cursor 8 pixels
                        addr += 1;
                    }
                },
                
                // Memory Ops (Thread-Safe via Mutex)
                OP_PEEK => {
                    let addr = self.pop();
                    let shared = self.shared.clone(); // Clone Arc to avoid borrowing self
                    let val_opt = {
                        let state = shared.lock();
                        if addr < 0 || addr as usize + 8 > state.memory.len() {
                             None
                        } else {
                             let bytes = &state.memory[addr as usize .. addr as usize + 8];
                             Some(i64::from_le_bytes(bytes.try_into().unwrap()))
                        }
                    };
                    
                    if let Some(val) = val_opt {
                        self.push(val);
                    } else {
                        kprintln!("Runtime Error: Segfault Read {}", addr); 
                        self.running = false;
                    }
                },
                OP_POKE => {
                    let val = self.pop();
                    let addr = self.pop();
                    let shared = self.shared.clone();
                    let success = {
                        let mut state = shared.lock();
                        if addr < 0 || addr as usize + 8 > state.memory.len() {
                             false
                        } else {
                             let bytes = val.to_le_bytes();
                             state.memory[addr as usize .. addr as usize + 8].copy_from_slice(&bytes);
                             true
                        }
                    };
                    
                    if !success {
                        kprintln!("Runtime Error: Segfault Write {}", addr);
                        self.running = false;
                    }
                },
                
                // File I/O Ops
                OP_FILE_OPEN => {
                    let _ptr = self.pop();
                    // Helper to read str
                    // ...
                    kprintln!("FILE_OPEN not supported in standalone yet");
                    self.push(-1);
                },
                OP_FILE_CLOSE => {
                    let fd = self.pop() as u64;
                    let mut shared = self.shared.lock();
                    let success = shared.files.remove(&fd).is_some();
                    drop(shared);
                    
                    if success {
                        self.push(1); // Success
                    } else {
                        self.push(0); // Fail
                    }
                },
                OP_FILE_READ => {
                    let len = self.pop() as usize;
                    let buf_ptr = self.pop() as usize;
                    let fd = self.pop() as u64;
                    
                    let mut shared = self.shared.lock();
                    if let Some(handle) = shared.files.get(&fd) {
                        let mut temp_buf = vec![0u8; len];
                        if let Ok(count) = handle.read(&mut temp_buf, 0) { // Offset 0 for now
                            // Copy back to memory
                            for i in 0..count {
                                if buf_ptr + i < shared.memory.len() {
                                    shared.memory[buf_ptr + i] = temp_buf[i];
                                }
                            }
                            drop(shared);
                            self.push(count as i64);
                        } else {
                            drop(shared);
                            self.push(-1); // Error
                        }
                    } else {
                        drop(shared);
                        self.push(-1); // Bad FD
                    }
                },
                OP_FILE_WRITE => {
                    let len = self.pop() as usize;
                    let buf_ptr = self.pop() as usize;
                    let fd = self.pop() as u64;
                    
                    let mut shared = self.shared.lock();
                    if let Some(handle) = shared.files.get(&fd) {
                        // Read from memory
                        let mut temp_buf: Vec<u8> = Vec::new(); // alloc::vec![0u8; len];
                        for i in 0..len {
                             if buf_ptr + i < shared.memory.len() {
                                 temp_buf.push(shared.memory[buf_ptr + i]);
                             } else {
                                 temp_buf.push(0);
                             }
                        }
                        
                        if let Ok(count) = handle.write(&temp_buf, 0) { // Offset 0
                            drop(shared);
                            self.push(count as i64);
                        } else {
                            drop(shared);
                            self.push(-1); // Error
                        }
                    } else {
                        drop(shared);
                        self.push(-1); // Bad FD
                    }
                },
                OP_FILE_EXISTS => {
                    let _ptr = self.pop();
                    self.push(0);
                },
                OP_FILE_MKDIR => {
                    let _ptr = self.pop();
                    self.push(0);
                },
                
                0x5C => { // OP_FILE_DELETE
                    let ptr = self.pop();
                    let mut path = String::new();
                    let shared = self.shared.lock();
                    let mem = &shared.memory;
                    let mut addr = ptr as usize;
                    while addr < mem.len() && mem[addr] != 0 {
                        path.push(mem[addr] as char);
                        addr += 1;
                    }
                    drop(shared);
                    
                    // VFS unlink/remove not yet implemented in Inode trait
                    // Stub:
                    kprintln!("OP_FILE_DELETE not supported (stub)");
                    self.push(0); // Fail
                    
                    /*
                    if let Ok(_) = crate::fs::vfs::root().unlink(&path) {
                         self.push(1);
                    } else {
                         self.push(0);
                    }
                    */
                },
                
                OP_SYSTEM => {
                    let ptr = self.pop();
                    let mut cmd = String::new();
                    let shared = self.shared.lock();
                    let mem = &shared.memory;
                    let mut addr = ptr as usize;
                    while addr < mem.len() && mem[addr] != 0 {
                        cmd.push(mem[addr] as char);
                        addr += 1;
                    }
                    drop(shared);
                    
                    kprintln!("System command (Stub): {}", cmd);
                    self.push(0); // Return Success
                },
                
                // --- DataManager Ops ---
                OP_DM_GET => {
                    let _ptr = self.pop();
                     // Stub: Return null
                     self.push(0);
                },
                
                OP_DM_SET => {
                    let _val_ptr = self.pop();
                    let _key_ptr = self.pop();
                    // Stub
                    self.push(0); 
                },
                
                // --- Security Ops ---
                OP_SEC_LOGIN => {
                    let _pass_ptr = self.pop();
                    let _user_ptr = self.pop();
                    // Stub: Always fail login in standalone
                    self.push(0);
                },
                
                OP_SEC_WHOAMI => {
                   // Return "user"
                    let name = "user"; 
                    let mut shared = self.shared.lock();
                    let dest_addr = shared.memory.len() - 1024; 
                    let val_bytes = name.as_bytes();
                    let len = core::cmp::min(val_bytes.len(), 1023);
                    
                    for i in 0..len {
                        shared.memory[dest_addr + i] = val_bytes[i];
                    }
                    shared.memory[dest_addr + len] = 0;
                    drop(shared);
                    
                    self.push(dest_addr as i64);
                },

                // Opcode 0xB5: Vision Detect (Mock)
                OP_EXIT => { self.running = false; },
                _ => { kprintln!("Unknown Opcode: {:02X}", op); }
            }
        }
    }
}
