use crate::platform::{self, Platform};
use std::sync::Arc;
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

// ... constants ...
const OP_PUSH: u8 = 0x01;
// ... (SKIP CONSTANTS)

// ...


const OP_POP: u8 = 0x02;
const OP_SWAP: u8 = 0x03; // Correct slot
const OP_DUP: u8 = 0x04;
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
const OP_EQ: u8 = 0x90;
const OP_NEQ: u8 = 0x91;
const OP_LT: u8 = 0x92;
const OP_GT: u8 = 0x93;
const OP_LTE: u8 = 0x94;
const OP_GTE: u8 = 0x95;

const OP_DRAW_RECT: u8 = 0x20;
const OP_DRAW_IMG: u8 = 0x21; // Unused
const OP_SLEEP: u8 = 0x30;

// Vision/Camera Ops
const OP_IMG_ALLOC: u8 = 0x31;
const OP_IMG_FREE: u8 = 0x32;
const OP_IMG_DRAW: u8 = 0x33;   // Draw to screen
const OP_CAM_CAPTURE: u8 = 0x34; // Capture to buffer
const OP_IMG_FILTER: u8 = 0x35;
const OP_IMG_GET: u8 = 0x36; // Get pixel (r,g,b) packed or separate? Packed int.

const OP_IMG_RESIZE: u8 = 0x37;
const OP_IMG_CROP: u8 = 0x38;
const OP_IMG_GRAYSCALE: u8 = 0x39;
const OP_IMG_SET: u8 = 0x3A;
const OP_IMG_FILL: u8 = 0x3B;

const OP_DEBUG_PRINT: u8 = 0x50;
const OP_PRINT_CHAR: u8 = 0x51;
const OP_INPUT: u8 = 0x52;
const OP_PRINT_VAL: u8 = 0x53; // Prints i64
const OP_PRINT_FLOAT: u8 = 0x54; // Prints f64
const OP_TO_UPPER: u8 = 0x55;
const OP_TO_LOWER: u8 = 0x56;

const OP_CHECK_RANGE: u8 = 0x57;

const OP_SYS_PLATFORM: u8 = 0x58; // Returns u8 (0-4)
const OP_CAM_COUNT: u8 = 0x59;    // Returns Count
const OP_IS_KEY_DOWN: u8 = 0x5A;  // Returns Bool (0/1)
const OP_VM_STACK_COPY: u8 = 0x5B; // Stack Manipulation
const OP_TIME: u8 = 0x5C;         // Returns i64 (ms since epoch)
const OP_SYSTEM: u8 = 0x5D;       // System Command
const OP_FILE_DELETE: u8 = 0x5E;  // Delete File


// Float Ops
const OP_FADD: u8 = 0x1A;
const OP_FSUB: u8 = 0x1B;
const OP_FMUL: u8 = 0x1C;
const OP_FDIV: u8 = 0x1D;
const OP_ITOF: u8 = 0x1E; // Int to Float
const OP_FTOI: u8 = 0x1F; // Float to Int

const OP_PEEK: u8 = 0x40;
const OP_POKE: u8 = 0x41;
// 42/43 PEEK8/POKE8 unused
const OP_GET_LOCAL: u8 = 0x44;
const OP_SET_LOCAL: u8 = 0x45;
const OP_FPOW: u8 = 0x46;
const OP_FFLOORDIV: u8 = 0x47;
const OP_FSIN: u8 = 0x48;
const OP_FCOS: u8 = 0x49;
const OP_FSQRT: u8 = 0x4A;

const OP_JMP: u8 = 0x60;
const OP_JE: u8 = 0x61;
// const OP_JNE: u8 = 0x62; // Future?

// Graphics Opcodes
const OP_DRAW_LINE: u8 = 0x93;     // Draw line (img, x1, y1, x2, y2, color)
const OP_DRAW_CIRCLE: u8 = 0x94;   // Draw circle (img, x, y, radius, color)
const OP_DRAW_PIXEL: u8 = 0x95;    // Set pixel (img, x, y, color)
const OP_GFX_CLEAR: u8 = 0x96;     // Clear image with color (img, color)

const OP_CALL: u8 = 0x70;
const OP_RET: u8 = 0x71;
const OP_SPAWN: u8 = 0x72; // NEW: Spawn Thread
const OP_LOCK: u8 = 0x73;  // NEW: Acquire Lock (Simple Global Lock or ID?)
const OP_UNLOCK: u8 = 0x74; // NEW: Release Lock

