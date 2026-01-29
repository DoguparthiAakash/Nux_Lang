// NVM - Nux Virtual Machine
// Stack-based bytecode interpreter similar to JVM/Python VM

use super::bytecode::{Opcode, Value, BytecodeChunk};
use std::collections::HashMap;

/// Nux Virtual Machine
pub struct NuxVM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    call_stack: Vec<CallFrame>,
    ip: usize, // Instruction pointer
}

/// Call frame for function calls
struct CallFrame {
    chunk: BytecodeChunk,
    ip: usize,
    stack_offset: usize,
}

impl NuxVM {
    pub fn new() -> Self {
        NuxVM {
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
            call_stack: Vec::new(),
            ip: 0,
        }
    }

    /// Execute a bytecode chunk
    pub fn execute(&mut self, chunk: BytecodeChunk) -> Result<Value, VMError> {
        self.call_stack.push(CallFrame {
            chunk,
            ip: 0,
            stack_offset: self.stack.len(),
        });

        loop {
            let frame = self.call_stack.last_mut().unwrap();
            
            if frame.ip >= frame.chunk.code.len() {
                break;
            }

            let opcode = Opcode::from_u8(frame.chunk.code[frame.ip])
                .ok_or_else(|| VMError::InvalidOpcode(frame.chunk.code[frame.ip]))?;
            
            frame.ip += 1;

            match opcode {
                Opcode::PUSH => self.op_push()?,
                Opcode::POP => self.op_pop()?,
                Opcode::DUP => self.op_dup()?,
                Opcode::SWAP => self.op_swap()?,
                
                Opcode::ADD => self.op_add()?,
                Opcode::SUB => self.op_sub()?,
                Opcode::MUL => self.op_mul()?,
                Opcode::DIV => self.op_div()?,
                Opcode::MOD => self.op_mod()?,
                Opcode::NEG => self.op_neg()?,
                
                Opcode::EQ => self.op_eq()?,
                Opcode::NE => self.op_ne()?,
                Opcode::LT => self.op_lt()?,
                Opcode::GT => self.op_gt()?,
                Opcode::LE => self.op_le()?,
                Opcode::GE => self.op_ge()?,
                
                Opcode::AND => self.op_and()?,
                Opcode::OR => self.op_or()?,
                Opcode::NOT => self.op_not()?,
                
                Opcode::JUMP => self.op_jump()?,
                Opcode::JUMP_IF_TRUE => self.op_jump_if_true()?,
                Opcode::JUMP_IF_FALSE => self.op_jump_if_false()?,
                Opcode::CALL => self.op_call()?,
                Opcode::RETURN => {
                    if self.call_stack.len() == 1 {
                        break;
                    }
                    self.op_return()?;
                }
                
                Opcode::LOAD_CONST => self.op_load_const()?,
                Opcode::LOAD_VAR => self.op_load_var()?,
                Opcode::STORE_VAR => self.op_store_var()?,
                Opcode::LOAD_GLOBAL => self.op_load_global()?,
                Opcode::STORE_GLOBAL => self.op_store_global()?,
                
                Opcode::LOAD_ATTR => self.op_load_attr()?,
                Opcode::STORE_ATTR => self.op_store_attr()?,
                Opcode::NEW_ARRAY => self.op_new_array()?,
                Opcode::NEW_MAP => self.op_new_map()?,
                Opcode::LOAD_INDEX => self.op_load_index()?,
                Opcode::STORE_INDEX => self.op_store_index()?,
                
                Opcode::HALT => break,
                Opcode::NOP => {}
                
                _ => return Err(VMError::UnimplementedOpcode(opcode)),
            }
        }

        // Return top of stack or null
        Ok(self.stack.pop().unwrap_or(Value::Null))
    }

    // Stack operations
    fn op_push(&mut self) -> Result<(), VMError> {
        // Push is handled by LOAD_CONST
        Ok(())
    }

    fn op_pop(&mut self) -> Result<(), VMError> {
        self.stack.pop().ok_or(VMError::StackUnderflow)?;
        Ok(())
    }

