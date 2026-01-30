// WASM Compiler - Compile Nux bytecode to WebAssembly
// Enables running Nux in browsers and WASM runtimes

use crate::nvm::bytecode::{Opcode, BytecodeChunk, Value};
use std::collections::HashMap;

/// WASM compiler for Nux bytecode
pub struct WasmCompiler {
    module: WasmModule,
    type_section: Vec<FunctionType>,
    function_section: Vec<u32>,
    code_section: Vec<FunctionBody>,
    export_section: Vec<Export>,
}

/// WASM module structure
#[derive(Debug)]
struct WasmModule {
    magic: [u8; 4],      // 0x00 0x61 0x73 0x6D
    version: [u8; 4],    // 0x01 0x00 0x00 0x00
}

/// WASM function type
#[derive(Debug, Clone)]
struct FunctionType {
    params: Vec<ValueType>,
    results: Vec<ValueType>,
}

/// WASM value types
#[derive(Debug, Clone, Copy, PartialEq)]
enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

/// WASM function body
#[derive(Debug)]
struct FunctionBody {
    locals: Vec<(u32, ValueType)>,
    code: Vec<u8>,
}

/// WASM export
#[derive(Debug)]
struct Export {
    name: String,
    kind: ExportKind,
    index: u32,
}

#[derive(Debug)]
enum ExportKind {
    Function,
    Table,
    Memory,
    Global,
}

impl WasmCompiler {
    pub fn new() -> Self {
        WasmCompiler {
            module: WasmModule {
                magic: [0x00, 0x61, 0x73, 0x6D],
                version: [0x01, 0x00, 0x00, 0x00],
            },
            type_section: Vec::new(),
            function_section: Vec::new(),
            code_section: Vec::new(),
            export_section: Vec::new(),
        }
    }

    /// Compile Nux bytecode to WASM
    pub fn compile(&mut self, chunk: &BytecodeChunk) -> Result<Vec<u8>, WasmError> {
        println!("[WASM] Compiling {} to WebAssembly...", chunk.name);

        // Add function type
        let func_type = FunctionType {
            params: vec![],
            results: vec![ValueType::I32],
        };
        self.type_section.push(func_type);

        // Compile bytecode to WASM instructions
        let wasm_code = self.compile_bytecode(&chunk.code)?;

        // Create function body
        let func_body = FunctionBody {
            locals: vec![],
            code: wasm_code,
        };
        self.code_section.push(func_body);

        // Add function to function section
        self.function_section.push(0); // Type index

        // Export main function
        self.export_section.push(Export {
            name: "main".to_string(),
            kind: ExportKind::Function,
            index: 0,
        });

        // Generate WASM binary
        self.generate_wasm_binary()
    }

    /// Compile Nux bytecode to WASM instructions
    fn compile_bytecode(&self, code: &[u8]) -> Result<Vec<u8>, WasmError> {
        let mut wasm_code = Vec::new();
        let mut i = 0;

        while i < code.len() {
            if let Some(opcode) = Opcode::from_u8(code[i]) {
                match opcode {
                    // Arithmetic operations
                    Opcode::ADD => {
                        wasm_code.push(0x6A); // i32.add
                    }
                    Opcode::SUB => {
                        wasm_code.push(0x6B); // i32.sub
                    }
                    Opcode::MUL => {
                        wasm_code.push(0x6C); // i32.mul
                    }
                    Opcode::DIV => {
                        wasm_code.push(0x6D); // i32.div_s
                    }
                    
                    // Comparison operations
                    Opcode::EQ => {
                        wasm_code.push(0x46); // i32.eq
                    }
                    Opcode::NE => {
                        wasm_code.push(0x47); // i32.ne
                    }
                    Opcode::LT => {
                        wasm_code.push(0x48); // i32.lt_s
                    }
                    Opcode::GT => {
                        wasm_code.push(0x4A); // i32.gt_s
                    }
                    
                    // Stack operations
                    Opcode::DUP => {
                        wasm_code.push(0x20); // local.get 0
                        wasm_code.push(0x00);
                        wasm_code.push(0x20); // local.get 0
                        wasm_code.push(0x00);
                    }
                    Opcode::POP => {
                        wasm_code.push(0x1A); // drop
                    }
                    
                    // Constants
                    Opcode::LOAD_CONST => {
                        let const_idx = code[i + 1];
                        wasm_code.push(0x41); // i32.const
                        wasm_code.push(const_idx);
                        i += 1;
                    }
                    
                    // Control flow
                    Opcode::RETURN => {
                        wasm_code.push(0x0F); // return
                    }
                    
                    _ => {
                        // Unsupported opcode - emit nop
                        wasm_code.push(0x01); // nop
                    }
                }
                i += 1;
            } else {
                return Err(WasmError::InvalidOpcode(code[i]));
            }
        }

        // End function
        wasm_code.push(0x0B); // end

        Ok(wasm_code)
    }