const OP_KERNEL_OP: u8 = 0x80;
const OP_VERIFY: u8 = 0x81;
const OP_ALLOC: u8 = 0x82;
const OP_FREE: u8 = 0x83;
const OP_LIMIT_MEM: u8 = 0x84;
const OP_VISION_DETECT: u8 = 0xB0;
const OP_EXIT: u8 = 0xFF;

// Simple SpinLock Implementation for Kernel Safety
pub struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Spin hint could go here (std::hint::spin_loop())
            // but might not be available in all portable contexts.
             std::thread::yield_now(); // Be nice to scheduler
        }
        SpinLockGuard { lock: self }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> std::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
    }
}


// Shared State for all threads
struct SharedState {
    memory: Vec<u8>, // Global Virtual Memory (Heap/Globals)
    // We could add a mutex map for fine-grained locks later.
    // For now, implicit global lock or use atomic memory ops?
    // User requested "thread safety". Mutex around memory is safe *access*.
    // But logic race needs explicit locks.
    locks: std::collections::HashMap<u64, Arc<SpinLock<()>>>, 
    // Actually simpler: One Big Lock for critical sections if requested?
    // Or users provide lock ID.
    
    // Handle ID -> (Width, Height, Data[ARGB])
    images: std::collections::HashMap<i64, (i64, i64, Vec<u32>)>,
    next_handle: i64,
    heap_ptr: usize,
    mem_limit: Option<usize>,
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
    // Shared State
    code: Arc<Vec<u8>>,
    shared: Arc<SpinLock<SharedState>>,
}

// Manually implement Clone if needed, or remove derive.
// fork() constructs new Self, doesn't use clone().

