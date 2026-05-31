// NVM Bytecode - Bytecode instruction set and generation for Nux Virtual Machine
// Similar to JVM bytecode or Python bytecode

use std::collections::HashMap;
use std::fmt;

/// Bytecode instruction set for NVM
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // Stack operations
    PUSH = 0x01,          // Push constant onto stack
    POP = 0x02,           // Pop value from stack
    DUP = 0x03,           // Duplicate top of stack
    SWAP = 0x04,          // Swap top two stack values
    
    // Arithmetic operations
    ADD = 0x10,           // Add two numbers
    SUB = 0x11,           // Subtract
    MUL = 0x12,           // Multiply
    DIV = 0x13,           // Divide
    MOD = 0x14,           // Modulo
    NEG = 0x15,           // Negate
    
    // Comparison operations
    EQ = 0x20,            // Equal
    NE = 0x21,            // Not equal
    LT = 0x22,            // Less than
    GT = 0x23,            // Greater than
    LE = 0x24,            // Less than or equal
    GE = 0x25,            // Greater than or equal
    
    // Logical operations
    AND = 0x30,           // Logical AND
    OR = 0x31,            // Logical OR
    NOT = 0x32,           // Logical NOT
    
    // Control flow
    JUMP = 0x40,          // Unconditional jump
    JUMP_IF_TRUE = 0x41,  // Jump if true
    JUMP_IF_FALSE = 0x42, // Jump if false
    CALL = 0x43,          // Call function
    RETURN = 0x44,        // Return from function
    
    // Variable operations
    LOAD_CONST = 0x50,    // Load constant from constant pool
    LOAD_VAR = 0x51,      // Load variable
    STORE_VAR = 0x52,     // Store variable
    LOAD_GLOBAL = 0x53,   // Load global variable
    STORE_GLOBAL = 0x54,  // Store global variable
    
    // Object operations
    LOAD_ATTR = 0x60,     // Load attribute
    STORE_ATTR = 0x61,    // Store attribute
    NEW_ARRAY = 0x62,     // Create new array
    NEW_MAP = 0x63,       // Create new map
    LOAD_INDEX = 0x64,    // Load array/map element
    STORE_INDEX = 0x65,   // Store array/map element
    
    // Foreign function calls
    CALL_PYTHON = 0x70,   // Call Python function
    CALL_JAVASCRIPT = 0x71, // Call JavaScript function
    CALL_RUST = 0x72,     // Call Rust function
    CALL_C = 0x73,        // Call C function
    
    // Exception handling
    TRY = 0x80,           // Begin try block
    CATCH = 0x81,         // Begin catch block
    THROW = 0x82,         // Throw exception
    FINALLY = 0x83,       // Begin finally block
    
    // Concurrency
    SPAWN_THREAD = 0x90,  // Spawn new thread
    JOIN_THREAD = 0x91,   // Join thread
    LOCK = 0x92,          // Acquire lock
    UNLOCK = 0x93,        // Release lock
    
    // Special
    NOP = 0xF0,           // No operation
    HALT = 0xFF,          // Halt execution
}

