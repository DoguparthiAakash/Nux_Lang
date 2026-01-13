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
    Unknown
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
                 Token::If | Token::While | Token::Print | Token::Return => return, // Start of new statement
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
            // println!("DEBUG: Token: {:?}", self.current_token); // TRACE
            match &self.current_token {
                Token::EOF => break,
                Token::Class => {
                     if let Err(e) = self.parse_class(&mut definitions) {
                          self.errors.push(e);
                          self.synchronize();
                     }
                },
                Token::Func => {
                     if let Err(e) = self.parse_func(&mut definitions, "") {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Identifier(_) | Token::Print | Token::Println | Token::Input |
                Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |
                Token::Var | Token::Return | Token::Lock | Token::Unlock | Token::Peek |
                Token::KwInt | Token::KwFloat | Token::KwByte | Token::KwShort | Token::KwLong | Token::KwChar | Token::KwString => {
                    if let Err(e) = self.parse_statement_or_expr(&mut main_body) {
                        self.errors.push(e);
                        self.synchronize();
                    }
                },
                Token::Import => { // Preprocessor-like include
                    // import "filename";
                    self.advance();
                    let filename = match &self.current_token {
                        Token::String(s) => s.clone(),
                        _ => {
                            self.errors.push(CompileError::new("Expected filename string".to_string(), self.current_span));
                            continue;
                        }
                    };
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
                    if let Ok(content) = std::fs::read_to_string(&filename) {
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
        self.advance(); // consume 'class'
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected class name".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LBrace { return self.error("Expected '{' after class name".to_string()); }
        self.advance();
        
        // Inside class, we expect functions (methods).
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if self.current_token == Token::Func {
                self.parse_func(out, &name)?;
            } else {
                return self.error("Only functions allowed in classes for now".to_string());
            }
        }
        
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

    fn parse_func(&mut self, out: &mut String, class_prefix: &str) -> Result<(), CompileError> {
        self.advance(); // consume 'func'
        
        if self.current_token == Token::Var {
             self.advance(); // consume 'var'
             // Parse Bound Type Declaration
             // func var MyType { start=...; end=...; }
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

        if self.current_token != Token::LBrace { return self.error("Expected '{'".to_string()); }
        
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
        
        let num_args = args.len() as i64;
        self.local_offset = num_args; // Locals start after arguments
        
        for (i, arg) in args.iter().enumerate() {
             // Arguments are at positive offsets 0, 1, 2, ...
             let offset = i as i64;
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
                 self.advance(); // skip name
                 if self.current_token == Token::Eq {
                       // Assignment
                        match self.resolve_var(&part1) {
                            Some((loc, _typ)) => {
                                self.advance(); // Skip =
                                self.parse_expression(out)?;
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
                                   self.parse_expression(out)?;
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
                               self.parse_expression(out)?;
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
                      out.push_str(&format!("CALL {} {}\nPOP\n", part1, arg_count));
                      
                 } else if self.current_token == Token::Dot {
                      self.advance(); // Skip .
                      let method = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected method name".to_string()) };
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
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
                      
                      if expect_semi {
                          if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                          self.advance();
                      } else if self.current_token == Token::SemiColon { self.advance(); }
                      
                      out.push_str(&format!("CALL {}_{} {}\nPOP\n", part1, method, arg_count));
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
             Token::Return => {
                 self.advance();
                 if self.current_token == Token::SemiColon {
                     out.push_str("PUSH 0\nRET\n");
                     self.advance();
                 } else {
                     self.parse_expression(out)?;
                     if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                     out.push_str("RET\n");
                     self.advance();
                 }
             },
             Token::If => {
                  self.advance(); // skip if
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  self.parse_expression(out)?;
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
                  self.parse_expression(out)?;
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
                       self.parse_expression(out)?;
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
                 self.parse_expression(out)?; // Handle
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
                 self.parse_expression(out)?; // Handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // X
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // Y
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
                 self.parse_expression(out)?; // Handle
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_FREE\n");
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFilter => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected ( for img_filter".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // Handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // Mode
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
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // x
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // y
                 if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; // color
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 // OP_IMG_SET not defined in VM yet, but let's leave it out or implement strict?
                 // User didn't ask for set. Let's just comment it out or emit panic?
                 // Wait, I declared the token. I should likely emit nothing or error.
                 // Actually I'll implement it as placeholder or error to avoid crash.
                 return self.error("img_set not implemented yet".to_string());
             },
             Token::ImgGet => {
                  // If used as statement: img_get(h,x,y); -> pop result
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  self.parse_expression(out)?; // h
                  self.advance(); // ,
                  self.parse_expression(out)?; // x
                  self.advance(); // ,
                  self.parse_expression(out)?; // y
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
        self.advance(); // consume keyword (int, var, etc)
        
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
             let expr_type = self.parse_expression(out)?;
             
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
                 // We need to Pop to Global.
                 // POKE expects [Value, Addr]. 
                 // Stack has [Value].
                 // Wait. POKE is [Addr, Value]? Or [Value, Addr]?
                 // VM `OP_POKE`: `let addr = self.pop(); let val = self.pop();`. 
                 // So Stack must be [..., Value, Addr].
                 // Currently Stack has [..., Value].
                 // If we `PUSH addr`, Stack is [..., Value, Addr].
                 // Then `POKE`. Correct.
                 out.push_str(&format!("PUSH {}\nPOKE\n", addr));
            },
            VarLocation::Local(_) => {
                 // Value is on stack. This IS the local.
                 // No action.
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
                let t = self.parse_expression(out)?;
                if t == Type::Float {
                    out.push_str("PRINT_FLOAT\n");
                } else {
                    out.push_str("PRINT_VAL\n");
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
        self.parse_expression(out)?; // Addr
        if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
        self.advance();
        self.parse_expression(out)?; // Val
        if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
        self.advance();
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
        self.advance();
        out.push_str("POKE\n");
        Ok(())
    }

    
    fn parse_expression(&mut self, out: &mut String) -> Result<Type, CompileError> {
        self.parse_logical_or(out)
    }

    fn parse_logical_or(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_logical_and(out)?;
        
        while self.current_token == Token::Or {
            self.advance();
            let _right_type = self.parse_logical_and(out)?; // Right type is consumed
            out.push_str("OR\n");
            left_type = Type::Bool; // Logic ops always bool
        }
        Ok(left_type)
    }

    fn parse_logical_and(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_equality(out)?;
        
        while self.current_token == Token::And {
            self.advance();
            let _right_type = self.parse_equality(out)?; // Right type is consumed
            out.push_str("AND\n");
            left_type = Type::Bool; 
        }
        Ok(left_type)
    }
    
    fn parse_equality(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_comparison(out)?;
        
        while self.current_token == Token::EqEq || self.current_token == Token::NotEq {
            let op = self.current_token.clone();
            self.advance();
            let right_type = self.parse_comparison(out)?;
            
            // Type check and promotion for equality
            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                match op {
                    Token::EqEq => out.push_str("FEQ\n"),
                    Token::NotEq => out.push_str("FNEQ\n"),
                    _ => {}
                }
            } else {
                match op {
                    Token::EqEq => out.push_str("EQ\n"),
                    Token::NotEq => out.push_str("NEQ\n"),
                    _ => {}
                }
            }
            left_type = Type::Bool; // Result is Bool
        }
        Ok(left_type)
    }

    fn parse_comparison(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_term(out)?;
        
        while matches!(self.current_token, Token::Lt | Token::Gt | Token::LtEq | Token::GtEq) {
            let op = self.current_token.clone();
            self.advance();
            let right_type = self.parse_term(out)?;
            
            // Type check and promotion for comparison
            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                match op {
                    Token::Lt => out.push_str("FLT\n"),
                    Token::Gt => out.push_str("FGT\n"),
                    Token::LtEq => out.push_str("FLTE\n"),
                    Token::GtEq => out.push_str("FGTE\n"),
                    _ => {}
                }
            } else {
                match op {
                    Token::Lt => out.push_str("LT\n"),
                    Token::Gt => out.push_str("GT\n"),
                    Token::LtEq => out.push_str("LTE\n"),
                    Token::GtEq => out.push_str("GTE\n"),
                    _ => {}
                }
            }
            left_type = Type::Bool;
        }
        Ok(left_type)
    }

    fn parse_term(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_factor(out)?;
        
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.advance();
            
            // Need to buffer right side code to inject conversion?
            // Wait, we are writing to `out` directly.
            // If left is Int and right is Float, we need to convert Left to Float BEFORE right is pushed?
            // No, Left is already pushed. Integer is on stack.
            // If we find right is Float, we are in trouble if we didn't convert Left.
            // Because we only know right's type AFTER parsing it.
            // And parsing it emits code to push it.
            
            // Solution: 
            // 1. If Left is Int, we assume Int math.
            // 2. Parse Right.
            // 3. If Right turns out to be Float:
            //    - If Left was Int, we need to convert Left (generic stack swap? `ITOF` under top? No).
            //    - We need to `SWAP, ITOF, SWAP`? VM doesn't have Swap.
            //    - Better: Simple Compiler limitation -> Float Must be on Left? Or explicit cast?
            //    - Or: We just emit generic ops and VM handles generic types? (Dynamic Typing). 
            //      But user asked for explicit sizes. VM is `i64`.
            //      Float bits in i64.
            //      ADD will mangle float bits. FADD works.
            
            // Alternative:
            // Buffer the Right side code.
            // 1. Evaluate Left type.
            // 2. Capture Right output in temp buffer.
            // 3. Evaluate Right type.
            // 4. Emit corrections.
            
            let mut right_out = String::new();
            let right_type = self.parse_factor(&mut right_out)?;
            
            // Promotion Logic
            if left_type == Type::Float || right_type == Type::Float {
                // Float Arithmetic
                if left_type != Type::Float {
                     // Left is Int (on stack). Convert to Float.
                     // But Right code isn't emitted yet.
                     out.push_str("ITOF\n");
                }
                out.push_str(&right_out);
                if right_type != Type::Float {
                    // Right is Int (on top of stack). Convert.
                     out.push_str("ITOF\n");
                }
                
                match op {
                    Token::Plus => out.push_str("FADD\n"),
                    Token::Minus => out.push_str("FSUB\n"),
                    _ => {}
                }
                left_type = Type::Float;
            } else {
                // Int Arithmetic
                out.push_str(&right_out);
                match op {
                    Token::Plus => out.push_str("ADD\n"),
                    Token::Minus => out.push_str("SUB\n"),
                    _ => {}
                }
                // left_type remains Int (or whatever it was)
            }
        }
        Ok(left_type)
    }

    fn parse_factor(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_power(out)?;
        
        while matches!(self.current_token, Token::Star | Token::Slash | Token::SlashSlash | Token::Percent) {
            let op = self.current_token.clone();
            self.advance();
            
            let mut right_out = String::new();
            let right_type = self.parse_power(&mut right_out)?;
            
            if left_type == Type::Float || right_type == Type::Float {
                 if left_type != Type::Float { out.push_str("ITOF\n"); }
                 out.push_str(&right_out);
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
                out.push_str(&right_out);
                match op {
                    Token::Star => out.push_str("MUL\n"),
                    Token::Slash => out.push_str("DIV\n"),
                    Token::SlashSlash => out.push_str("FLOORDIV\n"),
                    Token::Percent => out.push_str("MOD\n"),
                    _ => {}
                }
            }
        }
        Ok(left_type)
    }
    
    fn parse_power(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_unary(out)?;
        
        // Power is right-associative: 2**3**2 = 2**(3**2) = 2**9 = 512
        if self.current_token == Token::StarStar {
            self.advance();
            let mut right_out = String::new();
            let right_type = self.parse_power(&mut right_out)?; // Recursive for right-associativity
            
            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                out.push_str(&right_out);
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                out.push_str("FPOW\n");
                left_type = Type::Float;
            } else {
                out.push_str(&right_out);
                out.push_str("POW\n");
            }
        }
        
        Ok(left_type)
    }

    fn parse_unary(&mut self, out: &mut String) -> Result<Type, CompileError> {
        if self.current_token == Token::Minus {
            self.advance();
            let operand_type = self.parse_unary(out)?;
            if operand_type == Type::Float {
                out.push_str("PUSH -1.0\nITOF\nFMUL\n");
                return Ok(Type::Float);
            } else {
                out.push_str("PUSH 0\nSWAP\nSUB\n");
                return Ok(operand_type);
            }
        }
        
        if self.current_token == Token::Not {
            // Unary NOT: !x is equivalent to (x == 0)
            self.advance();
            let _ = self.parse_unary(out)?;
            out.push_str("PUSH 0\nEQ\n"); // 0 -> 1, non-zero -> 0
            return Ok(Type::Bool);
        }
        
        self.parse_primary(out)
    }

    fn parse_primary(&mut self, out: &mut String) -> Result<Type, CompileError> {
        match &self.current_token {
            Token::Input => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("INPUT\n");
                 Ok(Type::Int) // Input returns an integer
            },
            
            // --- Introspection ---
            Token::SysPlatform => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_SYS_PLATFORM\n");
                 Ok(Type::Int)
            },
            Token::CamCount => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_COUNT\n");
                 Ok(Type::Int)
            },
            
            // --- VISION EXPRESSIONS ---
            Token::CamCapture => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // Cam ID
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_CAPTURE\n");
                 Ok(Type::Int)
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
                 Ok(Type::Int) // Returns Handle ID
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
                 Ok(Type::Int)
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
                 Ok(Type::Int)
            },
            Token::ImgGrayscale => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // handle
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IMG_GRAYSCALE\n");
                 Ok(Type::Int) 
            },
            Token::UpperCase => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // value
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_TO_UPPER\n");
                 Ok(Type::Int)
            },
            Token::LowerCase => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?; // value
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_TO_LOWER\n");
                 Ok(Type::Int)
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
                 Ok(Type::Int) // Returns Pixel Value
            },

            Token::Number(n) => {
                let val = *n;
                self.advance();
                out.push_str(&format!("PUSH {}\n", val));
                Ok(Type::Int)
            },
            Token::Float(f) => {
                let val = *f;
                self.advance();
                // We need to push raw bits of f64
                let bits = val.to_bits() as i64;
                out.push_str(&format!("PUSH {}\n", bits));
                Ok(Type::Float)
            },
            Token::True => {
                self.advance();
                out.push_str("PUSH 1\n");
                Ok(Type::Bool)
            },
            Token::False => {
                self.advance();
                out.push_str("PUSH 0\n");
                Ok(Type::Bool)
            },
            Token::String(s) => {
                // String Literal
                // TODO: Store string in data segment and push pointer.
                // For now, fail or char pointer mock?
                // Minimal: Push chars?
                // Real: "String" type support not fully in VM yet.
                // Let's treat as sequence of chars?
                // Just error for now or basic?
                // Allow "String" type but value is 0?
                // Or basic char loop print support.
                // Let's return String type but emit nothing valuable yet except for print?
                // Actually, existing `print("foo")` works by iterating.
                // If this is part of expression `var s = "foo"`, we need value.
                // Implement: String Table?
                // Hack: Pass raw string if it's argument to print?
                // But this is parse_primary.
                let s_val = s.clone();
                self.advance();
                // We don't have good support for string variables yet.
                // Just push 0 and warn.
                out.push_str("PUSH 0 ; String Literal Placeholder\n"); 
                Ok(Type::String)
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
                    // Emit CALL with arg count
                    out.push_str(&format!("CALL {} {}\n", part1, arg_count));
                    Ok(Type::Int)
                } else if self.current_token == Token::Dot {
                     // Method call...
                     self.advance();
                     let method = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected method name".to_string()) };
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
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
                     
                     out.push_str(&format!("CALL {}_{} {}\n", part1, method, arg_count));
                     Ok(Type::Int)
                } else {
                    // Variable Access
                    match self.resolve_var(&part1) {
                        Some((loc, typ)) => {
                            match loc {
                                VarLocation::Global(addr) => {
                                    out.push_str(&format!("PUSH {}\nPEEK\n", addr)); 
                                },
                                VarLocation::Local(offset) => {
                                    out.push_str(&format!("GET_LOCAL {}\n", offset));
                                }
                            }
                            Ok(typ)
                        },
                        None => self.error(format!("Undefined variable: {}", part1)),
                    }
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
                Ok(Type::Int) // Peek returns Int (raw)
            },
            _ => return self.error(format!("Unexpected token in expression: {:?}", self.current_token)),
        }
    }
}
