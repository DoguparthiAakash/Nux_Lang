// Standalone Nux VM
// Implements executing Nux Bytecode

use std::vec::Vec;
use std::string::String;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, BTreeMap};
use rand::Rng;
use num_complex::Complex64;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;

pub enum NuxStream {
    Tcp(TcpStream),
    Tls(rustls::StreamOwned<rustls::ServerConnection, TcpStream>),
}

impl NuxStream {
    pub fn shutdown(&self, how: std::net::Shutdown) -> io::Result<()> {
        match self {
            NuxStream::Tcp(s) => s.shutdown(how),
            NuxStream::Tls(s) => s.get_ref().shutdown(how),
        }
    }
}

impl Read for NuxStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            NuxStream::Tcp(s) => s.read(buf),
            NuxStream::Tls(s) => s.read(buf),
        }
    }
}

impl Write for NuxStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            NuxStream::Tcp(s) => s.write(buf),
            NuxStream::Tls(s) => s.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            NuxStream::Tcp(s) => s.flush(),
            NuxStream::Tls(s) => s.flush(),
        }
    }
}

pub enum NuxListener {
    Tcp(TcpListener),
    Tls(TcpListener, Arc<ServerConfig>),
}

impl NuxListener {
    pub fn accept(&self) -> io::Result<NuxStream> {
        match self {
            NuxListener::Tcp(l) => {
                let (stream, _) = l.accept()?;
                Ok(NuxStream::Tcp(stream))
            },
            NuxListener::Tls(l, config) => {
                let (stream, _) = l.accept()?;
                let conn = rustls::ServerConnection::new(Arc::clone(config)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                let tls_stream = rustls::StreamOwned::new(conn, stream);
                Ok(NuxStream::Tls(tls_stream))
            }
        }
    }
}
use std::convert::TryInto;
// use std::thread;

#[cfg(feature = "minifb")]
use minifb::{Window, WindowOptions};

#[cfg(feature = "minifb")]
pub struct SendWindow(pub Window);
#[cfg(feature = "minifb")]
unsafe impl Send for SendWindow {}
// const OP_VBE_SET_MODE: u8 = 0x3A;
// const OP_VBE_GET_FB: u8 = 0x3B;
// const OP_VBE_UPDATE: u8 = 0x3C;

pub struct SharedVmState {
    pub memory: RwLock<Vec<u8>>,
    pub heap_ptr: std::sync::atomic::AtomicUsize,
    pub heap_strings: RwLock<Vec<String>>,
    pub heap_arrays: RwLock<Vec<Vec<i64>>>,
    pub quantum_state: RwLock<Vec<Complex64>>,
    pub threads: Mutex<std::collections::HashMap<i64, std::thread::JoinHandle<i64>>>,
    pub next_thread_id: std::sync::atomic::AtomicI64,
    pub listeners: RwLock<Vec<NuxListener>>,
    pub connections: RwLock<Vec<NuxStream>>,
}

pub struct Stack {
    pub data: [i64; 1024],
    pub len: usize,
}
impl Stack {
    #[inline(always)]
    pub fn new() -> Self { Self { data: [0; 1024], len: 0 } }
    #[inline(always)]
    pub fn push(&mut self, val: i64) { unsafe { *self.data.get_unchecked_mut(self.len) = val; } self.len += 1; }
    #[inline(always)]
    pub fn pop(&mut self) -> Option<i64> { if self.len == 0 { None } else { self.len -= 1; Some(unsafe { *self.data.get_unchecked(self.len) }) } }
    #[inline(always)]
    pub fn is_empty(&self) -> bool { self.len == 0 }
    #[inline(always)]
    pub fn len(&self) -> usize { self.len }
    #[inline(always)]
    pub fn truncate(&mut self, new_len: usize) { self.len = new_len; }
    #[inline(always)]
    pub fn last(&self) -> Option<&i64> { if self.len == 0 { None } else { Some(&self.data[self.len - 1]) } }
    #[inline(always)]
    pub fn resize(&mut self, new_len: usize, val: i64) {
        while self.len < new_len { self.data[self.len] = val; self.len += 1; }
    }
}
impl std::ops::Index<usize> for Stack {
    type Output = i64;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}
impl std::ops::IndexMut<usize> for Stack {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] }
}

pub struct NuxVm {
    stack: Stack,
    code: Arc<Vec<u8>>,
    ip: usize,
    call_stack: Vec<(usize, usize)>, // (ret_ip, base_pointer)
    catch_stack: Vec<(usize, usize, usize)>, // (catch_ip, call_stack_len, stack_len)
    base_pointer: usize,
    
    shared: Arc<SharedVmState>,
    
    // Graphics
    #[cfg(feature = "minifb")]
    window: Option<SendWindow>,
    fb_width: usize,
    fb_height: usize,
    fb_addr: usize,
    is_unsafe: bool,
}