impl Opcode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::PUSH),
            0x02 => Some(Opcode::POP),
            0x03 => Some(Opcode::DUP),
            0x04 => Some(Opcode::SWAP),
            0x10 => Some(Opcode::ADD),
            0x11 => Some(Opcode::SUB),
            0x12 => Some(Opcode::MUL),
            0x13 => Some(Opcode::DIV),
            0x14 => Some(Opcode::MOD),
            0x15 => Some(Opcode::NEG),
            0x20 => Some(Opcode::EQ),
            0x21 => Some(Opcode::NE),
            0x22 => Some(Opcode::LT),
            0x23 => Some(Opcode::GT),
            0x24 => Some(Opcode::LE),
            0x25 => Some(Opcode::GE),
            0x30 => Some(Opcode::AND),
            0x31 => Some(Opcode::OR),
            0x32 => Some(Opcode::NOT),
            0x40 => Some(Opcode::JUMP),
            0x41 => Some(Opcode::JUMP_IF_TRUE),
            0x42 => Some(Opcode::JUMP_IF_FALSE),
            0x43 => Some(Opcode::CALL),
            0x44 => Some(Opcode::RETURN),
            0x50 => Some(Opcode::LOAD_CONST),
            0x51 => Some(Opcode::LOAD_VAR),
            0x52 => Some(Opcode::STORE_VAR),
            0x53 => Some(Opcode::LOAD_GLOBAL),
            0x54 => Some(Opcode::STORE_GLOBAL),
            0x60 => Some(Opcode::LOAD_ATTR),
            0x61 => Some(Opcode::STORE_ATTR),
            0x62 => Some(Opcode::NEW_ARRAY),
            0x63 => Some(Opcode::NEW_MAP),
            0x64 => Some(Opcode::LOAD_INDEX),
            0x65 => Some(Opcode::STORE_INDEX),
            0x70 => Some(Opcode::CALL_PYTHON),
            0x71 => Some(Opcode::CALL_JAVASCRIPT),
            0x72 => Some(Opcode::CALL_RUST),
            0x73 => Some(Opcode::CALL_C),
            0x80 => Some(Opcode::TRY),
            0x81 => Some(Opcode::CATCH),
            0x82 => Some(Opcode::THROW),
            0x83 => Some(Opcode::FINALLY),
            0x90 => Some(Opcode::SPAWN_THREAD),
            0x91 => Some(Opcode::JOIN_THREAD),
            0x92 => Some(Opcode::LOCK),
            0x93 => Some(Opcode::UNLOCK),
            0xF0 => Some(Opcode::NOP),
            0xFF => Some(Opcode::HALT),
            _ => None,
        }
    }
}

/// Bytecode value types
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Function(usize), // Function ID
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Function(id) => write!(f, "<function #{}>", id),
        }
    }
}

/// Bytecode chunk - represents a compiled function or module
#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    pub name: String,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub line_numbers: Vec<usize>,
}

impl BytecodeChunk {
    pub fn new(name: String) -> Self {
        BytecodeChunk {
            name,
            code: Vec::new(),
            constants: Vec::new(),
            line_numbers: Vec::new(),
        }
    }

    /// Emit a single opcode
    pub fn emit(&mut self, opcode: Opcode, line: usize) {
        self.code.push(opcode as u8);
        self.line_numbers.push(line);
    }

    /// Emit an opcode with a byte argument
    pub fn emit_byte(&mut self, opcode: Opcode, arg: u8, line: usize) {
        self.code.push(opcode as u8);
        self.code.push(arg);
        self.line_numbers.push(line);
        self.line_numbers.push(line);
    }

    /// Emit an opcode with a u16 argument
    pub fn emit_u16(&mut self, opcode: Opcode, arg: u16, line: usize) {
        self.code.push(opcode as u8);
        self.code.push((arg >> 8) as u8);
        self.code.push((arg & 0xFF) as u8);
        self.line_numbers.push(line);
        self.line_numbers.push(line);
        self.line_numbers.push(line);
    }

    /// Add a constant to the constant pool
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Disassemble bytecode for debugging
    pub fn disassemble(&self) {
        println!("=== {} ===", self.name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        
        if offset > 0 && self.line_numbers[offset] == self.line_numbers[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.line_numbers[offset]);
        }

        let opcode = Opcode::from_u8(self.code[offset]);
        match opcode {
            Some(op) => {
                match op {
                    Opcode::LOAD_CONST | Opcode::LOAD_VAR | Opcode::STORE_VAR |
                    Opcode::LOAD_GLOBAL | Opcode::STORE_GLOBAL => {
                        let const_idx = self.code[offset + 1];
                        println!("{:?} {}", op, const_idx);
                        offset + 2
                    }
                    Opcode::JUMP | Opcode::JUMP_IF_TRUE | Opcode::JUMP_IF_FALSE => {
                        let jump_offset = ((self.code[offset + 1] as u16) << 8) | (self.code[offset + 2] as u16);
                        println!("{:?} -> {}", op, offset as u16 + jump_offset);
                        offset + 3
                    }
                    _ => {
                        println!("{:?}", op);
                        offset + 1
                    }
                }
            }
            None => {
                println!("Unknown opcode: {}", self.code[offset]);
                offset + 1
            }
        }
    }
}