    /// Generate WASM binary format
    fn generate_wasm_binary(&self) -> Result<Vec<u8>, WasmError> {
        let mut binary = Vec::new();

        // Magic number and version
        binary.extend_from_slice(&self.module.magic);
        binary.extend_from_slice(&self.module.version);

        // Type section
        if !self.type_section.is_empty() {
            binary.push(0x01); // Type section ID
            let type_data = self.encode_type_section();
            self.encode_u32(&mut binary, type_data.len() as u32);
            binary.extend(type_data);
        }

        // Function section
        if !self.function_section.is_empty() {
            binary.push(0x03); // Function section ID
            let func_data = self.encode_function_section();
            self.encode_u32(&mut binary, func_data.len() as u32);
            binary.extend(func_data);
        }

        // Export section
        if !self.export_section.is_empty() {
            binary.push(0x07); // Export section ID
            let export_data = self.encode_export_section();
            self.encode_u32(&mut binary, export_data.len() as u32);
            binary.extend(export_data);
        }

        // Code section
        if !self.code_section.is_empty() {
            binary.push(0x0A); // Code section ID
            let code_data = self.encode_code_section();
            self.encode_u32(&mut binary, code_data.len() as u32);
            binary.extend(code_data);
        }

        Ok(binary)
    }

    fn encode_type_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.encode_u32(&mut data, self.type_section.len() as u32);
        
        for func_type in &self.type_section {
            data.push(0x60); // func type
            self.encode_u32(&mut data, func_type.params.len() as u32);
            for param in &func_type.params {
                data.push(self.encode_value_type(*param));
            }
            self.encode_u32(&mut data, func_type.results.len() as u32);
            for result in &func_type.results {
                data.push(self.encode_value_type(*result));
            }
        }
        
        data
    }

    fn encode_function_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.encode_u32(&mut data, self.function_section.len() as u32);
        
        for &type_idx in &self.function_section {
            self.encode_u32(&mut data, type_idx);
        }
        
        data
    }

    fn encode_export_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.encode_u32(&mut data, self.export_section.len() as u32);
        
        for export in &self.export_section {
            self.encode_string(&mut data, &export.name);
            data.push(match export.kind {
                ExportKind::Function => 0x00,
                ExportKind::Table => 0x01,
                ExportKind::Memory => 0x02,
                ExportKind::Global => 0x03,
            });
            self.encode_u32(&mut data, export.index);
        }
        
        data
    }

    fn encode_code_section(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.encode_u32(&mut data, self.code_section.len() as u32);
        
        for func_body in &self.code_section {
            let mut body_data = Vec::new();
            self.encode_u32(&mut body_data, func_body.locals.len() as u32);
            for (count, val_type) in &func_body.locals {
                self.encode_u32(&mut body_data, *count);
                body_data.push(self.encode_value_type(*val_type));
            }
            body_data.extend(&func_body.code);
            
            self.encode_u32(&mut data, body_data.len() as u32);
            data.extend(body_data);
        }
        
        data
    }

    fn encode_value_type(&self, val_type: ValueType) -> u8 {
        match val_type {
            ValueType::I32 => 0x7F,
            ValueType::I64 => 0x7E,
            ValueType::F32 => 0x7D,
            ValueType::F64 => 0x7C,
        }
    }

    fn encode_u32(&self, data: &mut Vec<u8>, value: u32) {
        // LEB128 encoding
        let mut val = value;
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            data.push(byte);
            if val == 0 {
                break;
            }
        }
    }

    fn encode_string(&self, data: &mut Vec<u8>, s: &str) {
        self.encode_u32(data, s.len() as u32);
        data.extend(s.as_bytes());
    }

    /// Save WASM binary to file
    pub fn save_to_file(&self, binary: &[u8], path: &str) -> Result<(), WasmError> {
        std::fs::write(path, binary)
            .map_err(|e| WasmError::IoError(e.to_string()))?;
        println!("[WASM] Saved to {}", path);
        Ok(())
    }
}

/// WASM compilation errors
#[derive(Debug)]
pub enum WasmError {
    InvalidOpcode(u8),
    UnsupportedFeature(String),
    IoError(String),
}

impl std::fmt::Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmError::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
            WasmError::UnsupportedFeature(feat) => write!(f, "Unsupported feature: {}", feat),
            WasmError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for WasmError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_compilation() {
        let mut compiler = WasmCompiler::new();
        let chunk = BytecodeChunk::new("test".to_string());
        
        let result = compiler.compile(&chunk);
        assert!(result.is_ok());
        
        let binary = result.unwrap();
        assert_eq!(&binary[0..4], &[0x00, 0x61, 0x73, 0x6D]); // Magic number
    }

    #[test]
    fn test_value_type_encoding() {
        let compiler = WasmCompiler::new();
        assert_eq!(compiler.encode_value_type(ValueType::I32), 0x7F);
        assert_eq!(compiler.encode_value_type(ValueType::I64), 0x7E);
    }
}
