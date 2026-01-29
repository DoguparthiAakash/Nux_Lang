use crate::vm::opcodes::*;
use crate::hal::HardwareAbstraction;

const STACK_SIZE: usize = 256;
const CALL_STACK_SIZE: usize = 32;
const MEMORY_SIZE: usize = 1024; // 1KB for variables

pub struct NuxEmbeddedVm<'a, H: HardwareAbstraction> {
    // Execution state
    stack: [i32; STACK_SIZE],
    sp: usize, // Stack pointer
    
    call_stack: [(usize, usize); CALL_STACK_SIZE], // (return_ip, return_fp)
    call_sp: usize,
    
    ip: usize, // Instruction pointer
    fp: usize, // Frame pointer
    
    // Memory
    memory: [u8; MEMORY_SIZE],
    
    // Bytecode (stored in flash, not copied to RAM)
    code: &'a [u8],
    
    // Hardware abstraction
    hal: H,
    
    // Status
    running: bool,
}

impl<'a, H: HardwareAbstraction> NuxEmbeddedVm<'a, H> {
    pub fn new(code: &'a [u8], hal: H) -> Self {
        Self {
            stack: [0; STACK_SIZE],
            sp: 0,
            call_stack: [(0, 0); CALL_STACK_SIZE],
            call_sp: 0,
            ip: 64, // Skip 64-byte header
            fp: 0,
            memory: [0; MEMORY_SIZE],
            code,
            hal,
            running: false,
        }
    }
    