impl NuxVm {
    pub fn new(mut code: Vec<u8>) -> Self {
        // Security Verification
        let code_len = code.len();
        if code_len >= 12 && &code[code_len - 12..code_len - 8] == b"NUX!" {
            let mut expected_hash_bytes = [0u8; 8];
            expected_hash_bytes.copy_from_slice(&code[code_len - 8..]);
            let expected_hash = u64::from_le_bytes(expected_hash_bytes);
            
            // Remove the checksum bytes for execution
            code.truncate(code_len - 12);
            
            // Recompute FNV-1a checksum
            let mut hash: u64 = 0xcbf29ce484222325;
            for &b in &code {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
            
            if hash != expected_hash {
                eprintln!("SECURITY ERROR: Bytecode checksum verification failed. The file may be corrupted or tampered with.");
                std::process::exit(1);
            }
            
            // Decrypt the bytecode
            for b in code.iter_mut() {
                *b ^= 0x5A;
            }
        } else {
            // Optional: Refuse to run unsigned code? 
            // We'll allow it with a warning for now so standard dev works.
            // eprintln!("WARNING: Running unsigned Nux bytecode.");
        }

        Self {
            stack: Stack::new(),
            code: Arc::new(code),
            ip: 0,
            call_stack: Vec::new(),
            catch_stack: Vec::new(),
            base_pointer: 0,
            shared: Arc::new(SharedVmState {
                memory: RwLock::new(vec![0u8; 64 * 1024]),
                heap_ptr: std::sync::atomic::AtomicUsize::new(8),
                heap_strings: RwLock::new(Vec::new()),
                heap_arrays: RwLock::new(Vec::new()),
                quantum_state: RwLock::new(Vec::new()),
                threads: Mutex::new(std::collections::HashMap::new()),
                next_thread_id: std::sync::atomic::AtomicI64::new(1),
                listeners: RwLock::new(Vec::new()),
                connections: RwLock::new(Vec::new()),
            }),
            #[cfg(feature = "minifb")]
            window: None,
            fb_width: 640,
            fb_height: 480,
            fb_addr: 0,
            is_unsafe: false,
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
        // Skip Header (64 bytes) if starting fresh
        if self.ip == 0 {
            if self.code.len() > 64 {
                self.ip = 64;
            }
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
                     self.stack.push(a.wrapping_add(b));
                },
                0x11 => { // SUB
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a.wrapping_sub(b));
                },
                0x12 => { // MUL
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a.wrapping_mul(b));
                },
                0x13 => { // DIV
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     if b == 0 { self.stack.push(0); } else { self.stack.push(a.wrapping_div(b)); }
                },
                0x14 => { // MOD
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     if b == 0 { self.stack.push(0); } else { self.stack.push(a.wrapping_rem(b)); }
                },
                0x15 => { // POW (integer power)
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     let mut result: i64 = 1;
                     for _ in 0..b.max(0) { result = result.wrapping_mul(a); }
                     self.stack.push(result);
                },
                0x16 => { // FLOORDIV
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     if b == 0 { self.stack.push(0); } else { self.stack.push(a.div_euclid(b)); }
                },
                0x17 => { // SWAP
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(b);
                     self.stack.push(a);
                },
                0x18 => { // AND (bitwise)
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a & b);
                },
                0x19 => { // OR (bitwise)
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a | b);
                },
                0x1A => { // NOT (logical: 0->1, nonzero->0)
                     let a = self.stack.pop().unwrap();
                     self.stack.push(if a == 0 { 1 } else { 0 });
                },
                0x25 => { // SHL
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a << b);
                },
                0x26 => { // SHR
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a >> b);
                },
                0x1B => { // FADD
                     let b = f64::from_bits(self.stack.pop().unwrap() as u64);
                     let a = f64::from_bits(self.stack.pop().unwrap() as u64);
                     self.stack.push((a + b).to_bits() as i64);
                },
                0x1C => { // FMUL
                     let b = f64::from_bits(self.stack.pop().unwrap() as u64);
                     let a = f64::from_bits(self.stack.pop().unwrap() as u64);
                     self.stack.push((a * b).to_bits() as i64);
                },
                0x1D => { // FDIV
                     let b = f64::from_bits(self.stack.pop().unwrap() as u64);
                     let a = f64::from_bits(self.stack.pop().unwrap() as u64);
                     self.stack.push((a / b).to_bits() as i64);
                },

                0x22 => { // XOR (bitwise)
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a ^ b);
                },
                0x23 => { // XAND (a & !b)
                     let b = self.stack.pop().unwrap();
                     let a = self.stack.pop().unwrap();
                     self.stack.push(a & (!b));
                },
                0x30 => { // SLEEP (ms)
                     let ms = self.stack.pop().unwrap_or(0);
                     if ms > 0 {
                         std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                     }
                },
                0x50 => { // DEBUG_PRINT (print stack top without popping)
                     if let Some(&val) = self.stack.last() {
                         eprintln!("[DEBUG] stack top = {}", val);
                     } else {
                         eprintln!("[DEBUG] stack empty");
                     }
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
                0x5D => { // DUP
                    let val = *self.stack.last().unwrap();
                    self.stack.push(val);
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
                    print!("{}", char::from_u32(val as u32).unwrap_or('?'));
                    io::stdout().flush().unwrap();
                },
                0x52 => { // INPUT (read line from stdin, push string id)
                    io::stdout().flush().unwrap();
                    let mut line = String::new();
                    let _ = io::stdin().read_line(&mut line);
                    let line = line.trim_end_matches('\n').trim_end_matches('\r').to_string();
                    let mut heap_strings = self.shared.heap_strings.write().unwrap();
                    heap_strings.push(line);
                    self.stack.push((heap_strings.len() - 1) as i64);
                },
                0x53 => { // PRINT_VAL (print integer)
                    let val = self.stack.pop().unwrap_or(0);
                    print!("{}", val);
                    io::stdout().flush().unwrap();
                },
                0x54 => { // PRINT_FLOAT
                    let val = self.stack.pop().unwrap_or(0);
                    let f = f64::from_bits(val as u64);
                    print!("{:.4}", f);
                    io::stdout().flush().unwrap();
                },
                0x55 => { // FILE_OPEN (stub - push -1)
                    self.stack.pop(); // mode
                    self.stack.pop(); // path id
                    self.stack.push(-1);
                },
                0x56 => { // FILE_CLOSE (stub)
                    self.stack.pop();
                },
                0x57 => { // FILE_READ (read entire file by path id)
                    let path_id = self.stack.pop().unwrap_or(0);
                    let mut heap_strings = self.shared.heap_strings.write().unwrap();
                    let path = if path_id >= 0 && (path_id as usize) < heap_strings.len() {
                        heap_strings[path_id as usize].clone()
                    } else {
                        String::new()
                    };
                    let content = std::fs::read_to_string(&path).unwrap_or_default();
                    heap_strings.push(content);
                    self.stack.push((heap_strings.len() - 1) as i64);
                },
                0x58 => { // FILE_WRITE (write data id to path id)
                    let data_id = self.stack.pop().unwrap_or(0);
                    let path_id = self.stack.pop().unwrap_or(0);
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    
                    let path = if path_id >= 0 && (path_id as usize) < heap_strings.len() {
                        heap_strings[path_id as usize].clone()
                    } else {
                        String::new()
                    };
                    
                    let data = if data_id >= 0 && (data_id as usize) < heap_strings.len() {
                        heap_strings[data_id as usize].clone()
                    } else {
                        String::new()
                    };
                    
                    drop(heap_strings);
                    
                    if !path.is_empty() {
                         let _ = std::fs::write(&path, &data);
                    }
                },
                0x59 => { // FILE_EXISTS (stub - always false)
                    self.stack.pop();
                    self.stack.push(0);
                },
                0x81 => { // SYSTEM (stub)
                    self.stack.pop();
                    self.stack.push(0);
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
                    let mut heap_strings = self.shared.heap_strings.write().unwrap();
                    heap_strings.push(s);
                    // Encode as a "pointer" (index).
                    self.stack.push((heap_strings.len() - 1) as i64);
                },
                0x69 => { // TO_UPPER
                    let id = self.stack.pop().unwrap() as usize;
                    let mut heap_strings = self.shared.heap_strings.write().unwrap();
                    if id < heap_strings.len() {
                         let s = heap_strings[id].to_uppercase();
                         heap_strings.push(s);
                         self.stack.push((heap_strings.len() - 1) as i64);
                    } else { self.stack.push(0); }
                },
                0x6A => { // TO_LOWER
                    let id = self.stack.pop().unwrap() as usize;
                    let mut heap_strings = self.shared.heap_strings.write().unwrap();
                    if id < heap_strings.len() {
                         let s = heap_strings[id].to_lowercase();
                         heap_strings.push(s);
                         self.stack.push((heap_strings.len() - 1) as i64);
                    } else { self.stack.push(0); }
                },
                0x6B => { // PRINT_STR
                    let id = self.stack.pop().unwrap() as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    if id < heap_strings.len() {
                        print!("{}", heap_strings[id]);
                        io::stdout().flush().unwrap();
                    }
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
                     if idx >= self.stack.len() {
                         self.stack.resize(idx + 1, 0);
                     }
                     self.stack[idx] = val;
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
                0x2D => { // FTAN
                     let a = f64::from_bits(self.stack.pop().unwrap() as u64);
                     self.stack.push((a.tan()).to_bits() as i64);
                },
                0x46 => { // FPOW
                     let b = f64::from_bits(self.stack.pop().unwrap() as u64);
                     let a = f64::from_bits(self.stack.pop().unwrap() as u64);
                     self.stack.push((a.powf(b)).to_bits() as i64);
                },
                0x40 => { // PEEK
                     let addr = self.stack.pop().unwrap() as usize;
                     let mut memory = self.shared.memory.write().unwrap();
                     if addr + 8 > memory.len() {
                         let new_len = (addr + 8).max(memory.len() * 2);
                         memory.resize(new_len, 0);
                     }
                     let bytes: [u8; 8] = memory[addr..addr+8].try_into().unwrap();
                     self.stack.push(i64::from_le_bytes(bytes));
                },
                0x41 => { // POKE
                     let val = self.stack.pop().unwrap();
                     let addr = self.stack.pop().unwrap() as usize;
                     let mut memory = self.shared.memory.write().unwrap();
                     if addr + 8 > memory.len() {
                         let new_len = (addr + 8).max(memory.len() * 2);
                         memory.resize(new_len, 0);
                     }
                     let bytes = val.to_le_bytes();
                     memory[addr..addr+8].copy_from_slice(&bytes);
                },
                0x49 => { // OP_POKE32
                     let val = self.stack.pop().unwrap() as u32;
                     let addr = self.stack.pop().unwrap() as usize;
                     let mut memory = self.shared.memory.write().unwrap();
                     if addr + 4 > memory.len() {
                         let new_len = (addr + 4).max(memory.len() * 2);
                         memory.resize(new_len, 0);
                     }
                     let bytes = val.to_le_bytes();
                     memory[addr..addr+4].copy_from_slice(&bytes);
                 },
                 0x4C => { // OP_PEEK32
                     let addr = self.stack.pop().unwrap() as usize;
                     let mut memory = self.shared.memory.write().unwrap();
                     if addr + 4 > memory.len() {
                         let new_len = (addr + 4).max(memory.len() * 2);
                         memory.resize(new_len, 0);
                     }
                     let bytes: [u8; 4] = memory[addr..addr+4].try_into().unwrap();
                     self.stack.push(u32::from_le_bytes(bytes) as i64);
                 },
                0x42 => { // PEEK_PTR
                    if !self.is_unsafe { eprintln!("SECURITY ERROR: Raw pointer access outside of unsafe block."); std::process::exit(1); }
                    let addr = self.stack.pop().unwrap() as usize;
                    let val = unsafe { *(addr as *const i64) };
                    self.stack.push(val);
                },
                0x43 => { // POKE_PTR
                    if !self.is_unsafe { eprintln!("SECURITY ERROR: Raw pointer access outside of unsafe block."); std::process::exit(1); }
                    let val = self.stack.pop().unwrap();
                    let addr = self.stack.pop().unwrap() as usize;
                    unsafe { *(addr as *mut i64) = val; }
                },
                0x47 => { // SYSCALL
                    if !self.is_unsafe { eprintln!("SECURITY ERROR: syscall outside of unsafe block."); std::process::exit(1); }
                    let num = self.stack.pop().unwrap();
                    // System call logic placeholder
                    self.stack.push(0); // return 0 for now
                },
                0x4A => { // UNSAFE_START
                    self.is_unsafe = true;
                },
                0x4B => { // UNSAFE_END
                    self.is_unsafe = false;
                },
                
                
                0xEA => { // OP_Q_ALLOC
                    let size = self.stack.pop().unwrap() as usize;
                    let num_amplitudes = 1 << size;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    q_state.clear();
                    q_state.resize(num_amplitudes, Complex64::new(0.0, 0.0));
                    q_state[0] = Complex64::new(1.0, 0.0);
                },
                0xEB => { // OP_Q_H
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    let inv_sqrt2 = 1.0 / std::f64::consts::SQRT_2;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            let a = q_state[i];
                            let b = q_state[i | bit];
                            q_state[i] = (a + b) * inv_sqrt2;
                            q_state[i | bit] = (a - b) * inv_sqrt2;
                        }
                    }
                },
                0xEC => { // OP_Q_X
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            let temp = q_state[i];
                            q_state[i] = q_state[i | bit];
                            q_state[i | bit] = temp;
                        }
                    }
                },
                0xED => { // OP_Q_Z
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    for i in 0..n {
                        if (i & bit) != 0 {
                            q_state[i] = q_state[i] * -1.0;
                        }
                    }
                },
                0xEE => { // OP_Q_CX
                    let target = self.stack.pop().unwrap() as usize;
                    let control = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let cbit = 1 << control;
                    let tbit = 1 << target;
                    for i in 0..n {
                        if (i & cbit) != 0 && (i & tbit) == 0 {
                            let temp = q_state[i];
                            q_state[i] = q_state[i | tbit];
                            q_state[i | tbit] = temp;
                        }
                    }
                },
                0xEF => { // OP_Q_MEASURE
                    let target = self.stack.pop().unwrap() as usize;
                    let mut q_state = self.shared.quantum_state.write().unwrap();
                    let n = q_state.len();
                    let bit = 1 << target;
                    let mut prob_zero = 0.0;
                    for i in 0..n {
                        if (i & bit) == 0 {
                            prob_zero += q_state[i].norm_sqr();
                        }
                    }
                    
                    let random_val: f64 = rand::random();
                    let outcome = if random_val < prob_zero { 0 } else { 1 };
                    
                    let norm = if outcome == 0 { prob_zero.sqrt() } else { (1.0 - prob_zero).sqrt() };
                    
                    for i in 0..n {
                        if ((i & bit) == 0 && outcome == 1) || ((i & bit) != 0 && outcome == 0) {
                            q_state[i] = Complex64::new(0.0, 0.0);
                        } else {
                            q_state[i] = q_state[i] / norm;
                        }
                    }
                    self.stack.push(outcome as i64);
                },

                0xE7 => { // OP_FFI_PYTHON
                    let code_ptr = self.stack.pop().unwrap() as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    let code_str = if code_ptr < heap_strings.len() { heap_strings[code_ptr].clone() } else { String::new() };
                    drop(heap_strings);
                    
                    let output = std::process::Command::new("python").arg("-c").arg(code_str).output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    
                    let mut heap_strings_mut = self.shared.heap_strings.write().unwrap();
                    heap_strings_mut.push(res);
                    self.stack.push((heap_strings_mut.len() - 1) as i64);
                },
                0xE8 => { // OP_FFI_C
                    let code_ptr = self.stack.pop().unwrap() as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    let code_str = if code_ptr < heap_strings.len() { heap_strings[code_ptr].clone() } else { String::new() };
                    drop(heap_strings);
                    
                    std::fs::write(".tmp_inline.c", code_str).unwrap();
                    let _ = std::process::Command::new("gcc").arg(".tmp_inline.c").arg("-o").arg(".tmp_inline.exe").status();
                    let output = std::process::Command::new("./.tmp_inline.exe").output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    
                    let mut heap_strings_mut = self.shared.heap_strings.write().unwrap();
                    heap_strings_mut.push(res);
                    self.stack.push((heap_strings_mut.len() - 1) as i64);
                },
                0xE0 => { // SPAWN_THREAD
                    let dest = self.read_i64() as usize;
                    let num_args = self.read_i64() as usize;
                    
                    if self.stack.len() < num_args { panic!("Stack underflow on spawn"); }
                    
                    let mut child_vm = NuxVm {
                        stack: Stack::new(),
                        code: self.code.clone(),
                        ip: dest,
                        call_stack: Vec::new(),
                        catch_stack: Vec::new(),
                        base_pointer: 0,
                        shared: Arc::clone(&self.shared),
                        
                        #[cfg(feature = "minifb")]
                        window: None,
                        fb_width: self.fb_width,
                        fb_height: self.fb_height,
                        fb_addr: self.fb_addr,
                        is_unsafe: self.is_unsafe,
                    };
                    
                    let args_start = self.stack.len() - num_args;
                    for i in args_start..self.stack.len() {
                        child_vm.stack.push(self.stack[i]);
                    }
                    self.stack.truncate(args_start);
                    
                    let thread_id = self.shared.next_thread_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    
                    let handle = std::thread::spawn(move || {
                        child_vm.run();
                        child_vm.stack.pop().unwrap_or(0)
                    });
                    
                    self.shared.threads.lock().unwrap().insert(thread_id, handle);
                    self.stack.push(thread_id);
                },
                0xE1 => { // JOIN_THREAD
                    let thread_id = self.stack.pop().unwrap();
                    let handle_opt = self.shared.threads.lock().unwrap().remove(&thread_id);
                    if let Some(handle) = handle_opt {
                        let result = handle.join().unwrap_or(0);
                        self.stack.push(result);
                    } else {
                        self.stack.push(0); // Thread not found
                    }
                },
                
                0xA0 => { // OP_ARRAY_ALLOC
                    let count = self.stack.pop().unwrap() as usize;
                    let mut arr = Vec::with_capacity(count);
                    // Values are pushed onto the stack left to right, but since it's a stack, they are popped right to left.
                    // We pop them in reverse order to build the array correctly if we build from the end, but actually
                    // we want them in left to right order. Wait, if compiler visits `1, 2, 3`, it pushes 1, then 2, then 3.
                    // Pop order: 3, 2, 1. We should push into arr and then reverse, or fill backwards.
                    for _ in 0..count {
                        arr.push(self.stack.pop().unwrap());
                    }
                    arr.reverse();
                    
                    let mut heap_arrays = self.shared.heap_arrays.write().unwrap();
                    heap_arrays.push(arr);
                    self.stack.push((heap_arrays.len() - 1) as i64);
                },
                0xA1 => { // OP_ARRAY_GET
                    let idx = self.stack.pop().unwrap() as usize;
                    let arr_id = self.stack.pop().unwrap() as usize;
                    let heap_arrays = self.shared.heap_arrays.read().unwrap();
                    if let Some(arr) = heap_arrays.get(arr_id) {
                        if idx < arr.len() {
                            self.stack.push(arr[idx]);
                        } else {
                            eprintln!("Index out of bounds: {} (len {})", idx, arr.len());
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("Invalid array id: {}", arr_id);
                        std::process::exit(1);
                    }
                },
                0xA2 => { // OP_ARRAY_SET
                    let val = self.stack.pop().unwrap();
                    let idx = self.stack.pop().unwrap() as usize;
                    let arr_id = self.stack.pop().unwrap() as usize;
                    let mut heap_arrays = self.shared.heap_arrays.write().unwrap();
                    if let Some(arr) = heap_arrays.get_mut(arr_id) {
                        if idx < arr.len() {
                            arr[idx] = val;
                        } else {
                            eprintln!("Index out of bounds: {} (len {})", idx, arr.len());
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("Invalid array id: {}", arr_id);
                        std::process::exit(1);
                    }
                },
                0xA3 => { // OP_THROW
                    let err_val = self.stack.pop().unwrap_or(0);
                    if let Some((catch_ip, call_depth, stack_len)) = self.catch_stack.pop() {
                        // Unwind call stack
                        while self.call_stack.len() > call_depth {
                            if let Some((_, old_bp)) = self.call_stack.pop() {
                                self.base_pointer = old_bp;
                            }
                        }
                        // Unwind value stack
                        self.stack.truncate(stack_len);
                        self.stack.push(err_val);
                        self.ip = catch_ip;
                    } else {
                        eprintln!("Unhandled exception thrown: {}", err_val);
                        std::process::exit(1);
                    }
                },
                0xA4 => { // OP_CATCH
                    let label = self.read_i64() as usize;
                    self.catch_stack.push((label, self.call_stack.len(), self.stack.len()));
                },
                0xA5 => { // OP_END_TRY
                    self.catch_stack.pop();
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
                         let mut memory = self.shared.memory.write().unwrap();
                         if memory.len() < required {
                             memory.resize(required, 0);
                         }
                         
                         let mut window = Window::new(
                             "Nux Standalone Window",
                             width,
                             height,
                             WindowOptions::default(),
                         ).unwrap_or_else(|e| {
                             panic!("{}", e);
                         });
                         
                         #[allow(deprecated)]
                         window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
                         self.window = Some(SendWindow(window));
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
                    if let Some(send_window) = &mut self.window {
                        let window = &mut send_window.0;
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
                            let memory = self.shared.memory.read().unwrap();
                            for i in 0..len {
                                let addr = self.fb_addr + i * 4;
                                let b = memory[addr];
                                let g = memory[addr+1];
                                let r = memory[addr+2];
                                let _a = memory[addr+3]; 
                                // Minifb expects 00RRGGBB (or ARGB?)
                                // usually 0x00RRGGBB.
                                buffer[i] = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                            }
                            
                            window.update_with_buffer(&buffer, self.fb_width, self.fb_height).unwrap();
                        }
                    }
                },
                0x3D => { // OP_VBE_GET_KEY
                    #[cfg(feature = "minifb")]
                    {
                        let key_code = self.stack.pop().unwrap();
                        let mut pressed = 0;
                        if let Some(send_window) = &self.window {
                            let window = &send_window.0;
                            let key = match key_code {
                                87 => Some(minifb::Key::W),
                                65 => Some(minifb::Key::A),
                                83 => Some(minifb::Key::S),
                                68 => Some(minifb::Key::D),
                                32 => Some(minifb::Key::Space),
                                37 => Some(minifb::Key::Left),
                                38 => Some(minifb::Key::Up),
                                39 => Some(minifb::Key::Right),
                                40 => Some(minifb::Key::Down),
                                _ => None,
                            };
                            if let Some(k) = key {
                                if window.is_key_down(k) {
                                    pressed = 1;
                                }
                            }
                        }
                        self.stack.push(pressed);
                    }
                    #[cfg(not(feature = "minifb"))]
                    {
                        self.stack.pop();
                        self.stack.push(0);
                    }
                },
                0x3E => { // OP_VBE_GET_MOUSE_X
                    #[cfg(feature = "minifb")]
                    {
                        let mut mx = 0;
                        if let Some(send_window) = &self.window {
                            let window = &send_window.0;
                            if let Some((x, _y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                                mx = x as i64;
                            }
                        }
                        self.stack.push(mx);
                    }
                    #[cfg(not(feature = "minifb"))]
                    { self.stack.push(0); }
                },
                0x3F => { // OP_VBE_GET_MOUSE_Y
                    #[cfg(feature = "minifb")]
                    {
                        let mut my = 0;
                        if let Some(send_window) = &self.window {
                            let window = &send_window.0;
                            if let Some((_x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                                my = y as i64;
                            }
                        }
                        self.stack.push(my);
                    }
                    #[cfg(not(feature = "minifb"))]
                    { self.stack.push(0); }
                },
                0x48 => { // OP_VBE_GET_MOUSE_DOWN
                    #[cfg(feature = "minifb")]
                    {
                        let mut mdown = 0;
                        if let Some(send_window) = &self.window {
                            let window = &send_window.0;
                            if window.get_mouse_down(minifb::MouseButton::Left) {
                                mdown = 1;
                            }
                        }
                        self.stack.push(mdown);
                    }
                    #[cfg(not(feature = "minifb"))]
                    { self.stack.push(0); }
                },
                
                // Image Intrinsics (Mock/Simple)
                0x31 => { // OP_IMG_ALLOC
                    let h = self.stack.pop().unwrap() as usize;
                    let w = self.stack.pop().unwrap() as usize;
                    let bytes = w * h * 8; // Each field/pixel is 8 bytes
                    let ptr = self.shared.heap_ptr.fetch_add(bytes, std::sync::atomic::Ordering::SeqCst);
                    {
                        let mut memory = self.shared.memory.write().unwrap();
                        if ptr + bytes > memory.len() {
                            let new_len = (ptr + bytes).max(memory.len() * 2);
                            memory.resize(new_len, 0);
                        }
                    }
                    self.stack.push(ptr as i64);
                },
                
                0x20 => { // DRAW_RECT (stub)
                    self.stack.pop(); // color
                    self.stack.pop(); // h
                    self.stack.pop(); // w
                    self.stack.pop(); // y
                    self.stack.pop(); // x
                },
                
                // HW I/O stubs
                0xF0 => { // hw_read: pop addr, push simulated value
                    let addr = self.stack.pop().unwrap_or(0);
                    // Simulate hardware entropy using address + thread ID trick
                    let pseudo = (addr.wrapping_mul(6364136223846793005i64).wrapping_add(1442695040888963407i64) >> 33) & 0xFF;
                    self.stack.push(pseudo);
                },
                0xF1 => { // hw_write: pop addr and val, no-op
                    self.stack.pop(); // val
                    self.stack.pop(); // addr
                },
                
                // Sync primitives (no-op stubs for single-threaded execution)
                0x73 => { // LOCK (no-op stub)
                    self.stack.pop(); // addr
                },
                0x74 => { // UNLOCK (no-op stub)
                    self.stack.pop(); // addr
                },
                
                                  0xB0 => { // OP_NET_LISTEN
                      let port = self.stack.pop().unwrap_or(0) as u16;
                      let mut lock = self.shared.listeners.write().unwrap();
                      let id = lock.len();
                      if let Ok(listener) = std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                          lock.push(NuxListener::Tcp(listener));
                          self.stack.push(id as i64);
                      } else {
                          self.stack.push(-1);
                      }
                  },
                                  0xB1 => { // OP_NET_ACCEPT
                      let id = self.stack.pop().unwrap_or(0) as usize;
                      let lock = self.shared.listeners.read().unwrap();
                      if id < lock.len() {
                          if let Ok(stream) = lock[id].accept() {
                              let mut conn_lock = self.shared.connections.write().unwrap();
                              let conn_id = conn_lock.len();
                              conn_lock.push(stream);
                              self.stack.push(conn_id as i64);
                          } else {
                              self.stack.push(-1);
                          }
                      } else {
                          self.stack.push(-1);
                      }
                  },
                0xB2 => { // OP_NET_READ (conn_id) -> pushes string
                    let id = self.stack.pop().unwrap_or(0) as usize;
                    let mut lock = self.shared.connections.write().unwrap();
                    if id < lock.len() {
                        use std::io::Read;
                        let mut buf = vec![0; 4096];
                        let n = lock[id].read(&mut buf).unwrap_or(0);
                        buf.truncate(n);
                        let s = String::from_utf8_lossy(&buf).to_string();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        heap_strings.push(s);
                        self.stack.push((heap_strings.len() - 1) as i64);
                    } else {
                        self.stack.push(0);
                    }
                },
                0xB3 => { // OP_NET_WRITE (conn_id, str_id)
                    let str_id = self.stack.pop().unwrap_or(0) as usize;
                    let id = self.stack.pop().unwrap_or(0) as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    let data = if str_id < heap_strings.len() {
                        heap_strings[str_id].clone()
                    } else {
                        String::new()
                    };
                    drop(heap_strings);
                    
                    let mut lock = self.shared.connections.write().unwrap();
                    if id < lock.len() {
                        use std::io::Write;
                        lock[id].write_all(data.as_bytes()).unwrap_or(());
                        lock[id].flush().unwrap_or(());
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                0xB4 => { // OP_NET_CLOSE (conn_id)
                    let id = self.stack.pop().unwrap_or(0) as usize;
                    let mut lock = self.shared.connections.write().unwrap();
                    if id < lock.len() {
                        use std::net::Shutdown;
                        lock[id].shutdown(Shutdown::Both).unwrap_or(());
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                                                                                        0xC0 => { // OP_FS_READ
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                        }
                        let content = std::fs::read_to_string(path).unwrap_or_default();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(content);
                        self.stack.push(id as i64);
                    }
                    0xC1 => { // OP_FS_WRITE
                        let data_id = self.stack.pop().unwrap_or(0) as usize;
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        let mut data = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                            if data_id < heap_strings.len() { data = heap_strings[data_id].clone(); }
                        }
                        let res = std::fs::write(path, data).is_ok();
                        self.stack.push(if res { 1 } else { 0 });
                    }
                    0xC2 => { // OP_FS_EXISTS
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                        }
                        let res = std::path::Path::new(&path).exists();
                        self.stack.push(if res { 1 } else { 0 });
                    }
                    0xC5 => { // OP_OS_ENV
                        let key_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut key = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if key_id < heap_strings.len() { key = heap_strings[key_id].clone(); }
                        }
                        let val = std::env::var(key).unwrap_or_default();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xC6 => { // OP_OS_CWD
                        let val = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xC7 => { // OP_OS_EXEC
                        let cmd_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut cmd = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if cmd_id < heap_strings.len() { cmd = heap_strings[cmd_id].clone(); }
                        }
                        let output = Command::new("cmd")
                            .args(&["/C", &cmd])
                            .output();
                        let val = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xCA => { // OP_TIME_NOW
                        let start = SystemTime::now();
                        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                        self.stack.push(since_the_epoch.as_millis() as i64);
                    }
                    0xCB => { // OP_TIME_SLEEP
                        let ms = self.stack.pop().unwrap_or(0) as u64;
                        std::thread::sleep(std::time::Duration::from_millis(ms));
                    }
                    0xB5 => { // OP_NET_LISTEN_TLS
                       let key_file_id = self.stack.pop().unwrap_or(0) as usize;
                       let cert_file_id = self.stack.pop().unwrap_or(0) as usize;
                       let port = self.stack.pop().unwrap_or(0) as u16;
                       
                       let mut cert_file = String::new();
                       let mut key_file = String::new();
                       
                       {
                           let heap_strings = self.shared.heap_strings.read().unwrap();
                           if cert_file_id < heap_strings.len() {
                               cert_file = heap_strings[cert_file_id].clone();
                           }
                           if key_file_id < heap_strings.len() {
                               key_file = heap_strings[key_file_id].clone();
                           }
                       }
                       
                       let mut lock = self.shared.listeners.write().unwrap();
                       let id = lock.len();
                       
                       eprintln!("DEBUG: cert_file = '{}', key_file = '{}'", cert_file, key_file);
                       
                       let certs_res = (|| -> Result<Vec<CertificateDer<'static>>, Box<dyn std::error::Error>> {
                           let mut reader = BufReader::new(File::open(&cert_file)?);
                           let certs: Result<Vec<_>, _> = certs(&mut reader).collect();
                           Ok(certs?)
                       })();
                       
                       let key_res = (|| -> Result<PrivateKeyDer<'static>, Box<dyn std::error::Error>> {
                           let mut reader = BufReader::new(File::open(&key_file)?);
                           Ok(private_key(&mut reader)?.ok_or("No private key found")?)
                       })();
                       
                       match (certs_res, key_res) {
                           (Ok(certs_der), Ok(key_der)) => {
                               match ServerConfig::builder().with_no_client_auth().with_single_cert(certs_der, key_der) {
                                   Ok(config) => {
                                       match std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                                           Ok(listener) => {
                                               lock.push(NuxListener::Tls(listener, Arc::new(config)));
                                               self.stack.push(id as i64);
                                           },
                                           Err(e) => {
                                               eprintln!("Failed to bind to port {}: {}", port, e);
                                               self.stack.push(-1);
                                           }
                                       }
                                   },
                                   Err(e) => {
                                       eprintln!("Failed to configure TLS: {}", e);
                                       self.stack.push(-1);
                                   }
                               }
                           },
                           (Err(e1), Err(e2)) => {
                               eprintln!("Failed to read cert file: {}", e1);
                               eprintln!("Failed to read key file: {}", e2);
                               self.stack.push(-1);
                           },
                           (Err(e), _) => {
                               eprintln!("Failed to read cert file: {}", e);
                               self.stack.push(-1);
                           },
                           (_, Err(e)) => {
                               eprintln!("Failed to read key file: {}", e);
                               self.stack.push(-1);
                           }
                       }
                  },

                  _ => { 
                    // eprintln!("Unknown OpCode: 0x{:02X} at {}", op, self.ip - 1);
                }
            }
        }
    }
}