    fn op_dup(&mut self) -> Result<(), VMError> {
        let value = self.stack.last().ok_or(VMError::StackUnderflow)?.clone();
        self.stack.push(value);
        Ok(())
    }

    fn op_swap(&mut self) -> Result<(), VMError> {
        let len = self.stack.len();
        if len < 2 {
            return Err(VMError::StackUnderflow);
        }
        self.stack.swap(len - 1, len - 2);
        Ok(())
    }

    // Arithmetic operations
    fn op_add(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 + y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x + y as f64),
            (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
            _ => return Err(VMError::TypeError("Cannot add these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_sub(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
            (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 - y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x - y as f64),
            _ => return Err(VMError::TypeError("Cannot subtract these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_mul(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
            (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 * y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x * y as f64),
            _ => return Err(VMError::TypeError("Cannot multiply these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_div(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == 0 {
                    return Err(VMError::DivisionByZero);
                }
                Value::Int(x / y)
            }
            (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
            (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 / y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x / y as f64),
            _ => return Err(VMError::TypeError("Cannot divide these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_mod(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == 0 {
                    return Err(VMError::DivisionByZero);
                }
                Value::Int(x % y)
            }
            _ => return Err(VMError::TypeError("Modulo only works on integers".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_neg(&mut self) -> Result<(), VMError> {
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match value {
            Value::Int(x) => Value::Int(-x),
            Value::Float(x) => Value::Float(-x),
            _ => return Err(VMError::TypeError("Cannot negate this type".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    // Comparison operations
    fn op_eq(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Bool(x == y),
            (Value::Float(x), Value::Float(y)) => Value::Bool(x == y),
            (Value::String(x), Value::String(y)) => Value::Bool(x == y),
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x == y),
            (Value::Null, Value::Null) => Value::Bool(true),
            _ => Value::Bool(false),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_ne(&mut self) -> Result<(), VMError> {
        self.op_eq()?;
        self.op_not()
    }

    fn op_lt(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Bool(x < y),
            (Value::Float(x), Value::Float(y)) => Value::Bool(x < y),
            _ => return Err(VMError::TypeError("Cannot compare these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_gt(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Bool(x > y),
            (Value::Float(x), Value::Float(y)) => Value::Bool(x > y),
            _ => return Err(VMError::TypeError("Cannot compare these types".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_le(&mut self) -> Result<(), VMError> {
        self.op_gt()?;
        self.op_not()
    }

    fn op_ge(&mut self) -> Result<(), VMError> {
        self.op_lt()?;
        self.op_not()
    }

    // Logical operations
    fn op_and(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x && y),
            _ => return Err(VMError::TypeError("AND requires boolean operands".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_or(&mut self) -> Result<(), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x || y),
            _ => return Err(VMError::TypeError("OR requires boolean operands".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    fn op_not(&mut self) -> Result<(), VMError> {
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        let result = match value {
            Value::Bool(x) => Value::Bool(!x),
            _ => return Err(VMError::TypeError("NOT requires boolean operand".to_string())),
        };
        
        self.stack.push(result);
        Ok(())
    }

    // Control flow (simplified implementations)
    fn op_jump(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let offset = ((frame.chunk.code[frame.ip] as u16) << 8) | (frame.chunk.code[frame.ip + 1] as u16);
        frame.ip = offset as usize;
        Ok(())
    }

    fn op_jump_if_true(&mut self) -> Result<(), VMError> {
        let condition = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        if let Value::Bool(true) = condition {
            self.op_jump()?;
        } else {
            let frame = self.call_stack.last_mut().unwrap();
            frame.ip += 2;
        }
        
        Ok(())
    }

    fn op_jump_if_false(&mut self) -> Result<(), VMError> {
        let condition = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        
        if let Value::Bool(false) = condition {
            self.op_jump()?;
        } else {
            let frame = self.call_stack.last_mut().unwrap();
            frame.ip += 2;
        }
        
        Ok(())
    }

    fn op_call(&mut self) -> Result<(), VMError> {
        // Simplified - would need full function call implementation
        Ok(())
    }

    fn op_return(&mut self) -> Result<(), VMError> {
        let return_value = self.stack.pop().unwrap_or(Value::Null);
        self.call_stack.pop();
        self.stack.push(return_value);
        Ok(())
    }

    // Variable operations
    fn op_load_const(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let const_idx = frame.chunk.code[frame.ip] as usize;
        frame.ip += 1;
        
        let value = frame.chunk.constants[const_idx].clone();
        self.stack.push(value);
        Ok(())
    }

    fn op_load_var(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let var_idx = frame.chunk.code[frame.ip] as usize;
        frame.ip += 1;
        
        let value = self.stack[frame.stack_offset + var_idx].clone();
        self.stack.push(value);
        Ok(())
    }

    fn op_store_var(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let var_idx = frame.chunk.code[frame.ip] as usize;
        frame.ip += 1;
        
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        self.stack[frame.stack_offset + var_idx] = value;
        Ok(())
    }

    fn op_load_global(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let name_idx = frame.chunk.code[frame.ip] as usize;
        frame.ip += 1;
        
        let name = match &frame.chunk.constants[name_idx] {
            Value::String(s) => s.clone(),
            _ => return Err(VMError::TypeError("Global name must be a string".to_string())),
        };
        
        let value = self.globals.get(&name)
            .ok_or_else(|| VMError::UndefinedVariable(name.clone()))?
            .clone();
        
        self.stack.push(value);
        Ok(())
    }

    fn op_store_global(&mut self) -> Result<(), VMError> {
        let frame = self.call_stack.last_mut().unwrap();
        let name_idx = frame.chunk.code[frame.ip] as usize;
        frame.ip += 1;
        
        let name = match &frame.chunk.constants[name_idx] {
            Value::String(s) => s.clone(),
            _ => return Err(VMError::TypeError("Global name must be a string".to_string())),
        };
        
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        self.globals.insert(name, value);
        Ok(())
    }

    // Object operations (simplified)
    fn op_load_attr(&mut self) -> Result<(), VMError> {
        // Simplified implementation
        Ok(())
    }

    fn op_store_attr(&mut self) -> Result<(), VMError> {
        // Simplified implementation
        Ok(())
    }

    fn op_new_array(&mut self) -> Result<(), VMError> {
        self.stack.push(Value::Array(Vec::new()));
        Ok(())
    }

    fn op_new_map(&mut self) -> Result<(), VMError> {
        self.stack.push(Value::Map(HashMap::new()));
        Ok(())
    }

    fn op_load_index(&mut self) -> Result<(), VMError> {
        // Simplified implementation
        Ok(())
    }

    fn op_store_index(&mut self) -> Result<(), VMError> {
        // Simplified implementation
        Ok(())
    }
}

/// VM errors
#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    InvalidOpcode(u8),
    UnimplementedOpcode(Opcode),
    TypeError(String),
    DivisionByZero,
    UndefinedVariable(String),
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
            VMError::UnimplementedOpcode(op) => write!(f, "Unimplemented opcode: {:?}", op),
            VMError::TypeError(msg) => write!(f, "Type error: {}", msg),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
        }
    }
}

impl std::error::Error for VMError {}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::bytecode::BytecodeCompiler;

    #[test]
    fn test_vm_execution() {
        let mut compiler = BytecodeCompiler::new("test".to_string());
        
        // Compile: 1 + 2
        compiler.compile_literal(Value::Int(1), 1);
        compiler.compile_literal(Value::Int(2), 1);
        compiler.compile_binary_op("+", 1);
        compiler.compile_return(1);
        
        let chunk = compiler.finish();
        
        let mut vm = NuxVM::new();
        let result = vm.execute(chunk).unwrap();
        
        match result {
            Value::Int(3) => assert!(true),
            _ => panic!("Expected Int(3), got {:?}", result),
        }
    }
}