    #[inline]
    fn push(&mut self, val: i32) -> Result<(), &'static str> {
        if self.sp >= STACK_SIZE {
            return Err("Stack overflow");
        }
        self.stack[self.sp] = val;
        self.sp += 1;
        Ok(())
    }
    
    #[inline]
    fn pop(&mut self) -> Result<i32, &'static str> {
        if self.sp == 0 {
            return Err("Stack underflow");
        }
        self.sp -= 1;
        Ok(self.stack[self.sp])
    }
    
    #[inline]
    fn read_i32(&mut self) -> i32 {
        if self.ip + 4 > self.code.len() {
            return 0;
        }
        let bytes = [
            self.code[self.ip],
            self.code[self.ip + 1],
            self.code[self.ip + 2],
            self.code[self.ip + 3],
        ];
        self.ip += 4;
        i32::from_le_bytes(bytes)
    }
    
    pub fn run(&mut self) -> Result<(), &'static str> {
        self.running = true;
        
        while self.running && self.ip < self.code.len() {
            let op = self.code[self.ip];
            self.ip += 1;
            
            match op {
                OP_PUSH => {
                    let val = self.read_i32();
                    self.push(val)?;
                }
                
                OP_POP => {
                    self.pop()?;
                }
                
                OP_SWAP => {
                    if self.sp < 2 {
                        return Err("Not enough values to swap");
                    }
                    let a = self.stack[self.sp - 1];
                    let b = self.stack[self.sp - 2];
                    self.stack[self.sp - 1] = b;
                    self.stack[self.sp - 2] = a;
                }
                
                OP_DUP => {
                    if self.sp == 0 {
                        return Err("Cannot duplicate empty stack");
                    }
                    let val = self.stack[self.sp - 1];
                    self.push(val)?;
                }
                
                OP_ADD => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a.wrapping_add(b))?;
                }
                
                OP_SUB => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a.wrapping_sub(b))?;
                }
                
                OP_MUL => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a.wrapping_mul(b))?;
                }
                
                OP_DIV => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err("Division by zero");
                    }
                    self.push(a / b)?;
                }
                
                OP_MOD => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err("Modulo by zero");
                    }
                    self.push(a % b)?;
                }
                
                OP_EQ => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a == b { 1 } else { 0 })?;
                }
                
                OP_NEQ => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a != b { 1 } else { 0 })?;
                }
                
                OP_LT => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a < b { 1 } else { 0 })?;
                }
                
                OP_GT => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a > b { 1 } else { 0 })?;
                }
                
                OP_LTE => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a <= b { 1 } else { 0 })?;
                }
                
                OP_GTE => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a >= b { 1 } else { 0 })?;
                }
                
                OP_JMP => {
                    let target = self.read_i32() as usize;
                    self.ip = target;
                }
                
                OP_JE => {
                    let target = self.read_i32() as usize;
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if a == b {
                        self.ip = target;
                    }
                }
                
                OP_CALL => {
                    let target = self.read_i32() as usize;
                    let num_args = self.read_i32() as usize;
                    
                    if self.call_sp >= CALL_STACK_SIZE {
                        return Err("Call stack overflow");
                    }
                    
                    self.call_stack[self.call_sp] = (self.ip, self.fp);
                    self.call_sp += 1;
                    
                    if self.sp < num_args {
                        return Err("Not enough arguments");
                    }
                    
                    self.fp = self.sp - num_args;
                    self.ip = target;
                }
                
                OP_RET => {
                    if self.call_sp == 0 {
                        self.running = false;
                        break;
                    }
                    
                    self.call_sp -= 1;
                    let (ret_ip, ret_fp) = self.call_stack[self.call_sp];
                    
                    let ret_val = self.pop()?;
                    self.sp = self.fp;
                    self.push(ret_val)?;
                    
                    self.ip = ret_ip;
                    self.fp = ret_fp;
                }
                
                OP_GET_LOCAL => {
                    let offset = self.read_i32() as usize;
                    let idx = self.fp + offset;
                    if idx >= self.sp {
                        return Err("Invalid local access");
                    }
                    self.push(self.stack[idx])?;
                }
                
                OP_SET_LOCAL => {
                    let offset = self.read_i32() as usize;
                    let idx = self.fp + offset;
                    if idx >= STACK_SIZE {
                        return Err("Invalid local write");
                    }
                    let val = self.pop()?;
                    self.stack[idx] = val;
                }
                
                OP_PEEK => {
                    let addr = self.pop()? as usize;
                    if addr >= MEMORY_SIZE {
                        return Err("Memory access out of bounds");
                    }
                    self.push(self.memory[addr] as i32)?;
                }
                
                OP_POKE => {
                    let val = self.pop()?;
                    let addr = self.pop()? as usize;
                    if addr >= MEMORY_SIZE {
                        return Err("Memory write out of bounds");
                    }
                    self.memory[addr] = val as u8;
                }
                
                OP_PRINT_VAL => {
                    let val = self.pop()?;
                    self.hal.print_int(val);
                }
                
                OP_PRINT_CHAR => {
                    let val = self.pop()?;
                    self.hal.print_char(val as u8 as char);
                }
                
                // Embedded-specific opcodes
                OP_GPIO_WRITE => {
                    let value = self.pop()? != 0;
                    let pin = self.pop()? as u8;
                    self.hal.gpio_write(pin, value)?;
                }
                
                OP_GPIO_READ => {
                    let pin = self.pop()? as u8;
                    let value = self.hal.gpio_read(pin)?;
                    self.push(if value { 1 } else { 0 })?;
                }
                
                OP_GPIO_MODE => {
                    let mode = self.pop()? as u8;
                    let pin = self.pop()? as u8;
                    self.hal.gpio_set_mode(pin, mode)?;
                }
                
                OP_ANALOG_READ => {
                    let pin = self.pop()? as u8;
                    let value = self.hal.analog_read(pin)?;
                    self.push(value as i32)?;
                }
                
                OP_PWM_WRITE => {
                    let duty = self.pop()? as u16;
                    let pin = self.pop()? as u8;
                    self.hal.pwm_write(pin, duty)?;
                }
                
                OP_DELAY_MS => {
                    let ms = self.pop()? as u32;
                    self.hal.delay_ms(ms);
                }
                
                OP_DELAY_US => {
                    let us = self.pop()? as u32;
                    self.hal.delay_us(us);
                }
                
                OP_MILLIS => {
                    let ms = self.hal.millis();
                    self.push(ms as i32)?;
                }
                
                OP_MICROS => {
                    let us = self.hal.micros();
                    self.push(us as i32)?;
                }
                
                OP_EXIT => {
                    self.running = false;
                }
                
                _ => {
                    return Err("Unknown opcode");
                }
            }
        }
        
        Ok(())
    }
}