/// Bytecode compiler - compiles AST to bytecode
pub struct BytecodeCompiler {
    chunk: BytecodeChunk,
    locals: Vec<String>,
    scope_depth: usize,
}

impl BytecodeCompiler {
    pub fn new(name: String) -> Self {
        BytecodeCompiler {
            chunk: BytecodeChunk::new(name),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    /// Compile a literal value
    pub fn compile_literal(&mut self, value: Value, line: usize) {
        let const_idx = self.chunk.add_constant(value);
        self.chunk.emit_byte(Opcode::LOAD_CONST, const_idx as u8, line);
    }

    /// Compile a binary operation
    pub fn compile_binary_op(&mut self, op: &str, line: usize) {
        let opcode = match op {
            "+" => Opcode::ADD,
            "-" => Opcode::SUB,
            "*" => Opcode::MUL,
            "/" => Opcode::DIV,
            "%" => Opcode::MOD,
            "==" => Opcode::EQ,
            "!=" => Opcode::NE,
            "<" => Opcode::LT,
            ">" => Opcode::GT,
            "<=" => Opcode::LE,
            ">=" => Opcode::GE,
            "&&" => Opcode::AND,
            "||" => Opcode::OR,
            _ => panic!("Unknown binary operator: {}", op),
        };
        self.chunk.emit(opcode, line);
    }

    /// Compile a variable load
    pub fn compile_load_var(&mut self, name: &str, line: usize) {
        if let Some(idx) = self.resolve_local(name) {
            self.chunk.emit_byte(Opcode::LOAD_VAR, idx as u8, line);
        } else {
            let const_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.emit_byte(Opcode::LOAD_GLOBAL, const_idx as u8, line);
        }
    }

    /// Compile a variable store
    pub fn compile_store_var(&mut self, name: &str, line: usize) {
        if let Some(idx) = self.resolve_local(name) {
            self.chunk.emit_byte(Opcode::STORE_VAR, idx as u8, line);
        } else {
            let const_idx = self.chunk.add_constant(Value::String(name.to_string()));
            self.chunk.emit_byte(Opcode::STORE_GLOBAL, const_idx as u8, line);
        }
    }

    /// Compile a function call
    pub fn compile_call(&mut self, arg_count: u8, line: usize) {
        self.chunk.emit_byte(Opcode::CALL, arg_count, line);
    }

    /// Compile a return statement
    pub fn compile_return(&mut self, line: usize) {
        self.chunk.emit(Opcode::RETURN, line);
    }

    /// Begin a new scope
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// End current scope
    pub fn end_scope(&mut self, line: usize) {
        self.scope_depth -= 1;
        
        // Pop locals from this scope
        while !self.locals.is_empty() {
            self.chunk.emit(Opcode::POP, line);
            self.locals.pop();
        }
    }

    /// Add a local variable
    pub fn add_local(&mut self, name: String) {
        self.locals.push(name);
    }

    /// Resolve a local variable
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == name {
                return Some(i);
            }
        }
        None
    }

    /// Get the compiled bytecode chunk
    pub fn finish(self) -> BytecodeChunk {
        self.chunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecode_compilation() {
        let mut compiler = BytecodeCompiler::new("test".to_string());
        
        // Compile: 1 + 2
        compiler.compile_literal(Value::Int(1), 1);
        compiler.compile_literal(Value::Int(2), 1);
        compiler.compile_binary_op("+", 1);
        compiler.compile_return(1);
        
        let chunk = compiler.finish();
        assert_eq!(chunk.code.len(), 6); // LOAD_CONST, LOAD_CONST, ADD, RETURN
    }

    #[test]
    fn test_disassembly() {
        let mut compiler = BytecodeCompiler::new("test".to_string());
        compiler.compile_literal(Value::Int(42), 1);
        compiler.compile_return(1);
        
        let chunk = compiler.finish();
        chunk.disassemble();
    }
}