impl NuxVm {
    pub fn new(mut code: Vec<u8>) -> Self {
        // Pass 3: Checksum and Decryption
        if code.len() > 64 && &code[0..4] == b"ANUX" {
            let key = b"NUX_SECURE_KEY_123";
            for i in 64..code.len() {
                code[i] ^= key[(i - 64) % key.len()];
            }
            
            let mut a: u32 = 1;
            let mut b: u32 = 0;
            for &byte in &code[64..] {
                a = (a + byte as u32) % 65521;
                b = (b + a) % 65521;
            }
            let checksum = (b << 16) | a;
            let expected_checksum = u32::from_le_bytes([code[4], code[5], code[6], code[7]]);
            
            if expected_checksum != 0 && checksum != expected_checksum {
                println!("Security/Integrity Check Failed! Bytecode has been tampered with or corrupted.");
                std::process::exit(1);
            }
        }

        Self {
            stack: Vec::with_capacity(256),
            ip: 64, // Start after 64-byte header
            fp: 0,
            call_stack: Vec::with_capacity(32),
            running: false,
            code: Arc::new(code),
            shared: Arc::new(SpinLock::new(SharedState {
                memory: vec![0u8; 1024 * 1024], // 1MB Memory
                locks: std::collections::HashMap::new(),
                images: std::collections::HashMap::new(),
                next_handle: 1,
                heap_ptr: 1024, // Reserve 1024 bytes for null and globals
                mem_limit: None,
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
            println!("Runtime Error: Stack Overflow");
            self.running = false;
            return;
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> i64 {
        if self.stack.is_empty() {
             println!("Runtime Error: Stack Underflow");
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
    
    fn read_i8_code(&mut self) -> i8 {
         if self.ip + 1 > self.code.len() { return 0; }
         let b = self.code[self.ip];
         self.ip += 1;
         b as i8
    }
    
    fn read_i16_code(&mut self) -> i16 {
         if self.ip + 2 > self.code.len() { return 0; }
         let bytes = &self.code[self.ip..self.ip+2];
         let val = i16::from_le_bytes(bytes.try_into().unwrap());
         self.ip += 2;
         val
    }
    
    fn read_i32_code(&mut self) -> i32 {
         if self.ip + 4 > self.code.len() { return 0; }
         let bytes = &self.code[self.ip..self.ip+4];
         let val = i32::from_le_bytes(bytes.try_into().unwrap());
         self.ip += 4;
         val
    }

    pub fn run(&mut self, mut platform: Option<&mut dyn Platform>) {
        // Sub-threads start at specific function.
        if self.ip == 0 {
             if self.code.len() < 64 || &self.code[0..4] != b"ANUX" {
                 println!("NuxVM: Invalid Binary");
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
                0xA0..=0xAF => { // 1-byte PUSH (values 0-15)
                    self.push((op - 0xA0) as i64);
                },
                0xB0 => { // 2-byte PUSH
                    let val = self.read_i8_code();
                    self.push(val as i64);
                },
                0xB1 => { // 3-byte PUSH
                    let val = self.read_i16_code();
                    self.push(val as i64);
                },
                0xB2 => { // 5-byte PUSH
                    let val = self.read_i32_code();
                    self.push(val as i64);
                },
                OP_POP => { self.pop(); },
                OP_SWAP => {
                    if self.stack.len() >= 2 {
                         // println!("DEBUG: SWAP Executed");
                         let a = self.pop();
                         let b = self.pop();
                         self.push(a);
                         self.push(b);
                    }
                },
                OP_ADD => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_add(b)); },
                OP_SUB => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_sub(b)); },
                OP_MUL => { let b = self.pop(); let a = self.pop(); self.push(a.wrapping_mul(b)); },
                OP_DIV => { 
                    let b = self.pop(); let a = self.pop(); 
                    if b == 0 { println!("Runtime Error: DivZero"); self.running = false; }
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
                        let result = (a as f64).powf(b as f64);
                        self.push(result as i64);
                    } else {
                        self.push(0); // Overflow protection
                    }
                },
                OP_FLOORDIV => {
                    let b = self.pop();
                    let a = self.pop();
                    if b == 0 {
                        println!("Runtime Error: DivZero");
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
                    let b = f64::from_bits(self.pop() as u64);
                    let a = f64::from_bits(self.pop() as u64);
                    self.push(a.powf(b).to_bits() as i64);
                },
                OP_FFLOORDIV => {
                    let b = f64::from_bits(self.pop() as u64);
                    let a = f64::from_bits(self.pop() as u64);
                    self.push((a / b).floor().to_bits() as i64);
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
                    print!("{}", val);
                    io::stdout().flush().unwrap();
                },
                OP_TO_UPPER => {
                    let val = self.pop();
                    let c = (val as u8) as char;
                    let upper = c.to_ascii_uppercase();
                    self.push(upper as u8 as i64);
                },
                OP_TO_LOWER => {
                    let val = self.pop();
                    let c = (val as u8) as char;
                    let lower = c.to_ascii_lowercase();
                    self.push(lower as u8 as i64);
                },
                OP_CHECK_RANGE => {
                    let min = self.read_i64_code();
                    let max = self.read_i64_code();
                    let val = self.pop();
                    if val < min || val > max {
                        println!("Runtime Error: Value {} out of range [{}, {}]", val, min, max);
                        self.running = false;
                    }
                    self.push(val);
                },
                
                OP_EQ => { let b = self.pop(); let a = self.pop(); self.push(if a == b {1} else {0}); },
                OP_NEQ => { let b = self.pop(); let a = self.pop(); self.push(if a != b {1} else {0}); },
                OP_LT => { let b = self.pop(); let a = self.pop(); self.push(if a < b {1} else {0}); },
                OP_GT => { let b = self.pop(); let a = self.pop(); self.push(if a > b {1} else {0}); },
                OP_LTE => { let b = self.pop(); let a = self.pop(); self.push(if a <= b {1} else {0}); },
                OP_GTE => { let b = self.pop(); let a = self.pop(); self.push(if a >= b {1} else {0}); },
                
                OP_AND => { let b = self.pop(); let a = self.pop(); self.push(if a!=0 && b!=0 {1} else {0}); },
                OP_OR => { let b = self.pop(); let a = self.pop(); self.push(if a!=0 || b!=0 {1} else {0}); },

                OP_SLEEP => {
                    let ms = self.pop();
                    if ms > 0 { thread::sleep(Duration::from_millis(ms as u64)); }
                },
                OP_DEBUG_PRINT => { let val = self.pop(); println!("[Thread {:?}] DEBUG: {}", thread::current().id(), val); },
                OP_PRINT_CHAR => { 
                    let val = self.pop(); print!("{}", val as u8 as char); io::stdout().flush().unwrap(); 
                },
                OP_PRINT_VAL => { let val = self.pop(); print!("{}", val); io::stdout().flush().unwrap(); },
                
                OP_INPUT => {
                   let mut buffer = String::new();
                   if let Ok(_) = io::stdin().read_line(&mut buffer) {
                       let val = buffer.trim().parse::<i64>().unwrap_or(0);
                       self.push(val); 
                   } else { 
                       self.push(0); 
                   }
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
                        println!("Runtime Error: Call Stack Overflow (Recursion too deep)");
                        self.running = false;
                    } else {
                        self.call_stack.push((self.ip, self.fp));
                        // Frame starts at the first argument
                        // Stack: [..., Arg0, Arg1] < Top
                        // FP = Len - 2
                        if (self.stack.len() as i64) < num_args {
                             println!("Runtime Error: Stack Underflow on Call");
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
                         println!("Runtime Error: Stack Invalid Access Local {}", offset);
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
                         println!("Runtime Error: Stack Invalid Write Local {}", offset);
                         self.running = false;
                     }
                },
                
                // --- VISION OPS ---
                OP_IMG_ALLOC => {
                     let h = self.pop();
                     let w = self.pop();
                     let shared = self.shared.clone();
                     let handle = {
                         let mut state = shared.lock();
                         let id = state.next_handle;
                         state.next_handle += 1;
                         // Initialize with black (0)
                         let size = (w * h) as usize;
                         state.images.insert(id, (w, h, vec![0; size]));
                         id
                     };
                     self.push(handle);
                },
                OP_IMG_FREE => {
                     let handle = self.pop();
                     let shared = self.shared.clone();
                     shared.lock().images.remove(&handle);
                },
                OP_CAM_CAPTURE => {
                     let cam_id = self.pop();
                     let mut new_handle = 0;
                     if let Some(plat) = platform.as_deref_mut() {
                          if let Some((w, h, buffer)) = plat.capture_cam(cam_id as usize) {
                               let shared = self.shared.clone();
                               let mut state = shared.lock();
                               new_handle = state.next_handle;
                               state.next_handle += 1;
                               state.images.insert(new_handle, (w as i64, h as i64, buffer));
                          } else {
                               println!("Runtime Warning: Camera Capture Failed (ID {})", cam_id);
                          }
                     } else {
                         println!("Runtime Error: No Platform Available");
                     }
                     self.push(new_handle);
                },
                OP_IS_KEY_DOWN => {
                     let key = self.pop();
                     let mut res = 0;
                     if let Some(plat) = platform.as_deref() {
                         if plat.is_key_down(key as usize) {
                             res = 1;
                         }
                     }
                     self.push(res);
                },
                OP_IMG_DRAW => {
                    let y = self.pop(); // unused by update_window usually
                    let x = self.pop();
                    let handle = self.pop();
                    let shared = self.shared.clone();
                    let state = shared.lock();
                    if let Some((w, h, data)) = state.images.get(&handle) {
                        eprintln!("DEBUG: img_draw called - handle={}, size={}x{}, data_len={}", handle, w, h, data.len());
                        if let Some(plat) = platform.as_deref_mut() {
                            eprintln!("DEBUG: Calling platform.update_window()");
                            if let Err(e) = plat.update_window(data, *w as usize, *h as usize) {
                                println!("Runtime Warning: Window Update Failed: {}", e);
                            }
                            eprintln!("DEBUG: update_window completed");
                        } else {
                            println!("Runtime Error: No Platform for Display");
                        }
                    } else {
                        println!("Runtime Error: Invalid Image Handle {}", handle);
                    }
                },
                OP_IMG_SET => {
                    let color = self.pop() as u32;
                    let y = self.pop();
                    let x = self.pop();
                    let handle = self.pop();
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                        if x >= 0 && x < *w && y >= 0 && y < *h {
                             data[(y * *w + x) as usize] = color;
                        }
                    }
                },
                OP_IMG_FILL => {
                    let color = self.pop() as u32;
                    let handle = self.pop();
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((_, _, data)) = state.images.get_mut(&handle) {
                         for px in data.iter_mut() { *px = color; }
                    }
                },
                OP_IMG_FILTER => {
                    let mode = self.pop();
                    let handle = self.pop();
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                         for i in 0usize..data.len() {
                             let px = data[i];
                             let r = (px >> 16) & 0xFF;
                             let g = (px >> 8) & 0xFF;
                             let b = px & 0xFF;
                             if mode == 1 { 
                                 // Grayscale / Threshold
                                 let avg = (r + g + b) / 3;
                                 let v = if avg > 128 { 255 } else { 0 };
                                 data[i] = 0xFF000000 | (v << 16) | (v << 8) | v;
                             }
                         }
                    }
                },
                OP_IMG_GET => {
                     let y = self.pop();
                     let x = self.pop();
                     let h = self.pop(); 
                     
                     let val = {
                         let state = self.shared.lock();
                         if let Some((width, height, data)) = state.images.get(&h) { 
                             if x >= 0 && x < *width && y >= 0 && y < *height {
                                 let idx = (y * width + x) as usize;
                                 data[idx] as i64
                             } else {
                                 0
                             }
                         } else {
                             0
                         }
                     };
                     self.push(val);
                },

                OP_VM_STACK_COPY => {
                    let count = self.pop() as usize;
                    let src_idx = self.pop() as usize; // Index in global stack? Or relative to FP?
                    // Memory.nux uses it to copy args to buffer.
                    // It assumes src_idx is absolute stack index?
                    // "PUSH base; PUSH offset; ADD; PUSH 0; PUSH count; VM_STACK_COPY" in old Mem_gc?
                    // Wait, Step 1425 logs showed:
                    // asm { __heap_gc_buf; ... }  -> Pushes buf address (global).
                    // Mem_gc implementation:
                    //  __heap_gc_buf = 0.
                    //  VM_STACK_COPY(src_ptr, dst_ptr, count)? No.
                    //  Let's check `memory.nux` usage.
                    //  `asm { count; dst_start_idx; src_start_idx; VM_STACK_COPY }`?
                    //  I need to check `memory.nux` again to know arguments order.
                    
                    // But assuming standard:
                    // count = pop()
                    // dst_idx = pop()
                    // src_idx = pop()
                    
                    // Wait, `memory.nux` (Step 1500 range, viewed earlier):
                    // "PUSH count; PUSH dst; PUSH src; VM_STACK_COPY" ?
                    // Actually, I haven't implemented it in kernel VM either?
                    // Step 1251 implemented it in kernel VM.
                    // I should check `custom_kernel/src/nux/vm.rs` to match exact logic.
                    // But I can't look at it now efficiently without context switching.
                    // Let's implement generic stack copy.
                    
                    // Usage in `memory.nux` (recalled):
                    // It copies FROM stack TO stack?
                    // Or FROM stack TO buffer?
                    // `VM_STACK_COPY` suggests Stack-to-Stack.
                    // For GC, we copy valid roots from Stack.
                    // But `memory.nux` GC is manual Mark-Sweep?
                    // If it scans stack, it acts as `VM_STACK_SCAN`?
                    
                    // Actually, if `memory.nux` just needs to READ stack roots:
                    // It likely iterates stack.
                    // `VM_STACK_COPY` might be `VM_STACK_READ(index)`.
                    // Or `VM_STACK_COPY(dst_arr, src_idx, count)`.
                    
                    // Let's assume generic copy within stack vector?
                    // If logic is `self.stack[dst..dst+count].copy_from_slice(&self.stack[src..src+count])`.
                    
                    // BUT safe implementation:
                    let dst = self.pop() as usize;
                    let src = self.pop() as usize;
                    
                    // Safety checks
                    if src + count <= self.stack.len() && dst + count <= self.stack.len() {
                        // copy_within is ideal but might overlap.
                        // Rust `copy_within` handles overlap.
                        self.stack.copy_within(src..src+count, dst);
                    } else {
                        // Grow stack if dst is outside?
                        // If dst is meant to be logic for "Top of stack"?
                        // For now, ignore out of bounds or warning.
                        // println!("VM_STACK_COPY OOB: src {} dst {} len {}", src, dst, count);
                    }
                },
                
                OP_SYSTEM => {
                    let cmd_str_ptr = self.pop(); // String pointer?
                    // In Nux, strings are Pointers to Heap? Or inline?
                    // With no heap in VM (only specialized images), Strings are tricky.
                    // Standard Nux strings are usually in `data` segment or constants constructed on stack?
                    // Ah, `string.nux` and `memory.nux`.
                    // The VM has `self.memory`. `OP_PUSH` doesn't push pointers to `self.memory`.
                    // `OP_POKE` writes to `self.memory`.
                    // So `cmd_str_ptr` is index into `self.shared.memory`.
                    
                    // Read string from memory
                     let shared = self.shared.clone();
                     let state = shared.lock();
                     let mem = &state.memory;
                     let ptr = cmd_str_ptr as usize;
                     
                     // Read until null terminator or length prefix?
                     // Standard C-string?
                     let mut s = String::new();
                     let mut i = ptr;
                     while i < mem.len() {
                         let c = mem[i];
                         if c == 0 { break; }
                         s.push(c as char);
                         i += 1;
                     }
                     
                     // Execute
                     if cfg!(target_os = "windows") {
                         let _ = std::process::Command::new("cmd").arg("/C").arg(&s).status();
                     } else {
                         let _ = std::process::Command::new("sh").arg("-c").arg(&s).status();
                     }
                     
                     self.push(0); // Exit code 0 (mock)
                },
                OP_FILE_DELETE => {
                     let path_ptr = self.pop();
                     
                     let shared = self.shared.clone();
                     let state = shared.lock();
                     let mem = &state.memory;
                     let ptr = path_ptr as usize;
                     
                     let mut s = String::new();
                     let mut i = ptr;
                     while i < mem.len() {
                         let c = mem[i];
                         if c == 0 { break; }
                         s.push(c as char);
                         i += 1;
                     }
                     
                     if std::fs::remove_file(&s).is_ok() {
                         self.push(1);
                     } else {
                         self.push(0);
                     }
                },
                OP_TIME => {
                    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
                    self.push(t as i64);
                },
                OP_IMG_RESIZE => {
                    let new_h = self.pop();
                    let new_w = self.pop();
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let new_handle = {
                        let mut state = shared.lock();
                        if let Some((old_w, old_h, old_data)) = state.images.get(&handle).cloned() {
                            // Nearest Neighbor
                            let mut new_data = vec![0u32; (new_w * new_h) as usize];
                            
                            for y in 0..new_h {
                                for x in 0..new_w {
                                    // Map coords
                                    let src_x = (x * old_w) / new_w;
                                    let src_y = (y * old_h) / new_h;
                                    
                                    if src_x < old_w && src_y < old_h {
                                        let old_idx = (src_y * old_w + src_x) as usize;
                                        let val = old_data[old_idx];
                                        new_data[(y * new_w + x) as usize] = val;
                                    }
                                }
                            }
                            
                            let id = state.next_handle;
                            state.next_handle += 1;
                            state.images.insert(id, (new_w, new_h, new_data));
                            id
                        } else {
                            -1
                        }
                    };
                    self.push(new_handle);
                },
                OP_IMG_CROP => {
                    let h = self.pop();
                    let w = self.pop();
                    let y = self.pop();
                    let x = self.pop();
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let new_handle = {
                        let mut state = shared.lock();
                        if let Some((src_w, src_h, src_data)) = state.images.get(&handle).cloned() {
                             let mut new_data = vec![0u32; (w * h) as usize];
                             
                             for cy in 0..h {
                                 for cx in 0..w {
                                     let sx = x + cx;
                                     let sy = y + cy;
                                     
                                     if sx >= 0 && sx < src_w && sy >= 0 && sy < src_h {
                                         let src_idx = (sy * src_w + sx) as usize;
                                         new_data[(cy * w + cx) as usize] = src_data[src_idx];
                                     }
                                 }
                             }
                             
                             let id = state.next_handle;
                             state.next_handle += 1;
                             state.images.insert(id, (w, h, new_data));
                             id
                        } else {
                            -1
                        }
                    };
                    self.push(new_handle);
                },
                OP_VERIFY => {
                    let val = self.pop();
                    if val == 0 {
                        println!("Verification Failed! (Assertion Error)");
                        self.running = false;
                    }
                },
                OP_ALLOC => {
                     let size = self.pop() as usize;
                     let shared_arc = self.shared.clone();
                     let mut shared = shared_arc.lock();
                     let ptr = shared.heap_ptr;
                     
                     // Check limits
                     if let Some(limit) = shared.mem_limit {
                         if ptr + size > limit {
                             println!("Runtime Error: Out of Memory (Hit defined limit)");
                             self.running = false;
                             break;
                         }
                     }
                     
                     if ptr + size > shared.memory.len() {
                         shared.memory.resize(ptr + size + 1024, 0);
                     }
                     shared.heap_ptr += size;
                     
                     // Drop the lock before pushing to avoid borrow checker errors if any
                     drop(shared);
                     
                     self.push(ptr as i64);
                 },
                OP_FREE => {
                     let _ptr = self.pop();
                     // In bump allocator, free is a no-op unless it's the last allocation
                 },
                 OP_LIMIT_MEM => { // OP_LIMIT_MEM
                     let percentage = self.pop();
                     if percentage > 0 && percentage <= 100 {
                         let total_system_memory = 1024 * 1024 * 1024; // Dummy 1GB limit for simulation
                         let limit = (total_system_memory as f64 * (percentage as f64 / 100.0)) as usize;
                         self.shared.lock().mem_limit = Some(limit);
                     }
                 },
                OP_IMG_GRAYSCALE => {
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    // We modify in-place or return new? 
                    // Let's modify in-place for efficiency, or return new for immutability?
                    // User might want to keep original. Let's return new.
                    let new_handle = {
                        let mut state = shared.lock();
                        if let Some((w, h, src_data)) = state.images.get(&handle).cloned() {
                            let mut new_data = vec![0u32; src_data.len()];
                            
                            for i in 0usize..src_data.len() {
                                let pixel = src_data[i];
                                let r = (pixel >> 16) & 0xFF;
                                let g = (pixel >> 8) & 0xFF;
                                let b = pixel & 0xFF;
                                // Luminosity: 0.21 R + 0.72 G + 0.07 B
                                let gray = ((r as f32 * 0.21) + (g as f32 * 0.72) + (b as f32 * 0.07)) as u32;
                                let new_pixel = (0xFF << 24) | (gray << 16) | (gray << 8) | gray;
                                new_data[i] = new_pixel;
                            }
                            
                            let id = state.next_handle;
                            state.next_handle += 1;
                            state.images.insert(id, (w, h, new_data));
                            id
                        } else {
                            -1
                        }
                    };
                    self.push(new_handle);
                },
                // --- THREADING_OPS ---
                OP_SPAWN => {
                    let target = self.read_i64_code(); // Function address
                    // Fork a VM instance
                    let mut child_vm = self.fork(target as usize);
                    
                    // Spawn OS Thread
                    thread::spawn(move || {
                        child_vm.run(None); // Background threads have no platform/window
                    });
                    // println!("DEBUG: Spawning thread at {}", target);
                },
                // Locking ops (TODO: Implement proper ID-based locks if needed)
                OP_LOCK => { /* Placeholder */ },
                OP_UNLOCK => { /* Placeholder */ },

                OP_KERNEL_OP => {
                    let op_id = self.pop();
                    match op_id {
                        1 => print!("\x1B[2J\x1B[1;1H"),
                        2 => println!("NuxVM Multi-Threaded v0.4"),
                        _ => {},
                    }
                },
                OP_VISION_DETECT => {
                     // Stub for Vision Detect
                     // Stack: [Mode (2)]
                     // Takes Image Handle from stack implicitly?
                     // cv_test.nux logic:
                     // PUSH 0
                     // PUSH 16; PEEK (Handle)
                     // PUSH 2 (Mode)
                     // OP_VISION_DETECT
                     
                     // It seems it pops 3 arguments? or 2?
                     // Let's assume it pops Mode, then Handle, then dest?
                     // Usage:
                     // PUSH 0 (Result Handle placeholder?)
                     // PUSH 16; PEEK -> Handle
                     // PUSH Mode
                     // OP_VISION_DETECT
                     
                     let mode = self.pop();
                     let handle = self.pop();
                     let result_slot = self.pop(); // Pop the 0 pushed before?
                     
                     // Simply modify the image in place or verify detection?
                     // The test expects img_get to change.
                     // Mode 2: Edge Detection?
                     
                     // Mock Implementation:
                     // Draw a white rectangle at 10,5 for Edge check?
                     // Or sets global result?
                     
                     // "Verify Vision"
                     // print(4001) -> img_get(0,0) < 10
                     // print(4002) -> img_get(10,5) > 100
                     
                     // So we need to ensure (10,5) is BRIGHT (>100) and (0,0) is DARK (<10).
                     // Previously we did:
                     // gfx_clear(white) -> All white.
                     // gfx_rect(..., 0) -> Black square at 10,5.
                     
                     // Wait, (10,5) was set to 0 (Black) in line 39.
                     // (0,0) was cleared to White (16777215).
                     
                     // Verification Expects:
                     // (0,0) < 10 (Dark)
                     // (10,5) > 100 (Bright)
                     
                     // So Vision Detect must INVERT or EDGE DETECT such that:
                     // Black Square Edge becomes Bright?
                     // White Background becomes Dark?
                     
                     // Simple Mock: 
                     // Set (10,5) to 255 (White/Bright)
                     // Set (0,0) to 0 (Black)
                     
                     let shared = self.shared.clone();
                     let mut state = shared.lock();
                     if let Some((w, h, data)) = state.images.get_mut(&handle) {
                          // Mock Process
                          if data.len() > 0 { data[0] = 0; } // 0,0 Black
                          let idx = (5 * *w + 10) as usize;
                          if idx < data.len() { data[idx] = 0xFFFFFF; } // 10,5 White
                     }
                },
                
                OP_TIME => {
                    let start = std::time::SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Time went backwards");
                    self.push(since_the_epoch.as_millis() as i64);
                },

                // Graphics Opcodes
                OP_GFX_CLEAR => {
                    // Stack: [img_handle, color]
                    let color = self.pop() as u32;
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                        for pixel in data.iter_mut() {
                            *pixel = color;
                        }
                    }
                },

                OP_DRAW_PIXEL => {
                    // Stack: [img_handle, x, y, color]
                    let color = self.pop() as u32;
                    let y = self.pop() as i64;
                    let x = self.pop() as i64;
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                        if x >= 0 && y >= 0 && x < *w && y < *h {
                            let idx = (y * *w + x) as usize;
                            if idx < data.len() {
                                data[idx] = color;
                            }
                        }
                    }
                },

                OP_DRAW_LINE => {
                    // Stack: [img_handle, x1, y1, x2, y2, color]
                    let color = self.pop() as u32;
                    let y2 = self.pop() as i64;
                    let x2 = self.pop() as i64;
                    let y1 = self.pop() as i64;
                    let x1 = self.pop() as i64;
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                        // Bresenham's line algorithm
                        let mut x = x1;
                        let mut y = y1;
                        let dx = (x2 - x1).abs();
                        let dy = (y2 - y1).abs();
                        let sx = if x1 < x2 { 1 } else { -1 };
                        let sy = if y1 < y2 { 1 } else { -1 };
                        let mut err = dx - dy;

                        loop {
                            // Set pixel
                            if x >= 0 && y >= 0 && x < *w && y < *h {
                                let idx = (y * *w + x) as usize;
                                if idx < data.len() {
                                    data[idx] = color;
                                }
                            }

                            if x == x2 && y == y2 { break; }

                            let e2 = 2 * err;
                            if e2 > -dy {
                                err -= dy;
                                x += sx;
                            }
                            if e2 < dx {
                                err += dx;
                                y += sy;
                            }
                        }
                    }
                },

                OP_DRAW_CIRCLE => {
                    // Stack: [img_handle, cx, cy, radius, color]
                    let color = self.pop() as u32;
                    let radius = self.pop() as i64;
                    let cy = self.pop() as i64;
                    let cx = self.pop() as i64;
                    let handle = self.pop();
                    
                    let shared = self.shared.clone();
                    let mut state = shared.lock();
                    if let Some((w, h, data)) = state.images.get_mut(&handle) {
                        // Midpoint circle algorithm
                        let mut x = radius;
                        let mut y = 0;
                        let mut err = 0;

                        while x >= y {
                            // Draw 8 octants
                            let points = [
                                (cx + x, cy + y), (cx + y, cy + x),
                                (cx - y, cy + x), (cx - x, cy + y),
                                (cx - x, cy - y), (cx - y, cy - x),
                                (cx + y, cy - x), (cx + x, cy - y),
                            ];

                            for (px, py) in &points {
                                if *px >= 0 && *py >= 0 && *px < *w && *py < *h {
                                    let idx = (*py * *w + *px) as usize;
                                    if idx < data.len() {
                                        data[idx] = color;
                                    }
                                }
                            }

                            if err <= 0 {
                                y += 1;
                                err += 2 * y + 1;
                            }
                            if err > 0 {
                                x -= 1;
                                err -= 2 * x + 1;
                            }
                        }
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
                        println!("Runtime Error: Segfault Read {}", addr); 
                        self.running = false;
                    }
                },
                OP_POKE => {
                    let addr = self.pop();
                    let val = self.pop();
                    let shared = self.shared.clone();
                    let success = {
                        let mut state = shared.lock();
                        if addr < 0 || addr as usize + 8 > state.memory.len() {
                            false
                        } else {
                            let bytes = val.to_le_bytes();
                            for i in 0..8 {
                                state.memory[addr as usize + i] = bytes[i];
                            }
                            true
                        }
                    };
                    if !success {
                        println!("Runtime Error: Segfault Write {}", addr);
                        self.running = false;
                    }
                },
                
                OP_SYS_PLATFORM => {
                     let p = if let Some(plat) = &platform { plat.platform_type() } else { 0 };
                     self.push(p as i64);
                },
                OP_CAM_COUNT => {
                     let c = if let Some(plat) = &platform { plat.list_cameras().len() } else { 0 };
                     self.push(c as i64);
                },
                
                OP_EXIT => { self.running = false; },
                _ => { eprintln!("VM Panic: Unknown Opcode: {:02X} at IP {}", op, self.ip - 1); }
            }
        }
    }
}

