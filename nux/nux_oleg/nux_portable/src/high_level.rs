use crate::lexer::{Lexer, Token, Span};
use std::vec::Vec;
use std::collections::HashMap;

use std::fmt;



#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub span: Span,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Line {}, Col {})", self.message, self.span.line, self.span.col)
    }
}

impl CompileError {
    fn new(msg: String, span: Span) -> Self {
        Self { message: msg, span }
    }
}

#[derive(Debug)]
pub struct CompileResult {
    pub output: Option<String>,
    pub errors: Vec<CompileError>,
}

pub fn compile_to_asm_source(source: &str) -> Result<String, Vec<CompileError>> {
    let mut parser = Parser::new(source);
    match parser.parse_to_asm() {
        Ok(out) => {
            if parser.errors.is_empty() {
                Ok(out)
            } else {
                Err(parser.errors)
            }
        },
        Err(e) => {
           // Critical failure (e.g. unhandled), but we also have accumulated errors.
           parser.errors.push(e);
           Err(parser.errors)
        }
    }
}

pub fn compile_high_level(source: &str) -> Result<Vec<u8>, Vec<CompileError>> {
    match compile_to_asm_source(source) {
        Ok(asm) => {
            crate::compiler::compile(&asm).map_err(|e| vec![CompileError::new(e, Span { line: 0, col: 0 })])
        },
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Char,
    String,
    Bool,
    Unknown,
    Class(String),
    Pointer(Box<Type>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    None,
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub label: String,
    pub arg_count: usize,
    pub access: AccessModifier,
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub fields: HashMap<String, u32>,
    pub size: u32,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    current_span: Span,
    prev_span: Span,
    
    // Compiler State
    var_addr_counter: u64,
    label_id_counter: usize,
    // Scope: Name -> (VarLocation, Type)
    scopes: Vec<HashMap<String, (VarLocation, Type)>>,
    
    // Output
    asm_output: String,
    errors: Vec<CompileError>,
    
    // Loop control stack: (start_label, end_label)
    loop_stack: Vec<(String, String)>, // (ContinueLabel, BreakLabel)
    
    // Function State
    local_offset: i64, 
    
    // Advanced Types
    // Name -> (Start, End) (Inclusive)
    bound_types: HashMap<String, (i64, i64)>,
    classes: HashMap<String, ClassInfo>,
    functions: HashMap<String, FunctionInfo>,
}

#[derive(Debug, Clone, Copy)]
enum VarLocation {
    Global(u64),
    Local(i64),
}

impl Parser {
    fn new(source: &str) -> Self {
        let mut l = Lexer::new(source);
        let (first, span) = l.next_token();
        
        // Initialize with Global Scope
        let mut scopes = Vec::new();
        scopes.push(HashMap::new());

        Self {
            lexer: l,
            label_id_counter: 0,
            scopes: scopes,
            var_addr_counter: 0, 
            current_token: first,
            current_span: span,
            prev_span: span,
            asm_output: String::new(),
            errors: Vec::new(),
            loop_stack: Vec::new(),
            local_offset: 0,
            bound_types: HashMap::new(),
            classes: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn advance(&mut self) {
        self.prev_span = self.current_span;
        let (tok, span) = self.lexer.next_token();
        self.current_token = tok;
        self.current_span = span;
    }

    // --- Scope Helpers ---
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: String, typ: Type) -> VarLocation {
        // Check depth
        if self.scopes.len() > 1 {
            // Local Variable
            let offset = self.local_offset;
            self.local_offset += 1;
            let loc = VarLocation::Local(offset);
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name, (loc, typ));
            }
            loc
        } else {
            // Global Variable (Static)
            let addr = self.var_addr_counter;
            self.var_addr_counter += 8; 
            let loc = VarLocation::Global(addr);
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name, (loc, typ));
            }
            loc
        }
    }

    fn resolve_var(&self, name: &str) -> Option<(VarLocation, Type)> {
        if name == "this" {
             // Resolving 'this' in scopes
        }
        for scope in self.scopes.iter().rev() {
            if let Some(&(loc, ref t)) = scope.get(name) {
                return Some((loc, t.clone()));
            }
        }
        None
    }

    fn emit(&mut self, s: &str) {
        self.asm_output.push_str(s);
        self.asm_output.push('\n');
    }

    // Recover from error by skipping up to semicolon or end of block
    fn synchronize(&mut self) {
        self.advance(); // Consumes the bad token to prevent infinite loop
        while self.current_token != Token::EOF {
             if self.current_token == Token::SemiColon {
                 self.advance();
                 return;
             }
             match self.current_token {
                 Token::Class | Token::Func | Token::Var | Token::For | 
                 Token::If | Token::While | Token::Print | Token::Return |
                 Token::Safe | Token::Verify => return, // Start of new statement
                 Token::RBrace => return, // End of block
                 _ => self.advance(),
             }
        }
    }
    
    // Non-fatal error logging
    fn error_at_current(&mut self, msg: String) {
        self.errors.push(CompileError::new(msg, self.current_span));
    }

    fn parse_to_asm(&mut self) -> Result<String, CompileError> {
        self.emit("; Auto-Generated by Nux High-Level Compiler");
        self.emit("JMP __start_execution"); 
        
        let mut main_body = String::new();
        let mut definitions = String::new();
        
        loop {
            // Token trace (disabled in release TUI)
            match &self.current_token {
                Token::EOF => break,
                Token::Pub => {
                    self.advance();
                    if self.current_token == Token::Func {
                         self.parse_func(&mut definitions, "", AccessModifier::Public)?;
                    } else if self.current_token == Token::Class {
                         self.parse_class(&mut definitions)?;
                    } else {
                         return self.error("Expected func or class after pub".to_string());
                    }
                },
                Token::Private => {
                    self.advance();
                     if self.current_token == Token::Func {
                         self.parse_func(&mut definitions, "", AccessModifier::Private)?;
                    } else {
                         return self.error("Expected func after private".to_string());
                    }
                },
                Token::Protected => {
                    self.advance();
                    let mut key = String::new();
                    if self.current_token == Token::LParen {
                        self.advance();
                        match &self.current_token {
                            Token::String(s) => key = s.clone(),
                            Token::Number(n) => key = n.to_string(),
                             _ => return self.error("Expected key".to_string()),
                        }
                        self.advance();
                        if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                        self.advance();
                    }
                    if self.current_token == Token::Func {
                         self.parse_func(&mut definitions, "", AccessModifier::Protected(key))?;
                    } else {
                         return self.error("Expected func after protected".to_string());
                    }
                },
                Token::Class => {
                     if let Err(e) = self.parse_class(&mut definitions) {
                          self.errors.push(e);
                          self.synchronize();
                     }
                },
                Token::Func => {
                     if let Err(e) = self.parse_func(&mut definitions, "", AccessModifier::Public) {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Identifier(_) | Token::Print | Token::Println | Token::Input |
                Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |
                Token::Var | Token::Return | Token::Lock | Token::Unlock | Token::Peek |
                Token::Safe | Token::Verify | Token::Free | Token::KwLimitMem |
                Token::Star | Token::KwInt | Token::KwFloat | Token::KwByte | Token::KwShort | Token::KwLong | Token::KwChar | Token::KwString => {
                    if let Err(e) = self.parse_statement_or_expr(&mut main_body) {
                        self.errors.push(e);
                        self.synchronize();
                    }
                },
                Token::Import => { // Preprocessor-like include
                    // import "filename";
                    self.advance();
                    let raw_name = match &self.current_token {
                        Token::String(s) => s.clone(),
                        _ => {
                            self.errors.push(CompileError::new("Expected filename string".to_string(), self.current_span));
                            continue;
                        }
                    };
                    self.advance();
                    // Resolution Logic
                    let mut filename = String::from("lib/");
                    let mut is_raw = false;
                    
                    if raw_name.ends_with(".nux") || raw_name.ends_with(".nuxel") {
                        filename = raw_name.clone();
                        is_raw = true;
                    } else {
                        filename.push_str(&raw_name);
                        filename.push_str(".nuxel"); // Prefer .nuxel for libraries
                    }
                    self.advance();
                    if self.current_token != Token::SemiColon {
                        self.errors.push(CompileError::new("Expected ;".to_string(), self.prev_span));
                        continue;
                    }
                    self.advance();
                    
                    // TODO: Actually read file and parse it.
                    // This is complex because we need to inject tokens or recurse parser.
                    // Simplest: `compile_high_level` should maybe resolve imports?
                    // But `Parser` owns the lexer.
                    // Option: Delegate to a new Parser instance and merge output?
                    // Yes. We can parse the imported file into `definitions` string.
                    // Resolve Import Path
                    let mut content_opt = std::fs::read_to_string(&filename).ok();
                    
                    if content_opt.is_none() && !is_raw {
                        let fallback = filename.replace(".nuxel", ".nux");
                        content_opt = std::fs::read_to_string(&fallback).ok();
                        if content_opt.is_some() {
                            filename = fallback;
                        }
                    }
                    
                    if content_opt.is_none() {
                        // Check virtual environment lib directory first
                        if let Some(venv_lib) = crate::project::get_venv_lib_path() {
                            let full_path = venv_lib.join(&filename);
                            content_opt = std::fs::read_to_string(&full_path).ok();
                        }
                    }

                    if content_opt.is_none() {
                        // Check NUX_LIB_PATH
                        if let Ok(path) = std::env::var("NUX_LIB_PATH") {
                             let full_path = format!("{}/{}", path, filename);
                             content_opt = std::fs::read_to_string(&full_path).ok();
                        }
                    }
                    
                    if content_opt.is_none() {
                        // Check Standard Paths
                        let paths = ["/usr/local/lib/nux", "/usr/lib/nux", "/opt/nux/lib"];
                        for path in &paths {
                             let full_path = format!("{}/{}", path, filename);
                             if let Ok(c) = std::fs::read_to_string(&full_path) {
                                  content_opt = Some(c);
                                  break;
                             }
                        }
                    }

                    if let Some(content) = content_opt {
                         let mut sub_parser = Parser::new(&content);
                         match sub_parser.parse_to_asm() {
                             Ok(asm) => {
                                 // Strip Header: "; Auto-Generated...\nJMP __start_execution\n"
                                 // Strip Footer: "__start_execution:...\nEXIT\n" (and optional main wrapper)
                                 
                                 // Heuristic: Find first label that is NOT __start_execution or main skip?
                                 // Actually, import is for definitions.
                                 // Definitions appear AFTER JMP __start_execution and BEFORE __main/__start.
                                 // If we just remove lines starting with ";", "JMP __start_execution", "EXIT", "__start_execution:", "CALL __main", "POP", "JMP skip___main", "__main:", "RET", "skip___main:"?
                                 // Too fragile.
                                 
                                 // Better: Extract lines that look like function definitions?
                                 // Or: Just assume standard Nux compiler output structure.
                                 // Structure:
                                 // [Header]
                                 // [Definitions]
                                 // [Implicit Main Wrapper (optional)]
                                 // [Footer]
                                 
                                 // We only want [Definitions].
                                 // We can find where [Definitions] end.
                                 // They end when [Implicit Main] starts involving "skip___main" OR when Footer starts "__start_execution".
                                 
                                 let lines: Vec<&str> = asm.lines().collect();
                                 let mut capture = false;
                                 for line in lines {
                                     if line.trim().starts_with("JMP __start_execution") {
                                         capture = true;
                                         continue;
                                     }
                                     if line.trim().starts_with("; Implicit main") || line.trim().starts_with("__start_execution:") {
                                         capture = false;
                                         continue; // Stop capturing
                                     }
                                     
                                     if capture {
                                         definitions.push_str(line);
                                         definitions.push('\n');
                                     }
                                 }
                             },
                             Err(e) => {
                                  self.errors.push(CompileError::new(format!("Failed to parse import {}: {}", filename, e.message), self.prev_span));
                             }
                         }
                    } else {
                        self.errors.push(CompileError::new(format!("File not found: {}", filename), self.prev_span));
                    }
                },
                Token::SemiColon => self.advance(), // Empty
                Token::LBrace => {
                     if let Err(e) = self.parse_block(&mut main_body) {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Poke => {
                    // poke(addr, val);
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected ( for poke".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; // Addr
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; // Val
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                    self.advance();
                    main_body.push_str("POKE\n");
                },
                _ => {
                    self.error_at_current(format!("Unexpected token at top level: {:?}", self.current_token));
                    self.advance();
                }
            }
        }

        
        // Append Function Definitions
        self.asm_output.push_str(&definitions);
        
        // Wrap top-level statements in __main function
        if !main_body.trim().is_empty() {
            self.emit("; Implicit main function for top-level statements");
            self.emit("JMP skip___main");
            self.emit("__main:");
            self.asm_output.push_str(&main_body);
            self.emit("PUSH 0");
            self.emit("RET");
            self.emit("skip___main:");
        }
        
        // Program entry point
        self.emit("__start_execution:");
        if !main_body.trim().is_empty() {
            self.emit("CALL __main 0");
            self.emit("POP ; Discard __main return value");
        }
        self.emit("EXIT");
        
        Ok(self.asm_output.clone())
    }

    // --- Parsing Routines ---

    fn error<T>(&self, msg: String) -> Result<T, CompileError> {
        Err(CompileError::new(msg, self.current_span))
    }

    fn parse_class(&mut self, out: &mut String) -> Result<(), CompileError> {
        // parse_class entry
        self.advance(); // consume 'class'
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected class name".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LBrace { return self.error("Expected '{' after class name".to_string()); }
        self.advance();
        
        let mut fields = HashMap::new();
        let mut offset = 0;
        
        // Inside class, we expect functions (methods).
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if self.current_token == Token::Func {
                self.parse_func(out, &name, AccessModifier::Public)?;
            } else if self.current_token == Token::Var {
                // Field Declaration: var x: type;
                self.advance();
                let field_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected field name".to_string())
                };
                self.advance();
                
                // Add to fields
                fields.insert(field_name, offset);
                offset += 1; // All fields are 8 bytes (1 slot)
                
                // Optional initialization or type?
                // Expect : Type
                if self.current_token == Token::Colon {
                    self.advance();
                    // Consume type
                    // Identifier or KwType
                    self.advance(); 
                }
                
                // For now expect ;
                if self.current_token == Token::SemiColon { self.advance(); }
            } else {
                return self.error("Only functions/fields allowed in classes for now".to_string());
            }
        }
        
        // Register Class
        self.classes.insert(name, ClassInfo { fields, size: offset });
        
        if self.current_token != Token::RBrace { return self.error("Expected '}'".to_string()); }
        self.advance();
        Ok(())
    }

    // --- Advanced Types ---
    fn parse_bound_type_decl(&mut self) -> Result<(), CompileError> {
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected type name after 'func var'".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); }
        self.advance();
        
        let mut start_val = i64::MIN;
        let mut end_val = i64::MAX;
        
        // Parse Definitions: start = x; end = y;
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            let key = match &self.current_token {
                Token::Identifier(s) => s.clone(),
                _ => return self.error("Expected 'start' or 'end'".to_string()),
            };
            self.advance();
            
            if self.current_token != Token::Eq { return self.error("Expected =".to_string()); }
            self.advance();
            
            // For now, we only support constant expressions for bounds!
            let val = match &self.current_token {
                Token::Number(n) => *n,
                Token::Minus => {
                    self.advance();
                    match &self.current_token {
                         Token::Number(n) => -n,
                         _ => return self.error("Expected number after -".to_string()),
                    }
                },
                _ => return self.error("Expected constant number/char for bound".to_string()),
            };
            self.advance();
            
            if self.current_token == Token::SemiColon { self.advance(); }
            
            if key == "start" { start_val = val; }
            else if key == "end" { end_val = val; }
            else { return self.error(format!("Unknown property '{}'", key)); }
        }
        
        if self.current_token != Token::RBrace { return self.error("Expected }".to_string()); }
        self.advance();
        
        self.bound_types.insert(name, (start_val, end_val));
        Ok(())
    }

    fn parse_func(&mut self, out: &mut String, class_prefix: &str, access: AccessModifier) -> Result<(), CompileError> {
        // parse_func entry
        // Optional Pub (Legacy check? Should be handled by caller now, but keeping safe)
        if self.current_token == Token::Pub {
            self.advance();
        }

        self.advance(); // consume 'fn' (was func)
        
        if self.current_token == Token::Var {
             // Support: fn let MyType ... (legacy func var pattern? remove?)
             self.advance(); 
             return self.parse_bound_type_decl();
        }

        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected function name".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LParen { return self.error("Expected '('".to_string()); }
        self.advance();
        
        // Parse Arguments
        let mut args = Vec::new();
        if self.current_token != Token::RParen {
            loop {
                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                args.push(arg_name);
                
                if self.current_token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
        self.advance();
        
        // Optional Return Type: -> Type
        if self.current_token == Token::Arrow {
            self.advance(); // skip ->
            // Parse type (identifier or keyword)
            match self.current_token {
                Token::Identifier(_) | Token::KwInt | Token::KwFloat | Token::KwByte | Token::KwShort | Token::KwLong | Token::KwChar | Token::KwString => {
                    self.advance(); // consume type
                },
                _ => return self.error("Expected return type".to_string()),
            }
        }

// Placeholder to force view_file or find_by_name if I was using grep
// I will just use grep_search to find the line number of parse_class definition.
        
        // Generate Label
        let full_name = if class_prefix.is_empty() {
             name.clone()
        } else {
             format!("{}_{}", class_prefix, name)
        };
        
        out.push_str(&format!("; Function {}\n", full_name));
        out.push_str(&format!("JMP skip_{}\n", full_name)); // Skip definition
        out.push_str(&format!("{}:\n", full_name));
        
        // Arguments Handling:
        // VM sets FP = stack.len() - num_args
        // Stack: [Arg0, Arg1, ... ArgN-1] where FP points to Arg0
        // Arg0 is at offset 0 (fp + 0)
        // Arg1 is at offset 1 (fp + 1)
        // ArgN-1 is at offset N-1
        // Locals start at offset N
        
        self.enter_scope(); 
        
        let mut arg_start = 0;
        // Check if inside class method
        // Heuristic: class_prefix is not empty for class methods?
        // Actually, parse_class passes `&name` as prefix.
        if !class_prefix.is_empty() {
             // Inject 'this' at offset 0
             let loc = VarLocation::Local(0);
             if let Some(scope) = self.scopes.last_mut() {
                 scope.insert("this".to_string(), (loc, Type::Class(class_prefix.to_string())));
                 // 'this' injected into scope for class
             }
             arg_start = 1;
        }
        
        let num_args = args.len() as i64;
        self.local_offset = num_args + (arg_start as i64); // Locals start after arguments
        
        for (i, arg) in args.iter().enumerate() {
             // Arguments are at positive offsets 0, 1, 2, ...
             // If class method, they shift by 1.
             let offset = (i + arg_start) as i64;
             let loc = VarLocation::Local(offset);
             if let Some(scope) = self.scopes.last_mut() {
                 scope.insert(arg.clone(), (loc, Type::Int));
             }
        }
        
        // Parse Body
        let mut body_asm = String::new();
        self.parse_block(&mut body_asm)?;
        
        self.exit_scope(); // End Function Scope
        
        out.push_str(&body_asm);
        out.push_str("PUSH 0\n"); // Default return value
        out.push_str("RET\n");
        out.push_str(&format!("skip_{}:\n", full_name));
        
        Ok(())
    }
    
    fn parse_block(&mut self, out: &mut String) -> Result<(), CompileError> {
        if self.current_token != Token::LBrace { return self.error("Expected block '{'".to_string()); }
        self.advance();
        
        self.enter_scope();
        
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
             if let Err(e) = self.parse_statement_or_expr(out) {
                 self.errors.push(e);
                 self.synchronize();
             }
        }
        
        self.exit_scope();
        
        if self.current_token != Token::RBrace { return self.error("Expected '}'".to_string()); }
        self.advance();
        Ok(())
    }

    fn parse_statement_or_expr(&mut self, out: &mut String) -> Result<(), CompileError> {
        self.parse_statement_impl(out, true)
    }

    fn parse_statement_impl(&mut self, out: &mut String, expect_semi: bool) -> Result<(), CompileError> {
        match &self.current_token {
             Token::At => {
                 self.advance();
                 if self.current_token != Token::Hardware { return self.error("Expected 'hardware' after '@'".to_string()); }
                 self.advance(); // Skip hardware
                 if self.current_token != Token::LParen { return self.error("Expected '('".to_string()); }
                 self.advance();
                 if let Token::String(name) = &self.current_token {
                     // We just ignore the name for now, or print it
                     println!("Target Hardware: {}", name);
                     self.advance();
                 } else { return self.error("Expected hardware name string".to_string()); }
                 if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
                 self.advance();
                 // No semicolon required for @hardware() but let's allow it
                 if expect_semi && self.current_token == Token::SemiColon {
                     self.advance();
                 }
                 return Ok(());
             },
             Token::Link => {
                 self.advance();
                 if let Token::String(filename) = self.current_token.clone() {
                     self.advance();
                     if expect_semi {
                         if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                         self.advance();
                     }
                     
                     // Use the exact same import logic
                     let mut content_opt = std::fs::read_to_string(&filename).ok();
                     if content_opt.is_none() {
                         let fallback = filename.replace(".nuxel", ".nux");
                         content_opt = std::fs::read_to_string(&fallback).ok();
                     }
                     if content_opt.is_none() {
                         if let Some(venv_lib) = crate::project::get_venv_lib_path() {
                             content_opt = std::fs::read_to_string(&venv_lib.join(&filename)).ok();
                         }
                     }
                     if content_opt.is_none() {
                         if let Ok(path) = std::env::var("NUX_LIB_PATH") {
                             content_opt = std::fs::read_to_string(&format!("{}/{}", path, filename)).ok();
                         }
                     }
                     if let Some(c) = content_opt {
                         let mut sub_parser = Parser::new(&c);
                         if let Ok(asm) = sub_parser.parse_to_asm() {
                             // Extract only the definitions, skipping headers/footers
                             let lines: Vec<&str> = asm.lines().collect();
                             let mut capture = false;
                             let mut definitions = String::new();
                             for line in lines {
                                 if line.trim().starts_with("JMP __start_execution") {
                                     capture = true;
                                     continue;
                                 }
                                 if line.trim().starts_with("; Implicit main") || line.trim().starts_with("__start_execution:") {
                                     capture = false;
                                     continue;
                                 }
                                 if capture {
                                     definitions.push_str(line);
                                     definitions.push('\n');
                                 }
                             }
                             out.push_str(&definitions);
                             
                             // Also extend classes and functions
                             for (k, v) in sub_parser.classes {
                                 self.classes.insert(k, v);
                             }
                             for (k, v) in sub_parser.functions {
                                 self.functions.insert(k, v);
                             }
                         }
                     } else {
                         return self.error(format!("Could not resolve link file: {}", filename));
                     }
                 } else { return self.error("Expected link path string".to_string()); }
                 return Ok(());
             },
             Token::Register => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected '('".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new();
                 let (_val_type, val) = self.parse_expression(&mut sub_out)?;
                 
                 if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
                 self.advance();
                 
                 if self.current_token != Token::As { return self.error("Expected 'as'".to_string()); }
                 self.advance();
                 
                 if let Token::Identifier(name) = self.current_token.clone() {
                     self.advance();
                     
                     let addr = self.var_addr_counter;
                     self.var_addr_counter += 8;
                     let loc = VarLocation::Global(addr);
                     if let Some(scope) = self.scopes.first_mut() {
                         scope.insert(name.clone(), (loc, Type::Int));
                     }
                     
                     if let Some(val) = val {
                         match val {
                             ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                             _ => out.push_str("PUSH 0\n"),
                         }
                     } else {
                         out.push_str(&sub_out);
                     }
                     out.push_str(&format!("STORE_GLOBAL {}\n", addr));
                     
                     if expect_semi {
                         if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                         self.advance();
                     }
                 } else { return self.error("Expected identifier after 'as'".to_string()); }
                 return Ok(());
             },
             Token::Asm => {
                 self.advance();
                 if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); }
                 self.advance();
                 while self.current_token != Token::RBrace && self.current_token != Token::EOF {
                     if let Token::String(s) = &self.current_token {
                         out.push_str(s); out.push('\n'); self.advance();
                     } else if let Token::Identifier(name) = &self.current_token {
                         // Resolve Variable
                         if let Some((loc, _)) = self.resolve_var(name) {
                             match loc {
                                 VarLocation::Local(idx) => out.push_str(&format!("GET_LOCAL {}\n", idx)),
                                 VarLocation::Global(addr) => out.push_str(&format!("GET_GLOBAL {}\n", addr)),
                             }
                         } else {
                              // If not resolved, assume opcode/label
                              out.push_str(name); out.push('\n');
                         }
                         self.advance();
                     } else if let Token::Number(n) = &self.current_token {
                         out.push_str(&format!("{}\n", n)); 
                         self.advance();
                     } else if self.current_token == Token::Comma || self.current_token == Token::SemiColon {
                         self.advance();
                     } else {
                         return self.error("Invalid token in asm".to_string());
                     }
                 }
                 if self.current_token != Token::RBrace { return self.error("Expected }".to_string()); }
                 self.advance();
             },
             Token::Pub => {
                 self.advance();
                 if let Token::Identifier(name) = &self.current_token {
                     let n = name.clone();
                     self.advance();
                     self.parse_call_args(out, &n)?;
                 } else { return self.error("Expected function name after pub".to_string()); }
             },
             Token::Pri => {
                 self.advance();
                 if let Token::Identifier(name) = &self.current_token {
                     let n = name.clone();
                     self.advance();
                     // Check Access
                     if let Some(info) = self.functions.get(&n) {
                         // if info.access != AccessModifier::Private ... warn?
                     }
                     self.parse_call_args(out, &n)?;
                 } else { return self.error("Expected function name after pri".to_string()); }
             },
             Token::Pro => {
                 self.advance();
                 if self.current_token == Token::LParen {
                     self.advance();
                     let key = match &self.current_token {
                         Token::String(s) => s.clone(),
                         _ => return self.error("Expected key string".to_string()),
                     };
                     self.advance();
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     
                     if let Token::Identifier(name) = &self.current_token {
                         let n = name.clone();
                         self.advance();
                         self.parse_call_args(out, &n)?;
                     } else { return self.error("Expected function name".to_string()); }
                 } else {
                     return self.error("Expected ('key') after pro".to_string());
                 }
             },
             Token::Print => {
                 self.advance();
                 self.parse_print(out, false)?;
             },
             Token::Println => {
                 self.advance();
                 self.parse_print(out, true)?;
             },
             Token::Identifier(name) => {
                 let part1 = name.clone();
                 
                 // Intrinsic: sleep
                 if part1 == "sleep" {
                     self.advance(); // skip name
                     if self.current_token != Token::LParen { return self.error("Expected ( for sleep".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi {
                         if self.current_token == Token::SemiColon { self.advance(); }
                     } else if self.current_token == Token::SemiColon { self.advance(); }
                     
                     out.push_str("OP_SLEEP\n");
                     return Ok(());
                 }
                 

                 self.advance(); // skip name
                 if self.current_token == Token::Eq {
                       // Assignment
                        match self.resolve_var(&part1) {
                            Some((loc, _typ)) => {
                                self.advance(); // Skip =
                                let mut sub_out = String::new();
                                let (_, constant) = self.parse_expression(&mut sub_out)?;
                                
                                if let Some(val) = constant {
                                    match val {
                                        ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                        ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                                        ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                        _ => {}
                                    }
                                } else {
                                    out.push_str(&sub_out);
                                }

                                if expect_semi {
                                    if self.current_token != Token::SemiColon {
                                        return self.error("Expected ;".to_string());
                                    }
                                    self.advance();
                                } else if self.current_token == Token::SemiColon {
                                    self.advance(); // optionally consume
                                }
                                match loc {
                                    VarLocation::Global(addr) => {
                                        out.push_str(&format!("PUSH {}\nPOKE\n", addr)); 
                                    },
                                    VarLocation::Local(offset) => {
                                        out.push_str(&format!("SET_LOCAL {}\n", offset));
                                    }
                                }
                            },
                           None => {
                               // Auto-declare undefined variables in global scope (Python-like)
                               if self.scopes.len() == 1 {
                                   // Global scope - auto-declare
                                   self.advance(); // Skip =
                                   
                                   let mut sub_out = String::new();
                                   let (expr_type, constant) = self.parse_expression(&mut sub_out)?;
                                   
                                   if let Some(val) = constant {
                                        match val {
                                            ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                            ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                                            ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                            _ => {}
                                        }
                                   } else {
                                        out.push_str(&sub_out);
                                   }

                                   if expect_semi {
                                       if self.current_token != Token::SemiColon {
                                           return self.error("Expected ;".to_string());
                                       }
                                       self.advance();
                                   } else if self.current_token == Token::SemiColon {
                                       self.advance();
                                   }
                                   
                                   // Declare as global with inferred type
                                   let loc = self.declare_var(part1.clone(), Type::Int);
                                   match loc {
                                       VarLocation::Global(addr) => {
                                           out.push_str(&format!("PUSH {}\nPOKE\n", addr));
                                       },
                                       VarLocation::Local(_) => {
                                           // Should never happen in global scope
                                           return self.error("Internal error: local var in global scope".to_string());
                                       }
                                   }
                               } else {
                                   // Function scope - require explicit declaration
                                   return self.error(format!("Undefined variable '{}' (use 'var {}' to declare)", part1, part1));
                               }
                           }
                       }
                 } else if self.current_token == Token::LParen {
                      self.advance(); // Skip (
                      // Args
                      let mut arg_count = 0;
                      if self.current_token != Token::RParen {
                           loop {
                               let mut sub_out = String::new();
                               let (_, constant) = self.parse_expression(&mut sub_out)?;
                               if let Some(val) = constant {
                                   match val {
                                       ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                       ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                                       ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                       _ => {}
                                   }
                               } else {
                                   out.push_str(&sub_out);
                               }
                               arg_count += 1;
                               if self.current_token == Token::Comma { self.advance(); } else { break; }
                           }
                      }
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      if expect_semi {
                           if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                           self.advance();
                      } else if self.current_token == Token::SemiColon { self.advance(); }
                      
                      // Emit CALL (vm handles arg cleanup), then POP return value
                      // Checking intrinsic for identifier
                      if let Some(opcode) = self.get_intrinsic(&part1) {
                          // Found intrinsic opcode
                          out.push_str(&format!("{}\n", opcode));
                      } else {
                          // No intrinsic matched
                          out.push_str(&format!("CALL {} {}\n", part1, arg_count));
                      }
                      out.push_str("POP\n");
                      
                 } else if self.current_token == Token::Dot {
                      self.advance(); // Skip .
                      let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                      self.advance();
                      
                      if self.current_token == Token::Eq {
                          // Field Assignment: obj.field = expr;
                          let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                          
                          let offset = if let Type::Class(cname) = typ {
                              if let Some(cinfo) = self.classes.get(&cname) {
                                  if let Some(off) = cinfo.fields.get(&member) {
                                      *off
                                  } else { return self.error(format!("Class '{}' has no field '{}'", cname, member)); }
                              } else { return self.error(format!("Unknown class '{}'", cname)); }
                          } else {
                              // Fallback: Search all classes
                             let mut found = None;
                             for (cname, cinfo) in &self.classes {
                                 if let Some(off) = cinfo.fields.get(&member) {
                                     if found.is_some() { return self.error(format!("Ambiguous field '{}' (found in multiple classes)", member)); }
                                     found = Some(*off);
                                 }
                             }
                             if let Some(off) = found {
                                 off
                             } else {
                                 return self.error(format!("Variable '{}' is not a class instance and field '{}' not found globally", part1, member));
                             }
                          };
                          
                          // Push Object Addr
                          match loc {
                              VarLocation::Global(addr) => {
                                  // Global var holds the POINTER
                                  out.push_str(&format!("PUSH {}\n", addr));
                                  out.push_str("PEEK\n"); 
                              },
                              VarLocation::Local(idx) => {
                                  out.push_str(&format!("OP_GET_LOCAL {}\n\n", idx));
                              }
                          }
                          
                          // Field Addr
                          out.push_str(&format!("PUSH {}\nOP_ADD\n", offset));
                          
                          self.advance(); // Skip =
                          let mut sub_out = String::new();
                          let (_, constant) = self.parse_expression(&mut sub_out)?;
                          if let Some(val) = constant {
                               match val {
                                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                                    ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                    _ => {}
                               }
                          } else {
                               out.push_str(&sub_out);
                          }
                          
                          if expect_semi {
                              if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                              self.advance();
                          } else if self.current_token == Token::SemiColon { self.advance(); }
                          
                          out.push_str("POKE\n");
                          
                      } else if self.current_token == Token::LParen {
                          // Method Call
                          // Resolve "this"/object again
                          let (loc, _) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                           
                          // Push Object Instance (implicitly passed as first arg)
                          match loc {
                              VarLocation::Global(addr) => {
                                  out.push_str(&format!("PUSH {}\n", addr));
                                  out.push_str("PEEK\n"); 
                              },
                              VarLocation::Local(idx) => {
                                  out.push_str(&format!("OP_GET_LOCAL {}\n", idx));
                              }
                          }

                          self.advance();
                          let mut arg_count = 1; // 'this' counts as 1 argument
                          if self.current_token != Token::RParen {
                               loop {
                                   let mut sub_out = String::new();
                                   let (_, constant) = self.parse_expression(&mut sub_out)?;
                                   if let Some(val) = constant {
                                       match val {
                                           ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                           ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                                           ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                           _ => {}
                                       }
                                   } else {
                                       out.push_str(&sub_out);
                                   }
                                   arg_count += 1;
                                   if self.current_token == Token::Comma { self.advance(); } else { break; }
                               }
                          }
                          if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                          self.advance();
                          
                          if expect_semi {
                               if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                               self.advance();
                          } else if self.current_token == Token::SemiColon { self.advance(); }
                          
                          out.push_str(&format!("CALL {}_{} {}\nPOP\n", part1, member, arg_count));
                      } else {
                           return self.error("Expected = or ( after member name".to_string());
                      }
                 } else {
                       return self.error(format!("Unexpected token in statement (ID match): {:?} name={}", self.current_token, part1));
                 }
             },
              Token::Input => {
                self.advance();
                if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                self.advance();
                if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                self.advance();
                if expect_semi {
                    if self.current_token != Token::SemiColon { 
                        self.errors.push(CompileError::new("Expected ;".to_string(), self.prev_span));
                        self.synchronize();
                        return Ok(());
                    }
                    self.advance();
                } else if self.current_token == Token::SemiColon { self.advance(); }
                out.push_str("INPUT\n");
             },
             Token::Free => {
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected ( for free".to_string()); }
                  self.advance();
                  self.parse_expression(out)?;
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  if expect_semi {
                      if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                      self.advance();
                  } else if self.current_token == Token::SemiColon { self.advance(); }
                  out.push_str("OP_FREE\n");
             },
              Token::Var => {
                  self.parse_var_decl(out, Type::Unknown)?;
              },
              Token::KwInt => { self.parse_var_decl(out, Type::Int)?; },
              Token::KwFloat => { self.parse_var_decl(out, Type::Float)?; },
              Token::KwByte => { self.parse_var_decl(out, Type::Byte)?; },
              Token::KwShort => { self.parse_var_decl(out, Type::Short)?; },
              Token::KwLong => { self.parse_var_decl(out, Type::Long)?; },
              Token::KwChar => { self.parse_var_decl(out, Type::Char)?; },
              Token::KwString => { self.parse_var_decl(out, Type::String)?; },
              Token::Star => {
                  self.advance();
                  let base_type = match self.current_token {
                      Token::KwInt => Type::Int,
                      Token::KwFloat => Type::Float,
                      Token::KwByte => Type::Byte,
                      Token::KwShort => Type::Short,
                      Token::KwLong => Type::Long,
                      _ => return self.error("Expected primitive type after '*' for pointer declaration".to_string()),
                  };
                  // parse_var_decl will consume the current_token (which is the type keyword)
                  self.parse_var_decl(out, Type::Pointer(Box::new(base_type)))?;
              },
             Token::Return => {
                 self.advance();
                 if self.current_token == Token::SemiColon {
                     out.push_str("PUSH 0\nRET\n");
                     self.advance();
                 } else {
                     let mut sub_out = String::new();
                     let (_, constant) = self.parse_expression(&mut sub_out)?;
                     if let Some(val) = constant {
                        match val {
                            ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                            ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                            ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                             _ => {}
                        }
                     } else {
                         out.push_str(&sub_out);
                     }
                     
                     if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                     out.push_str("RET\n");
                     self.advance();
                 }
             },
             Token::If => {
                  self.advance(); // skip if
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  
                  let mut sub_out = String::new();
                  let (_, constant) = self.parse_expression(&mut sub_out)?;
                  
                  if let Some(val) = constant {
                       // Constant Folding for IF
                       // If True: Emit block, skip else.
                       // If False: Skip block, emit else if present.
                       match val {
                           ConstantValue::Bool(b) => {
                               if b {
                                   if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                                   self.advance();
                                   self.parse_block(out)?;
                                   // Check if else exists and consume it but don't emit
                                   if self.current_token == Token::Else {
                                       self.advance();
                                       // consume else block without emitting
                                       let mut junk = String::new();
                                       if self.current_token == Token::If {
                                            self.parse_statement_or_expr(&mut junk)?;
                                       } else {
                                            self.parse_block(&mut junk)?;
                                       }
                                   }
                                   return Ok(());
                               } else {
                                   // False
                                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                                   self.advance();
                                   // consume if block
                                   let mut junk = String::new();
                                   self.parse_block(&mut junk)?;
                                   
                                   if self.current_token == Token::Else {
                                       self.advance();
                                       if self.current_token == Token::If {
                                            self.parse_statement_or_expr(out)?;
                                       } else {
                                            self.parse_block(out)?;
                                       }
                                   }
                                   return Ok(());
                               }
                           },
                           _ => {} // Non-bool constant in if? Treat as generic?
                       }
                        // Fallthrough if not bool or we want to emit PUSH for it (e.g. Int used as bool)
                         match val {
                            ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                            ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)), // Float as bool?
                            ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                             _ => {}
                        }
                  } else {
                      out.push_str(&sub_out);
                  }

                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  
                  let label_id = self.label_id_counter;
                  self.label_id_counter += 1;
                  let label_else = format!("__if_else_{}", label_id);
                  let label_end = format!("__if_end_{}", label_id);
                  
                  out.push_str("PUSH 0\n");
                  out.push_str(&format!("JE {}\n", label_else));
                  
                  self.parse_block(out)?;
                  out.push_str(&format!("JMP {}\n", label_end));
                  
                  out.push_str(&format!("{}:\n", label_else));
                  
                  if self.current_token == Token::Else {
                      self.advance();
                      if self.current_token == Token::If {
                          self.parse_statement_or_expr(out)?; 
                      } else {
                          self.parse_block(out)?;
                      }
                  }
                  out.push_str(&format!("{}:\n", label_end));
             },
             Token::While => {
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  
                  let label_id = self.label_id_counter;
                  self.label_id_counter += 1;
                  let label_start = format!("__while_start_{}", label_id);
                  let label_end = format!("__while_end_{}", label_id);
                  self.loop_stack.push((label_start.clone(), label_end.clone())); 
                  
                  out.push_str(&format!("{}:\n", label_start));
                  
                  let mut sub_out = String::new();
                  let (_, constant) = self.parse_expression(&mut sub_out)?;
                  // While (false) -> Dead code?
                  // While (true) -> Infinite loop (unless break)
                  
                  if let Some(val) = constant {
                       match val {
                           ConstantValue::Bool(b) => {
                               if !b {
                                   // while(false)
                                   // consume body?
                                   // But we already emitted label_start.
                                   // We can emit JMP label_end?
                                   // Better: Just emit PUSH 0; JE ...
                                    out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 }));
                               } else {
                                   // while(true)
                                    out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 }));
                               }
                           },
                           _ => {
                                match val {
                                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                    _ => {}
                                }
                           }
                       }
                  } else {
                      out.push_str(&sub_out);
                  }
                  
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  
                  out.push_str("PUSH 0\n");
                  out.push_str(&format!("JE {}\n", label_end));
                  
                  self.parse_block(out)?;
                  out.push_str(&format!("JMP {}\n", label_start));
                  out.push_str(&format!("{}:\n", label_end));
                  self.loop_stack.pop();
             },
             Token::For => {
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  
                  // Init
                  self.parse_statement_or_expr(out)?; 
                  // parse_statement logic consumes semicolon inside "var" or "assign" or "expr-stmt" logic?
                  // Wait, parse_statement_or_expr checks for semicolon in ExpressionStmt, Var, Assign, Input, Return.
                  // But Token::If, While don't consume trailing semicolon.
                  // Check current token. If we are in 'For', we expect 'statement' then maybe we advanced passed semicolon?
                  // parse_statement_or_expr for "var i=0;" consumes ';'.
                  
                  let label_id = self.label_id_counter;
                  self.label_id_counter += 1;
                  let label_start = format!("__for_start_{}", label_id);
                  let label_step = format!("__for_step_{}", label_id);
                  let label_end = format!("__for_end_{}", label_id);
                  self.loop_stack.push((label_step.clone(), label_end.clone())); // Continue goes to Step
                  
                  out.push_str(&format!("{}:\n", label_start));
                  
                  // Cond
                  if self.current_token != Token::SemiColon {
                       let mut sub_out = String::new();
                       let (_, constant) = self.parse_expression(&mut sub_out)?;
                       if let Some(val) = constant {
                           match val {
                                ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                                ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                                _ => {}
                           }
                       } else {
                           out.push_str(&sub_out);
                       }
                       out.push_str("PUSH 0\n");
                       out.push_str(&format!("JE {}\n", label_end));
                  }
                  if self.current_token != Token::SemiColon { return self.error("Expected ; in for".to_string()); }
                  self.advance();
                  
                  out.push_str(&format!("JMP __for_body_{}\n", label_id));

                  // Step
                  out.push_str(&format!("{}:\n", label_step));
                  let mut step_out = String::new();
                  if self.current_token != Token::RParen {
                       // We need to parse expression but NOT consume semicolon? Step often doesn't have semicolon.
                       // Usually "i = i + 1". Assign consumes semicolon. 
                       // Standard for syntax: for(init; cond; step).
                       // Step is expression or assignment.
                       // My parse_expr/stmt consumes semicolon. 
                       // This is tricky if step is "i=i+1". 
                       // Let's tolerate ; if present.
                       self.parse_statement_impl(&mut step_out, false)?;
                  }
                  out.push_str(&step_out);
                  out.push_str(&format!("JMP {}\n", label_start));
                  
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  
                  out.push_str(&format!("__for_body_{}:\n", label_id));
                  self.parse_block(out)?;
                  
                  out.push_str(&format!("JMP {}\n", label_step));
                  out.push_str(&format!("{}:\n", label_end));
                  self.loop_stack.pop();
             },
             Token::Do => {
                 self.advance();
                 
                 let label_id = self.label_id_counter;
                 self.label_id_counter += 1;
                 let label_start = format!("__do_start_{}", label_id);
                 let label_end = format!("__do_end_{}", label_id);
                 // Continue in do-while usually goes to check condition.
                 // Let's say continues restart the loop body? No, standard is check condition.
                 // So we need a label for condition check?
                 let label_cond = format!("__do_cond_{}", label_id);
                 
                 self.loop_stack.push((label_cond.clone(), label_end.clone())); 
                 
                 out.push_str(&format!("{}:\n", label_start));
                 
                 self.parse_block(out)?;
                 
                 out.push_str(&format!("{}:\n", label_cond));
                 if self.current_token != Token::While { return self.error("Expected while after do block".to_string()); }
                 self.advance();
                 
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                 self.advance();
                 
                 // If true, jump back to start
                 // Cond returns 1 or 0.
                 // VM JE pops b, a. a==b -> jump.
                 // We push 1. Stack: [cond_res, 1]. JE compares 1 == cond_res.
                 out.push_str("PUSH 1\n");
                 out.push_str(&format!("JE {}\n", label_start));
                 
                 out.push_str(&format!("{}:\n", label_end));
                 self.loop_stack.pop();
             },
             Token::LBrace => {
                 self.parse_block(out)?;
             },
             Token::Peek => {
                 self.parse_expression(out)?; 
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Poke => {
                 self.parse_poke(out)?;
             },
             Token::Safe => {
                 self.advance();
                 if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); }
                 out.push_str("; BEGIN SAFE BLOCK\n");
                 self.parse_block(out)?;
                 out.push_str("; END SAFE BLOCK\n");
             },
             Token::Verify => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; 
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 if expect_semi {
                     if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                     self.advance();
                 } else if self.current_token == Token::SemiColon { self.advance(); }
                 
                 // Use OP_VERIFY or a conditional panic
                 out.push_str("OP_VERIFY\n");
             },
             Token::KwLimitMem => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; 
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 if expect_semi {
                     if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                     self.advance();
                 } else if self.current_token == Token::SemiColon { self.advance(); }
                 out.push_str("OP_LIMIT_MEM\n");
             },
             Token::Break => {
                 if let Some(label) = self.loop_stack.last() {
                     out.push_str(&format!("JMP {}\n", label.1));
                 } else {
                     return self.error("Break outside of loop".to_string());
                 }
                 self.advance();
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Continue => {
                 if let Some(label) = self.loop_stack.last() {
                     out.push_str(&format!("JMP {}\n", label.0));
                 } else {
                     return self.error("Continue outside of loop".to_string());
                 }
                 self.advance();
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgAlloc | Token::ImgFree | Token::ImgDraw | Token::CamCapture | 
             Token::ImgFilter | Token::ImgGet | Token::ImgSet | Token::ImgFill | 
             Token::ImgResize | Token::ImgCrop | Token::ImgGrayscale => {
                 self.parse_expression(out)?;
                 // Expression leaves result on stack, discard it for statement
                 out.push_str("POP\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Asm => {
                 self.advance();
                 if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); }
                 self.advance();
                 while self.current_token != Token::RBrace && self.current_token != Token::EOF {
                     match &self.current_token {
                         Token::String(s) => {
                             out.push_str(s);
                             out.push('\n');
                             self.advance();
                         },
                         Token::Comma | Token::SemiColon => self.advance(),
                         _ => return self.error("Expected string literals in asm block".to_string()),
                     }
                 }
                 self.advance();
             },
             Token::Spawn => {
                  self.advance();
                  match &self.current_token {
                      Token::Identifier(func_name) => {
                          out.push_str(&format!("PUSH {}\nSPAWN\n", func_name));
                      },
                      _ => return self.error("Expected function name".to_string()),
                  }
                  self.advance();
                  if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Lock | Token::Unlock => {
                 self.advance();
                 if self.current_token == Token::LParen { self.advance(); } 
                 if let Token::Identifier(_) = self.current_token { self.advance(); } 
                 if self.current_token == Token::RParen { self.advance(); }
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             
             // --- VISION STATEMENTS ---
             // --- VISION STATEMENTS ---
             Token::CamCapture => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected ( for cam_capture".to_string()); }
                 self.advance();
                 
                  let mut sub_out = String::new();
                  let (_, constant) = self.parse_expression(&mut sub_out)?; 
                  if let Some(val) = constant { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_CAPTURE\n");
                 out.push_str("POP\n"); // VM pushes 0, we must pop it.
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgDraw => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected ( for img_draw".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_DRAW\n");
                 // out.push_str("POP\n"); // VM OP_IMG_DRAW is void, don't pop.
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFree => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected ( for img_free".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_FREE\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFilter => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected ( for img_filter".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_FILTER\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgSet => {
                 self.advance();
                 // img_set(handle, x, y, color)
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_SET\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFill => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_FILL\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgGet => {
                  // If used as statement: img_get(h,x,y); -> pop result
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                 
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                  self.advance(); // ,
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }
                  self.advance(); // ,
                  let mut sub_out = String::new(); let (_, c) = self.parse_expression(&mut sub_out)?; if let Some(val) = c { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

                  self.advance(); // )
                  out.push_str("OP_IMG_GET\nPOP\n"); // Discard result
                  if self.current_token == Token::SemiColon { self.advance(); }
             },

             _ => {
                  return self.error(format!("Unexpected statement token: {:?}", self.current_token));
             }
        }
        Ok(())
    }



    // --- Var Declaration ---
    
    fn parse_var_decl(&mut self, out: &mut String, expected_type: Type) -> Result<(), CompileError> {
        self.advance(); // consume keyword (var, int, etc)
        
        // Name
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected variable name".to_string()),
        };
        self.advance();
        
        // Optional Type Constraint
        let mut constraint = None;
        if self.current_token == Token::Colon {
             self.advance(); // skip :
             match &self.current_token {
                 Token::Identifier(s) => {
                     if let Some(bounds) = self.bound_types.get(s) {
                         constraint = Some(*bounds);
                     }
                 },
                 _ => {}
             }
             self.advance(); // consume type
        }
        
        let mut final_type = expected_type.clone();
        
        // = value
        if self.current_token == Token::Eq {
             self.advance();
             
             // CONSTANT FOLDING UPDATE:
             let mut sub_out = String::new();
             let (mut expr_type, constant) = self.parse_expression(&mut sub_out)?;
             
             if let Some(val) = constant {
                 // We have a constant!
                 // If we have a type constraint/expected type, we might want to cast IT NOW?
                 // Or just emit PUSH and let runtime checks handle it?
                 // Optimization: PUSH the final value.
                 match val {
                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                    ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                     _ => {}
                 }
                 // Update expr_type based on constant? (Already correct from parser)
             } else {
                 out.push_str(&sub_out);
             }
             
             // Inject Range Check
             if let Some((min, max)) = constraint {
                 out.push_str("OP_CHECK_RANGE\n");
                 out.push_str(&format!("{}\n", min));
                 out.push_str(&format!("{}\n", max));
             }
             
             // Type Inference
             if final_type == Type::Unknown || final_type == Type::Void {
                 final_type = expr_type.clone();
             }
             
             // Type Check / Cast
             if final_type == Type::Float && expr_type == Type::Int {
                 out.push_str("ITOF\n");
             } else if final_type == Type::Int && expr_type == Type::Float {
                 out.push_str("FTOI\n");
             }
        } else {
            // Default init 0 (implicit)
            out.push_str("PUSH 0\n");
        }
        
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
        self.advance();
        
        let loc = self.declare_var(name, final_type);
        match loc {
            VarLocation::Global(addr) => {
                 // Value is on stack.
                 out.push_str(&format!("PUSH {}\nPOKE\n", addr));
            },
            VarLocation::Local(_) => {
                 // Value is on stack. This IS the local.
            }
        }
        
        Ok(())
    }


    fn parse_print(&mut self, out: &mut String, newline: bool) -> Result<(), CompileError> {
        if self.current_token != Token::LParen { return self.error("Expected ( for print".to_string()); }
        self.advance();
        
        if self.current_token == Token::RParen {
             self.advance();
             if newline {
                 out.push_str("PUSH 10\nPRINT_CHAR\n");
             }
        } else {
             if let Token::String(s) = &self.current_token {
                for c in s.chars() {
                    out.push_str(&format!("PUSH {}\nPRINT_CHAR\n", c as u32));
                }
                self.advance();
             } else {
                let mut sub_out = String::new();
                let (t, constant) = self.parse_expression(&mut sub_out)?;
                
                if let Some(val) = constant {
                    match val {
                        ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                        ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                        ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                        ConstantValue::String(ref s) => {
                             for c in s.chars() {
                                out.push_str(&format!("PUSH {}\nPRINT_CHAR\n", c as u32));
                             }
                             // String constant usually not fully supported as value yet, but for print it works if expanded.
                             // But constant folding usually returns Int/Float/Bool. 
                        },
                        _ => {}
                    }
                    if t == Type::Float {
                         // If it was a float constant, we pushed it as bits (Int).
                         // PRINT_FLOAT expects bits on stack.
                         out.push_str("PRINT_FLOAT\n");
                    } else if matches!(val, ConstantValue::String(_)) {
                         // Handled above loop
                    } else {
                         out.push_str("PRINT_VAL\n");
                    }
                } else {
                    out.push_str(&sub_out);
                    if t == Type::Float {
                        out.push_str("PRINT_FLOAT\n");
                    } else {
                        out.push_str("PRINT_VAL\n");
                    }
                }
             }
             
             if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
             self.advance();
             
             if newline {
                 out.push_str("PUSH 10\nPRINT_CHAR\n");
             }
        }
        
         if self.current_token != Token::SemiColon { 
              self.errors.push(CompileError::new("Expected ;".to_string(), self.prev_span));
              self.synchronize();
              return Ok(());
         }
         self.advance();
         Ok(())
    }

    fn parse_poke(&mut self, out: &mut String) -> Result<(), CompileError> {
        self.advance(); // Skip POKE
        if self.current_token != Token::LParen { return self.error("Expected ( for poke".to_string()); }
        self.advance();
        let mut sub_out = String::new();
        let (_, constant) = self.parse_expression(&mut sub_out)?; // Addr
        if let Some(val) = constant { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

        if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
        self.advance();
        
        let mut sub_out = String::new();
        let (_, constant) = self.parse_expression(&mut sub_out)?; // Val
        if let Some(val) = constant { match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} } } else { out.push_str(&sub_out); }

        if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
        self.advance();
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
        self.advance();
        out.push_str("POKE\n");
        Ok(())
    }

    
    fn parse_expression(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        self.parse_logical_or(out)
    }

    fn parse_logical_or(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out = String::new();
        let (mut left_type, mut left_const) = self.parse_logical_and(&mut sub_out)?;
        
        if left_const.is_none() { out.push_str(&sub_out); }
        
        while self.current_token == Token::Or {
            self.advance();
             // Emit left if it was constant
            if let Some(val) = left_const.take() {
                match val { ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b{1}else{0})), _ => {} }
            }
            
            let mut sub_out_right = String::new();
            let (_, right_const) = self.parse_logical_and(&mut sub_out_right)?; // Right type 
            if let Some(val) = right_const {
                 match val { ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b{1}else{0})), _ => {} }
            } else {
                 out.push_str(&sub_out_right);
            }
            out.push_str("OR\n");
            left_type = Type::Bool; 
            // Result is not constant (simplification)
        }
        Ok((left_type, left_const))
    }

    fn parse_logical_and(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out = String::new();
        let (mut left_type, mut left_const) = self.parse_equality(&mut sub_out)?;
        
        if left_const.is_none() { out.push_str(&sub_out); }
        
        while self.current_token == Token::And {
            self.advance();
            if let Some(val) = left_const.take() {
                match val { ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b{1}else{0})), _ => {} }
            }
            let mut sub_out_right = String::new();
            let (_, right_const) = self.parse_equality(&mut sub_out_right)?; 
            if let Some(val) = right_const {
                 match val { ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b{1}else{0})), _ => {} }
            } else {
                 out.push_str(&sub_out_right);
            }
            out.push_str("AND\n");
            left_type = Type::Bool; 
        }
        Ok((left_type, left_const))
    }
    
    fn parse_equality(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out = String::new();
        let (mut left_type, mut left_const) = self.parse_comparison(&mut sub_out)?;
        
        if left_const.is_none() { out.push_str(&sub_out); }
        
        while self.current_token == Token::EqEq || self.current_token == Token::NotEq {
            let op = self.current_token.clone();
            self.advance();
             if let Some(val) = left_const.take() {
                match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} }
            }
            
            let mut sub_out_right = String::new();
            let (_, right_const) = self.parse_comparison(&mut sub_out_right)?;
             if let Some(val) = right_const {
                 match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} }
            } else {
                 out.push_str(&sub_out_right);
            }
            
            match op {
                Token::EqEq => out.push_str("EQ\n"),
                Token::NotEq => out.push_str("NEQ\n"),
                _ => {}
            }
            left_type = Type::Bool;
        }
        Ok((left_type, left_const))
    }

    fn parse_comparison(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
         let mut sub_out = String::new();
        let (mut left_type, mut left_const) = self.parse_term(&mut sub_out)?;
        
        if left_const.is_none() { out.push_str(&sub_out); }
        
        while matches!(self.current_token, Token::Lt | Token::Gt | Token::LtEq | Token::GtEq) {
            let op = self.current_token.clone();
            self.advance();
             if let Some(val) = left_const.take() {
                match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} }
            }
            let mut sub_out_right = String::new();
            let (_, right_const) = self.parse_term(&mut sub_out_right)?;
             if let Some(val) = right_const {
                 match val { ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)), _ => {} }
            } else {
                 out.push_str(&sub_out_right);
            }
            
            match op {
                Token::Lt => out.push_str("LT\n"),
                Token::Gt => out.push_str("GT\n"),
                Token::LtEq => out.push_str("LTE\n"),
                Token::GtEq => out.push_str("GTE\n"),
                _ => {}
            }
            left_type = Type::Bool;
        }
        Ok((left_type, left_const))
    }

    fn parse_term(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out_left = String::new();
        let (mut left_type, mut left_const) = self.parse_factor(&mut sub_out_left)?;
        
        // If left is not constant, emit it
        if left_const.is_none() {
            out.push_str(&sub_out_left);
        }

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.advance();
            
            let mut sub_out_right = String::new();
            let (right_type, right_const) = self.parse_factor(&mut sub_out_right)?;
            
            // Constant Folding
            if let (Some(l_val), Some(r_val)) = (&left_const, &right_const) {
                 // Both are constants! Compute result.
                 // Note: no code has been emitted to 'out' for left side yet if it was constant.
                 match (l_val, r_val) {
                     (ConstantValue::Int(l), ConstantValue::Int(r)) => {
                         let res = match op {
                             Token::Plus => l + r,
                             Token::Minus => l - r,
                             _ => 0,
                         };
                         left_const = Some(ConstantValue::Int(res));
                         continue; // Skip emitting code
                     },
                     (ConstantValue::Float(l), ConstantValue::Float(r)) => {
                          let res = match op {
                             Token::Plus => l + r,
                             Token::Minus => l - r,
                             _ => 0.0,
                         };
                         left_const = Some(ConstantValue::Float(res));
                         left_type = Type::Float;
                         continue;
                     },
                     (ConstantValue::Int(l), ConstantValue::Float(r)) => {
                          let res = match op {
                             Token::Plus => (*l as f64) + r,
                             Token::Minus => (*l as f64) - r,
                             _ => 0.0,
                         };
                         left_const = Some(ConstantValue::Float(res));
                         left_type = Type::Float;
                         continue;
                     },
                     (ConstantValue::Float(l), ConstantValue::Int(r)) => {
                          let res = match op {
                             Token::Plus => l + (*r as f64),
                             Token::Minus => l - (*r as f64),
                             _ => 0.0,
                         };
                         left_const = Some(ConstantValue::Float(res));
                         left_type = Type::Float;
                         continue;
                     },
                     _ => {} // Fallthrough to dynamic
                 }
            }
            
            // If we are here, at least one is NOT constant.
            // If left was constant, we MUST emit it now because we are processing an operation.
            if let Some(val) = left_const.take() {
                match val {
                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                    ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                    ConstantValue::String(_) => {}, // Strings shouldn't be here in math
                    ConstantValue::None => {},
                }
            }
            // Emit Right side code (if it wasn't constant)
            // If right is constant, we emit PUSH for it.
            if let Some(val) = right_const {
                 match val {
                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                     _ => {}
                }
                out.push_str(&sub_out_right); // Should be empty if it was constant? 
                // Wait. parse_primary returns Some(Constant) AND creates EMPTY out for numbers.
                // But for nested expressions (e.g. (1+2)), out is NOT empty?
                // Ah. In `parse_primary`, `Token::Number` does NOT write to out.
                // So `sub_out_right` is valid code ONLY if `right_const` is None.
                // Correct.
            } else {
                 out.push_str(&sub_out_right);
            }
            
            // Promotion Logic for Dynamic execution
            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                // Right is already on stack
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                
                match op {
                    Token::Plus => out.push_str("FADD\n"),
                    Token::Minus => out.push_str("FSUB\n"),
                    _ => {}
                }
                left_type = Type::Float;
            } else {
                match op {
                    Token::Plus => out.push_str("ADD\n"),
                    Token::Minus => out.push_str("SUB\n"),
                    _ => {}
                }
            }
        }
        
        // Final check: If we ended up with a constant after strict folding (e.g. 1+2-3 -> 0),
        // we return it. Logic above handles continuations.
        // If we broke out of loop because of non-constant usage, `left_const` became None.
        // If loop finished and `left_const` is Some, we return it without emitting.
        Ok((left_type, left_const))
    }

    fn parse_call_args(&mut self, out: &mut String, func_name: &str) -> Result<(), CompileError> {
        if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
        self.advance();
        
        let mut arg_count = 0;
        if self.current_token != Token::RParen {
            loop {
                // We must emit args to stack.
                // If parse_expression returns a constant, we must EMIT IT here.
                let mut sub_out = String::new();
                let (_, constant) = self.parse_expression(&mut sub_out)?;
                
                if let Some(val) = constant {
                    match val {
                        ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                        ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                        ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                        // Strings?
                         ConstantValue::String(_) => {}, // TODO: String support
                        _ => {}
                    }
                } else {
                    out.push_str(&sub_out);
                }
                
                arg_count += 1;
                if self.current_token == Token::Comma { self.advance(); } else { break; }
            }
        }
        if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
        self.advance();
        
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
        self.advance();
        
        if let Some(opcode) = self.get_intrinsic(func_name) {
             out.push_str(&format!("{}\n", opcode));
             // Intrinsics managed via parse_call_args (statement context) are followed by POP.
             // Ensure all intrinsics push a value (even if dummy) to allow POP.
        } else {
             out.push_str(&format!("CALL {} {}\n", func_name, arg_count));
        }
        out.push_str("POP\n");
        Ok(())
    }

    fn parse_factor(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out_left = String::new();
        let (mut left_type, mut left_const) = self.parse_power(&mut sub_out_left)?;
        
        if left_const.is_none() {
            out.push_str(&sub_out_left);
        }

        while matches!(self.current_token, Token::Star | Token::Slash | Token::SlashSlash | Token::Percent) {
            let op = self.current_token.clone();
            self.advance();
            
            let mut sub_out_right = String::new();
            let (right_type, right_const) = self.parse_power(&mut sub_out_right)?;
            
            // Constant Folding
             if let (Some(l_val), Some(r_val)) = (&left_const, &right_const) {
                 match (l_val, r_val) {
                     (ConstantValue::Int(l), ConstantValue::Int(r)) => {
                         // Zero check for div?
                         if (op == Token::Slash || op == Token::SlashSlash || op == Token::Percent) && *r == 0 {
                             // Compile error?
                             return self.error("Division by zero in constant expression".to_string());
                         }
                         let res = match op {
                             Token::Star => l * r,
                             Token::Slash => l / r, // Integer Division
                             Token::SlashSlash => l / r,
                             Token::Percent => l % r,
                             _ => 0,
                         };
                         left_const = Some(ConstantValue::Int(res));
                         continue;
                     },
                     // Float cases... (omitted for brevity, assume similar pattern)
                      (ConstantValue::Float(l), ConstantValue::Float(r)) => {
                             let res = match op {
                             Token::Star => l * r,
                             Token::Slash => l / r,
                             Token::SlashSlash => (l / r).floor(),
                             _ => 0.0,
                         };
                         left_const = Some(ConstantValue::Float(res));
                         left_type = Type::Float;
                         continue;
                     },
                     _ => {}
                 }
            }

            // Not constant
             if let Some(val) = left_const.take() {
                match val {
                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                    ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\n", if b { 1 } else { 0 })),
                    _ => {}
                }
            }
             if let Some(val) = right_const {
                 match val {
                    ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                    ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                     _ => {}
                }
            } else {
                 out.push_str(&sub_out_right);
            }
            
            // Dynamic code gen
            if left_type == Type::Float || right_type == Type::Float {
                 if left_type != Type::Float { out.push_str("ITOF\n"); }
                 if right_type != Type::Float { out.push_str("ITOF\n"); }
                 match op {
                     Token::Star => out.push_str("FMUL\n"),
                     Token::Slash => out.push_str("FDIV\n"),
                     Token::SlashSlash => out.push_str("FFLOORDIV\n"),
                     Token::Percent => return self.error("Modulo operator not supported for floats".to_string()),
                     _ => {} 
                 }
                 left_type = Type::Float;
            } else {
                match op {
                    Token::Star => out.push_str("MUL\n"),
                    Token::Slash => out.push_str("DIV\n"),
                    Token::SlashSlash => out.push_str("FLOORDIV\n"),
                    Token::Percent => out.push_str("MOD\n"),
                    _ => {}
                }
            }
        }
        Ok((left_type, left_const))
    }
    
    fn parse_power(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        let mut sub_out = String::new();
        let (mut left_type, left_const) = self.parse_unary(&mut sub_out)?;
        
        if let Some(val) = &left_const {
             // If left uses constant, we don't emit yet.
        } else {
            out.push_str(&sub_out);
        }

        if self.current_token == Token::StarStar {
            self.advance();
            
            // Power is right associative, but handling constant folding w/ recursion is standard.
             // If left is constant, we need to know if right is constant.
             let mut right_out = String::new();
             let (right_type, right_const) = self.parse_power(&mut right_out)?;
             
             if let (Some(l_val), Some(r_val)) = (&left_const, &right_const) {
                  // Fold
                  // ...
                  // For now, let's just emit dynamic if complex power.
             }
             
             // Emit Left if it was constant
             if let Some(val) = left_const {
                match val {
                     ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                     ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\n", f.to_bits() as i64)),
                     _ => {}
                }
             }
             
             // Emit Right
             if let Some(val) = right_const {
                 match val {
                     ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\n", i)),
                     _ => {}
                 }
             } else {
                 out.push_str(&right_out);
             }

            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                out.push_str("FPOW\n");
                left_type = Type::Float;
            } else {
                out.push_str("POW\n");
            }
            
            return Ok((left_type, None)); // Result of power is dynamic (simplification)
        }
        
        Ok((left_type, left_const))
    }

    fn parse_unary(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        if self.current_token == Token::Minus {
            self.advance();
            // Use temp buffer to support folding
            let mut sub_out = String::new();
            let (operand_type, constant) = self.parse_unary(&mut sub_out)?;
            
            if let Some(val) = constant {
                // Constant Folding: Negate immediate
                match val {
                    ConstantValue::Int(i) => return Ok((Type::Int, Some(ConstantValue::Int(-i)))),
                    ConstantValue::Float(f) => return Ok((Type::Float, Some(ConstantValue::Float(-f)))),
                    _ => {} // Fallback for types that can't be negated normally?
                }
            }
            
            // Not a constant
            out.push_str(&sub_out); // Emit buffered code
            if operand_type == Type::Float {
                let neg_one: f64 = -1.0;
                let bits = neg_one.to_bits() as i64;
                out.push_str(&format!("PUSH {}\nFMUL\n", bits));
                Ok((Type::Float, None))
            } else {
                out.push_str("PUSH 0\nSWAP\nSUB\n"); // 0 - x
                Ok((operand_type, None))
            }
        } else if self.current_token == Token::Not {
            // Unary NOT
            self.advance();
            let mut sub_out = String::new();
            let (operand_type, constant) = self.parse_unary(&mut sub_out)?;
            
            if let Some(val) = constant {
                match val {
                    ConstantValue::Bool(b) => return Ok((Type::Bool, Some(ConstantValue::Bool(!b)))),
                    ConstantValue::Int(i) => return Ok((Type::Bool, Some(ConstantValue::Bool(i == 0)))),
                    _ => {}
                }
            }
            
            out.push_str(&sub_out);
            out.push_str("PUSH 0\nEQ\n"); 
            Ok((Type::Bool, None))
        } else {
            self.parse_primary(out)
        }
    }

    fn parse_primary(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        match &self.current_token {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                out.push_str(&format!("PUSH {}\n", val));
                Ok((Type::Int, Some(ConstantValue::Int(val))))
            },
            Token::Float(f) => {
                let val = *f;
                self.advance();
                let bits = val.to_bits() as i64;
                out.push_str(&format!("PUSH {}\n", bits));
                Ok((Type::Float, Some(ConstantValue::Float(val))))
            },
            Token::True => {
                self.advance();
                out.push_str("PUSH 1\n");
                Ok((Type::Bool, Some(ConstantValue::Bool(true))))
            },
            Token::False => {
                self.advance();
                out.push_str("PUSH 0\n");
                Ok((Type::Bool, Some(ConstantValue::Bool(false))))
            },
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                out.push_str("PUSH 0 ; String Literal Placeholder\n"); 
                Ok((Type::String, Some(ConstantValue::String(val))))
            },
            Token::Identifier(name) => {
                let part1 = name.clone();
                self.advance();
                
                if self.current_token == Token::LParen {
                    // Function Call
                    self.advance(); // Skip (
                    let mut arg_count = 0;
                    if self.current_token != Token::RParen {
                           loop {
                               self.parse_expression(out)?;
                               arg_count += 1;
                               if self.current_token == Token::Comma { self.advance(); } else { break; }
                           }
                    }
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    
                    if let Some(opcode) = self.get_intrinsic(&part1) {
                        out.push_str(&format!("{}\n", opcode));
                    } else {
                        // Emit CALL with arg count
                        out.push_str(&format!("CALL {} {}\n", part1, arg_count));
                    }
                    Ok((Type::Int, None))
                } else {
                    // Variable Access
                    let (loc, mut typ) = match self.resolve_var(&part1) {
                        Some(r) => r,
                        None => {
                            // Check intrinsics or other globals if not found?
                            // For now error
                            return self.error(format!("Undefined variable '{}'", part1));
                        }
                    };
                    
                    match loc {
                        VarLocation::Global(addr) => {
                             out.push_str(&format!("PUSH {}\nPEEK\n", addr)); 
                        },
                        VarLocation::Local(idx) => {
                             out.push_str(&format!("GET_LOCAL {}\n", idx));
                        }
                    }
                    
                    // Handle Chain: .x.y.z
                    while self.current_token == Token::Dot {
                        self.advance();
                        let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                        self.advance();
                        
                        let offset = if let Type::Class(cname) = &typ {
                             if let Some(cinfo) = self.classes.get(cname) {
                                  if let Some(off) = cinfo.fields.get(&member) {
                                      *off
                                  } else { return self.error(format!("Class '{}' has no field '{}'", cname, member)); }
                             } else { return self.error(format!("Unknown class '{}'", cname)); }
                        } else {
                             // Fallback: Search all classes for field
                             let mut found = None;
                             for (cname, cinfo) in &self.classes {
                                 if let Some(off) = cinfo.fields.get(&member) {
                                     if found.is_some() { return self.error(format!("Ambiguous field '{}' (found in multiple classes)", member)); }
                                     found = Some(*off);
                                     // Optimization: Could infer type here? 
                                     // typ = Type::Class(cname.clone());
                                 }
                             }
                             if let Some(off) = found {
                                 off
                             } else {
                                 return self.error(format!("Field '{}' not found in any class (variable '{}' type unknown)", member, part1));
                             }
                        };
                        
                        out.push_str(&format!("PUSH {}\nOP_ADD\nPEEK\n", offset));
                        // Typ becomes Unknown unless we track field types
                        typ = Type::Unknown;
                    }
                    
                    Ok((typ, None))
                }
            },
            Token::LParen => {
                self.advance();
                let t = self.parse_expression(out)?;
                if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                self.advance();
                Ok(t)
            },
            Token::Peek => {
                self.advance();
                if self.current_token != Token::LParen { return self.error("Expected ( for peek".to_string()); }
                self.advance();
                self.parse_expression(out)?;
                if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                self.advance();
                out.push_str("PEEK\n");
                Ok((Type::Int, None))
            },
            Token::New => {
                self.advance();
                let name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected class name after new".to_string()),
                };
                self.advance();
                
                let size = if let Some(info) = self.classes.get(&name) {
                    info.size as i32
                } else {
                     return self.error(format!("Undefined class '{}'", name));
                };
                
                out.push_str(&format!("PUSH {}\nPUSH 1\nOP_IMG_ALLOC\n", size));
                
                if self.current_token == Token::LParen { 
                     self.advance(); 
                     if self.current_token != Token::RParen { loop { self.parse_expression(out)?; if self.current_token==Token::Comma{self.advance();}else{break;} } }
                     self.advance(); 
                }
                
                Ok((Type::Class(name), None)) 
            },
            Token::Input => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("INPUT\n");
                 Ok((Type::Int, None))
            },
            Token::KwInt => {
                 self.advance();
                 if self.current_token == Token::LParen {
                     self.advance();
                     let mut sub_out = String::new();
                     let (t, c) = self.parse_expression(&mut sub_out)?;
                     
                     if let Some(val) = c {
                         match val {
                             ConstantValue::Float(f) => return Ok((Type::Int, Some(ConstantValue::Int(f as i64)))),
                             ConstantValue::Int(i) => return Ok((Type::Int, Some(ConstantValue::Int(i)))),
                             _ => {}
                         }
                     }
                     out.push_str(&sub_out);
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if t == Type::Float { out.push_str("FTOI\n"); }
                     Ok((Type::Int, None))
                 } else {
                     return self.error("Expected ( after int cast".to_string());
                 }
            },
            Token::KwFloat => {
                 self.advance();
                 if self.current_token == Token::LParen {
                     self.advance();
                     let mut sub_out = String::new();
                     let (t, c) = self.parse_expression(&mut sub_out)?;
                     
                      if let Some(val) = c {
                         match val {
                             ConstantValue::Int(i) => return Ok((Type::Float, Some(ConstantValue::Float(i as f64)))),
                             ConstantValue::Float(f) => return Ok((Type::Float, Some(ConstantValue::Float(f)))),
                             _ => {}
                         }
                     }
                     
                     out.push_str(&sub_out);
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if t != Type::Float { out.push_str("ITOF\n"); }
                     Ok((Type::Float, None))
                 } else {
                     return self.error("Expected ( after float cast".to_string());
                 }
            },
            Token::SysPlatform => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_SYS_PLATFORM\n");
                 Ok((Type::Int, None))
            },
            Token::CamCount => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_COUNT\n");
                 Ok((Type::Int, None))
            },
            Token::IsKeyDown => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IS_KEY_DOWN\n");
                 Ok((Type::Bool, None))
            },
            Token::Sin => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FSIN\n");
                 Ok((Type::Float, None))
            },
            Token::Alloc => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_ALLOC\n");
                 Ok((Type::Pointer(Box::new(Type::Unknown)), None))
            },
            Token::Free => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FREE\n");
                 Ok((Type::Void, None))
            },
            Token::Cos => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FCOS\n");
                 Ok((Type::Float, None))
            },
            Token::Sqrt => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FSQRT\n");
                 Ok((Type::Float, None))
            },
            Token::CamCapture => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // Cam ID
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_CAPTURE\n");
                 Ok((Type::Int, None))
            },
            Token::ImgAlloc => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // width
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // height
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_ALLOC\n");
                 Ok((Type::Int, None))
            },
            Token::ImgResize => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // new_w
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // new_h
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_RESIZE\n");
                 Ok((Type::Int, None))
            },
            Token::ImgCrop => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // x
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // y
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // w
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // h
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_CROP\n");
                 Ok((Type::Int, None))
            },
            Token::ImgGrayscale => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_GRAYSCALE\n");
                 Ok((Type::Int, None))
            },
            Token::UpperCase => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // value
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_TO_UPPER\n");
                 Ok((Type::Int, None))
            },
            Token::LowerCase => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // value
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_TO_LOWER\n");
                 Ok((Type::Int, None))
            },
            Token::ImgGet => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // x
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // y
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_GET\n");
                 Ok((Type::Int, None)) // Returns Pixel Value
            },
            
            _ => return self.error(format!("Unexpected token in expression: {:?}", self.current_token)),
        }
    }
    fn get_intrinsic(&self, name: &str) -> Option<String> {
        match name {
            "time" => Some("OP_TIME".to_string()),
            "random" => Some("OP_RANDOM".to_string()),
            "system" => Some("OP_SYSTEM".to_string()),
            "vision_detect" => Some("OP_VISION_DETECT".to_string()),
            // Graphics intrinsics
            "gfx_clear" => Some("OP_GFX_CLEAR".to_string()),
            "gfx_pixel" => Some("OP_DRAW_PIXEL".to_string()),
            "gfx_line" => Some("OP_DRAW_LINE".to_string()),
            "gfx_circle" => Some("OP_DRAW_CIRCLE".to_string()),
            _ => None
        }
    }
}
