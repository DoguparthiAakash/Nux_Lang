use crate::lexer::{Lexer, Token, Span};
use std::vec::Vec;
use std::vec;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::format;
use core::fmt;

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
           parser.errors.push(e);
           Err(parser.errors)
        }
    }
}

pub fn compile_high_level(source: &str) -> Result<Vec<u8>, Vec<CompileError>> {
    match compile_to_asm_source(source) {
        Ok(asm) => {
            crate::assembler::compile(&asm).map_err(|e| vec![CompileError::new(e, Span { line: 0, col: 0 })])
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
    Class(String)
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub fields: BTreeMap<String, u32>,
    pub size: u32,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    current_span: Span,
    prev_span: Span,
    var_addr_counter: u64,
    label_id_counter: usize,
    scopes: Vec<BTreeMap<String, (VarLocation, Type)>>,
    asm_output: String,
    errors: Vec<CompileError>,
    loop_stack: Vec<(String, String)>, 
    local_offset: i64, 
    bound_types: BTreeMap<String, (i64, i64)>,
    classes: BTreeMap<String, ClassInfo>,
    current_class_name: Option<String>,
    current_class_fields: BTreeMap<String, u32>,
    pub in_unsafe_block: bool,
    pub enums: BTreeMap<String, BTreeMap<String, i64>>,
    pub traits: BTreeMap<String, Vec<String>>,
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
        
        let mut scopes = Vec::new();
        scopes.push(BTreeMap::new());

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
            bound_types: BTreeMap::new(),
            classes: BTreeMap::new(),
            current_class_name: None,
            current_class_fields: BTreeMap::new(),
            in_unsafe_block: false,
            enums: BTreeMap::new(),
            traits: BTreeMap::new(),
        }
    }

    fn advance(&mut self) {
        self.prev_span = self.current_span;
        let (tok, span) = self.lexer.next_token();
        self.current_token = tok;
        self.current_span = span;
    }

    fn enter_scope(&mut self) {
        self.scopes.push(BTreeMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: String, typ: Type) -> VarLocation {
        if self.scopes.len() > 1 {
            let offset = self.local_offset;
            self.local_offset += 1;
            let loc = VarLocation::Local(offset);
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name, (loc, typ));
            }
            loc
        } else {
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

    fn synchronize(&mut self) {
        self.advance();
        while self.current_token != Token::EOF {
             if self.current_token == Token::SemiColon {
                 self.advance();
                 return;
             }
             match self.current_token {
                 Token::Class | Token::Func | Token::Var | Token::For | 
                 Token::If | Token::While | Token::Print | Token::Return => return,
                 Token::RBrace => return,
                 _ => self.advance(),
             }
        }
    }
    
    fn error_at_current(&mut self, msg: String) {
        self.errors.push(CompileError::new(msg, self.current_span));
    }

    fn parse_to_asm(&mut self) -> Result<String, CompileError> {
        self.emit("; Auto-Generated by Nux High-Level Compiler (Kernel)");
        self.emit("JMP __start_execution"); 
        
        let mut main_body = String::new();
        let mut definitions = String::new();
        
        loop {
            match &self.current_token {
                Token::EOF => break,
                Token::Class => {
                     if let Err(e) = self.parse_class(&mut definitions) {
                          self.errors.push(e);
                          self.synchronize();
                     }
                },
                Token::Func | Token::Fn => {
                     if let Err(e) = self.parse_func(&mut definitions, "") {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Trait => {
                     if let Err(e) = self.parse_trait() {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Identifier(_) | Token::Print | Token::Println | Token::Input |
                Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |
                Token::Var | Token::Let | Token::Const | Token::Return | Token::Lock | Token::Unlock | Token::Peek |
                Token::KwInt | Token::KwFloat | Token::KwByte | Token::KwShort | Token::KwLong | Token::KwChar | Token::KwString => {
                    if let Err(e) = self.parse_statement_or_expr(&mut main_body) {
                        self.errors.push(e);
                        self.synchronize();
                    }
                },
                Token::Import => { 
                    self.advance();
                     let raw_name = match &self.current_token {
                        Token::String(s) => s.clone(),
                        _ => {
                            self.errors.push(CompileError::new("Expected import path string".to_string(), self.current_span));
                            continue;
                        }
                    };
                    self.advance();
                    if self.current_token == Token::SemiColon { self.advance(); }
                    
                    let rel = raw_name.replace(".", "/");
                    let file_name = format!("{}.nux", rel);
                    
                    let mut src_content: Option<String> = None;
                    
                    if let Ok(env_path) = std::env::var("NUX_LIB_PATH") {
                        let path = std::path::Path::new(&env_path).join(&file_name);
                        if path.exists() {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                src_content = Some(content);
                            }
                        }
                    }
                    
                    if src_content.is_none() {
                         let path = std::path::Path::new("lib").join(&file_name);
                         if let Ok(content) = std::fs::read_to_string(&path) {
                             src_content = Some(content);
                         }
                    }
                    
                    if let Some(src) = src_content {
                        let mut imported_defs = String::new();
                        self.parse_imported_source(&src, &mut definitions);
                        
                    } else {
                        self.errors.push(CompileError::new(format!("Import not found: {} (Searched in NUX_LIB_PATH and lib/)", file_name), self.prev_span));
                    }
                },
                Token::SemiColon => self.advance(), 
                Token::LBrace => {
                     if let Err(e) = self.parse_block(&mut main_body) {
                         self.errors.push(e);
                         self.synchronize();
                     }
                },
                Token::Poke => {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected ( for poke".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; 
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; 
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                    self.advance();
                    main_body.push_str("POKE\n");
                },
                Token::Poke32 => {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected ( for poke32".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; 
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(&mut main_body)?; 
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                    self.advance();
                    main_body.push_str("OP_POKE32\n");
                },
                _ => {
                    let msg = self.format_unexpected_token(&self.current_token, "Unexpected token at top level:");
                    self.error_at_current(msg);
                    self.advance();
                }
            }
        }
        
        self.asm_output.push_str(&definitions);
        
        if !main_body.trim().is_empty() {
            self.emit("; Implicit main function");
            self.emit("JMP skip___main");
            self.emit("__main:");
            self.asm_output.push_str(&main_body);
            self.emit("PUSH 0");
            self.emit("RET");
            self.emit("skip___main:");
        }
        
        self.emit("__start_execution:");
        if !main_body.trim().is_empty() {
            self.emit("CALL __main 0");
            self.emit("POP");
        }
        self.emit("EXIT");
        
        Ok(self.asm_output.clone())
    }

    fn parse_imported_source(&mut self, source: &str, definitions: &mut String) {
        let mut sub_parser = Parser::new(source);
        loop {
            match sub_parser.current_token {
                Token::EOF => break,
                Token::Class => {
                    if let Err(e) = sub_parser.parse_class(definitions) {
                         eprintln!("Import Parse Error (Class): {:?}", e);
                         break;
                    }
                },
                Token::Func => {
                    if let Err(e) = sub_parser.parse_func(definitions, "") {
                         eprintln!("Import Parse Error (Func): {:?}", e);
                         break;
                    }
                },
                Token::Import => { 
                     if let Token::String(raw_name) = &sub_parser.current_token {
                        let rel = raw_name.replace(".", "/");
                        let file_name = format!("{}.nux", rel);
                        
                        let mut src_content: Option<String> = None;
                        
                        if let Ok(env_path) = std::env::var("NUX_LIB_PATH") {
                            let path = std::path::Path::new(&env_path).join(&file_name);
                            if path.exists() {
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    src_content = Some(content);
                                }
                            }
                        }
                        
                        if src_content.is_none() {
                             let path = std::path::Path::new("lib").join(&file_name);
                             if let Ok(content) = std::fs::read_to_string(&path) {
                                 src_content = Some(content);
                             }
                        }
                        
                        if let Some(src) = src_content {
                             self.parse_imported_source(&src, definitions);
                        } else {
                             eprintln!("Warning: Transitive import not found: {}", raw_name);
                        }
                     }
                     sub_parser.advance();
                     if sub_parser.current_token == Token::SemiColon { sub_parser.advance(); }
                },
                _ => { sub_parser.advance(); }
            }
        }
        
        for (k, v) in sub_parser.classes {
            self.classes.insert(k, v);
        }
        for (k, v) in sub_parser.bound_types {
            self.bound_types.insert(k, v);
        }
    }

    fn error<T>(&self, msg: String) -> Result<T, CompileError> {
        Err(CompileError::new(msg, self.current_span))
    }

    fn format_unexpected_token(&self, token: &Token, context: &str) -> String {
        match token {
            Token::Identifier(s) => {
                let lower = s.to_lowercase();
                match lower.as_str() {
                    "string" | "int" | "float" | "char" | "byte" | "short" | "long" => {
                        format!("{} Unexpected identifier '{}'. Note: Nux data types are lowercase (e.g. '{}').", context, s, lower)
                    },
                    _ => format!("{} Unexpected identifier '{}'. Check your syntax and variable names.", context, s)
                }
            },
            _ => format!("{} {:?}", context, token)
        }
    }

    fn parse_trait(&mut self) -> Result<(), CompileError> {
        self.advance();
        let trait_name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected trait name".to_string())
        };
        self.advance();
        
        if self.current_token != Token::LBrace {
            return self.error("Expected '{' for trait".to_string());
        }
        self.advance();
        
        let mut methods = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if self.current_token == Token::SemiColon {
                self.advance();
                continue;
            }
            if self.current_token == Token::Func || self.current_token == Token::Fn {
                self.advance();
                let method_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected method name in trait".to_string())
                };
                self.advance();
                if self.current_token != Token::LParen { return self.error("Expected '(' in trait method".to_string()); }
                self.advance();
                while self.current_token != Token::RParen && self.current_token != Token::EOF {
                    self.advance();
                }
                if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
                self.advance();
                if self.current_token == Token::Minus {
                    self.advance(); if self.current_token == Token::Gt { self.advance(); self.advance(); }
                }
                if self.current_token == Token::SemiColon { self.advance(); }
                methods.push(method_name);
            } else {
                return self.error("Only functions are allowed in traits".to_string());
            }
        }
        
        if self.current_token != Token::RBrace {
            return self.error("Expected '}' at end of trait".to_string());
        }
        self.advance();
        self.traits.insert(trait_name, methods);
        Ok(())
    }

    fn parse_class(&mut self, out: &mut String) -> Result<(), CompileError> {
        self.advance(); 
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected class name".to_string()),
        };
        self.advance();
        
        self.current_class_name = Some(name.clone());
        self.current_class_fields.clear();
        
        let mut implemented_traits = Vec::new();
        if self.current_token == Token::Colon {
            self.advance();
            while let Token::Identifier(t) = &self.current_token {
                implemented_traits.push(t.clone());
                self.advance();
                if self.current_token == Token::Comma { self.advance(); } else { break; }
            }
        }
        
        if self.current_token != Token::LBrace { return self.error("Expected '{' after class name".to_string()); }
        self.advance();
        
        let mut fields = BTreeMap::new();
        let mut methods = std::collections::HashSet::new();
        let mut offset = 0;
        
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if self.current_token == Token::SemiColon {
                self.advance();
                continue;
            }
            if self.current_token == Token::Func {
                let m_name = self.parse_func(out, &name)?;
                methods.insert(m_name);
            } else if self.current_token == Token::Var {
                self.advance();
                let field_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected field name".to_string())
                };
                self.advance();
                self.current_class_fields.insert(field_name.clone(), offset);
                fields.insert(field_name, offset);
                offset += 1; 
                if self.current_token == Token::Colon {
                    self.advance();
                    self.advance(); 
                }
                if self.current_token == Token::SemiColon { self.advance(); }
            } else {
                return self.error(format!("Only functions/fields allowed in classes for now, got {:?}", self.current_token));
            }
        }
        self.classes.insert(name.clone(), ClassInfo { fields, size: offset });
        
        if self.current_token != Token::RBrace { return self.error("Expected '}'".to_string()); }
        self.advance();
        
        self.current_class_name = None;
        self.current_class_fields.clear();
        
        for tr in implemented_traits {
            if let Some(trait_methods) = self.traits.get(&tr) {
                for m in trait_methods {
                    if !methods.contains(m) {
                        return self.error(format!("Class '{}' missing trait method '{}::{}'", name, tr, m));
                    }
                }
            } else {
                return self.error(format!("Unknown trait '{}' implemented by class '{}'", tr, name));
            }
        }
        
        Ok(())
    }

    fn parse_bound_type_decl(&mut self) -> Result<(), CompileError> {
        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected type name".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); }
        self.advance();
        
        let mut start_val = i64::MIN;
        let mut end_val = i64::MAX;
        
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            let key = match &self.current_token {
                Token::Identifier(s) => s.clone(),
                _ => return self.error("Expected 'start' or 'end'".to_string()),
            };
            self.advance();
            if self.current_token != Token::Eq { return self.error("Expected =".to_string()); }
            self.advance();
            
            let val = match &self.current_token {
                Token::Number(n) => *n,
                Token::Minus => {
                    self.advance();
                    match &self.current_token {
                         Token::Number(n) => -n,
                         _ => return self.error("Expected number".to_string()),
                    }
                },
                _ => return self.error("Expected constant for bound".to_string()),
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

    fn parse_func(&mut self, out: &mut String, class_prefix: &str) -> Result<String, CompileError> {
        self.advance();
        if self.current_token == Token::Var {
             self.advance();
             self.parse_bound_type_decl()?;
             return Ok("".to_string());
        }

        let name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected function name".to_string()),
        };
        self.advance();
        
        if self.current_token != Token::LParen { return self.error("Expected '('".to_string()); }
        self.advance();
        
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
        
        let full_name = if class_prefix.is_empty() {
             name.clone()
        } else {
             format!("{}_{}", class_prefix, name)
        };
        
        out.push_str(&format!("; Function {}\nJMP skip_{}\n{}:\n", full_name, full_name, full_name));
        
        self.enter_scope(); 
        
        if !class_prefix.is_empty() {
             args.insert(0, "self".to_string());
        }

        let num_args = args.len() as i64;
        self.local_offset = num_args; 
        
        for (i, arg) in args.iter().enumerate() {
             let offset = i as i64;
             let loc = VarLocation::Local(offset);
             if let Some(scope) = self.scopes.last_mut() {
                 scope.insert(arg.clone(), (loc, Type::Int));
             }
        }
        
        let mut body_asm = String::new();
        self.parse_block(&mut body_asm)?;
        self.exit_scope(); 
        
        out.push_str(&body_asm);
        out.push_str("PUSH 0\nRET\n");
        out.push_str(&format!("skip_{}:\n", full_name));
        
        Ok(name)
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
                 self.parse_print(out, true)?;
             },
             Token::Println => {
                 self.advance();
                 self.parse_print(out, false)?;
             },
             Token::SemiColon => {
                 self.advance();
                 return Ok(());
             },
             Token::Identifier(name) => {
                 let part1 = name.clone();
                 if part1 == "vbe_set_mode" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_VBE_SET_MODE\n");
                     return Ok(());
                 }
                 if part1 == "vbe_update" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_VBE_UPDATE\n");
                     return Ok(());
                 }
                 if part1 == "sleep" {
                     self.advance(); 
                     if self.current_token != Token::LParen { return self.error("Expected ( for sleep".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_SLEEP\n");
                     return Ok(());
                 }
                 
                  if part1 == "poke_ptr" {
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                      else if self.current_token == Token::SemiColon { self.advance(); }
                      out.push_str("OP_POKE_PTR\n");
                      return Ok(());
                  }

                 if part1 == "peek_ptr" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_PEEK_PTR\n");
                     return Ok(());
                 }

                 if part1 == "syscall" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_SYSCALL\n");
                     return Ok(());
                 }
                 
                 if part1 == "dm_get" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_DM_GET\n");
                     return Ok(());
                 }
                 if part1 == "dm_set" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_DM_SET\n");
                     return Ok(());
                 }
                 
                 if part1 == "sec_login" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_SEC_LOGIN\n");
                     return Ok(());
                 }
                 if part1 == "sec_whoami" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_SEC_WHOAMI\n");
                     return Ok(());
                 }

                 self.advance(); 
                 if self.current_token == Token::Eq {
                        match self.resolve_var(&part1) {
                            Some((loc, _typ)) => {
                                self.advance(); 
                                self.parse_expression(out)?;
                                if expect_semi && self.current_token != Token::SemiColon {
                                    return self.error("Expected ;".to_string());
                                } else if self.current_token == Token::SemiColon { self.advance(); }
                                match loc {
                                    VarLocation::Global(addr) => { out.push_str(&format!("PUSH {}\nSWAP\nPOKE\n", addr)); },
                                    VarLocation::Local(offset) => { out.push_str(&format!("SET_LOCAL {}\n", offset)); }
                                }
                            },
                           None => {
                               if self.scopes.len() == 1 {
                                   self.advance(); 
                                   self.parse_expression(out)?;
                                   if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                                   else if self.current_token == Token::SemiColon { self.advance(); }
                                   let loc = self.declare_var(part1.clone(), Type::Int);
                                   if let VarLocation::Global(addr) = loc {
                                       out.push_str(&format!("PUSH {}\nSWAP\nPOKE\n", addr));
                                   }
                               } else {
                                   return self.error(format!("Undefined variable '{}' (use 'var {}')", part1, part1));
                               }
                           }
                       }
                 } else if self.current_token == Token::LParen {
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
                      if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                      else if self.current_token == Token::SemiColon { self.advance(); }
                      out.push_str(&format!("CALL {} {}\nPOP\n", part1, arg_count));
                 } else if self.current_token == Token::Dot {
                      self.advance();
                      let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                      self.advance();
                      if self.current_token == Token::Eq {
                          let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                          let offset = if let Type::Class(cname) = typ {
                              if let Some(cinfo) = self.classes.get(&cname) { *cinfo.fields.get(&member).unwrap() } 
                              else { return self.error(format!("Unknown class '{}'", cname)); }
                          } else {
                             if let Some(off) = self.current_class_fields.get(&member) {
                                 *off
                             } else {
                                 let mut found = None;
                                 for (cname, cinfo) in &self.classes {
                                     if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); }
                                 }
                                 if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                             }
                          };
                          match loc {
                              VarLocation::Global(addr) => { out.push_str(&format!("PUSH {}\nPEEK\n", addr)); },
                              VarLocation::Local(idx) => { out.push_str(&format!("OP_GET_LOCAL {}\n\n", idx)); }
                          }
                          out.push_str(&format!("PUSH {}\nOP_ADD\n", offset));
                          self.advance(); 
                          self.parse_expression(out)?;
                          if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                          else if self.current_token == Token::SemiColon { self.advance(); }
                          out.push_str("POKE\n");
                      } else if self.current_token == Token::LParen {
                           let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                           let cname = if let Type::Class(n) = typ { 
                               n 
                           } else {
                               if let Some(ref cn) = self.current_class_name {
                                   cn.clone()
                               } else {
                                   return self.error(format!("Variable '{}' is not an object", part1));
                               }
                           };

                           match loc {
                               VarLocation::Global(addr) => { out.push_str(&format!("PUSH {}\nPEEK\n", addr)); },
                               VarLocation::Local(idx) => { out.push_str(&format!("OP_GET_LOCAL {}\n", idx)); }
                           }

                           self.advance();
                           let mut arg_count = 1; 
                           if self.current_token != Token::RParen {
                                loop {
                                    self.parse_expression(out)?; arg_count += 1; 
                                    if self.current_token == Token::Comma { self.advance(); } else { break; }
                                }
                           }
                           self.advance(); 
                           if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                           else if self.current_token == Token::SemiColon { self.advance(); }
                           out.push_str(&format!("CALL {}_{} {}\nPOP\n", cname, member, arg_count));
                      } else {
                           return self.error("Expected = or ( after member name".to_string());
                      }
                 } else {
                       if let Token::Identifier(ref s) = self.current_token {
                           let lower = part1.to_lowercase();
                           match lower.as_str() {
                               "string" | "int" | "float" | "char" | "byte" | "short" | "long" => {
                                   return self.error(format!("Unexpected identifier '{}' after '{}'. Note: Nux types are lowercase (e.g. '{}'). Did you mean to declare a variable?", s, part1, lower));
                               },
                               _ => return self.error(format!("Unexpected identifier '{}' after '{}'.", s, part1))
                           }
                       }
                       return self.error(self.format_unexpected_token(&self.current_token, "Unexpected token in statement:"));
                 }
             },
             Token::Input => {
                self.advance();
                if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                self.advance();
                if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                self.advance();
                if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                else if self.current_token == Token::SemiColon { self.advance(); }
                out.push_str("INPUT\n");
             },
              Token::Var | Token::Let | Token::Const => { self.parse_var_decl(out, Type::Unknown)?; },
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
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  self.parse_expression(out)?;
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  let label_id = self.label_id_counter; self.label_id_counter += 1;
                  let label_else = format!("__if_else_{}", label_id);
                  let label_end = format!("__if_end_{}", label_id);
                  out.push_str(&format!("PUSH 0\nJE {}\n", label_else));
                  self.parse_block(out)?;
                  out.push_str(&format!("JMP {}\n{}:\n", label_end, label_else));
                  if self.current_token == Token::Else {
                      self.advance();
                      if self.current_token == Token::If { self.parse_statement_or_expr(out)?; } 
                      else { self.parse_block(out)?; }
                  }
                  out.push_str(&format!("{}:\n", label_end));
             },
             Token::While => {
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  let label_id = self.label_id_counter; self.label_id_counter += 1;
                  let label_start = format!("__while_start_{}", label_id);
                  let label_end = format!("__while_end_{}", label_id);
                  self.loop_stack.push((label_start.clone(), label_end.clone())); 
                  out.push_str(&format!("{}:\n", label_start));
                  self.parse_expression(out)?;
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  out.push_str(&format!("PUSH 0\nJE {}\n", label_end));
                  self.parse_block(out)?;
                  out.push_str(&format!("JMP {}\n{}:\n", label_start, label_end));
                  self.loop_stack.pop();
             },
             Token::For => {
                  self.advance();
                  if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                  self.advance();
                  self.parse_statement_or_expr(out)?; 
                  let label_id = self.label_id_counter; self.label_id_counter += 1;
                  let label_start = format!("__for_start_{}", label_id);
                  let label_step = format!("__for_step_{}", label_id);
                  let label_end = format!("__for_end_{}", label_id);
                  self.loop_stack.push((label_step.clone(), label_end.clone())); 
                  out.push_str(&format!("{}:\n", label_start));
                  if self.current_token != Token::SemiColon {
                       self.parse_expression(out)?;
                       out.push_str(&format!("PUSH 0\nJE {}\n", label_end));
                  }
                  if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                  self.advance();
                  out.push_str(&format!("JMP __for_body_{}\n{}:\n", label_id, label_step));
                  let mut step_out = String::new();
                  if self.current_token != Token::RParen {
                       self.parse_statement_impl(&mut step_out, false)?;
                  }
                  out.push_str(&step_out);
                  out.push_str(&format!("JMP {}\n", label_start));
                  if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                  self.advance();
                  out.push_str(&format!("__for_body_{}:\n", label_id));
                  self.parse_block(out)?;
                  out.push_str(&format!("JMP {}\n{}:\n", label_step, label_end));
                  self.loop_stack.pop();
             },
             Token::Do => {
                 self.advance();
                 let label_id = self.label_id_counter; self.label_id_counter += 1;
                 let label_start = format!("__do_start_{}", label_id);
                 let label_end = format!("__do_end_{}", label_id);
                 let label_cond = format!("__do_cond_{}", label_id);
                 self.loop_stack.push((label_cond.clone(), label_end.clone())); 
                 out.push_str(&format!("{}:\n", label_start));
                 self.parse_block(out)?;
                 out.push_str(&format!("{}:\n", label_cond));
                 if self.current_token != Token::While { return self.error("Expected while".to_string()); }
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                 self.advance();
                 out.push_str(&format!("PUSH 1\nJE {}\n{}:\n", label_start, label_end));
                 self.loop_stack.pop();
             },
             Token::LBrace => { self.parse_block(out)?; },
             Token::Unsafe => {
                 self.advance();
                 if self.current_token != Token::LBrace { return self.error("Expected { after unsafe".to_string()); }
                 self.advance();
                 out.push_str("OP_UNSAFE_START\n");
                 while self.current_token != Token::RBrace && self.current_token != Token::EOF {
                     self.parse_statement_impl(out, true)?;
                 }
                 if self.current_token == Token::RBrace { self.advance(); }
                 out.push_str("OP_UNSAFE_END\n");
             },
             Token::Peek => {
                 self.parse_expression(out)?; 
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Peek32 => {
                 self.parse_expression(out)?; 
                 if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Poke => { self.parse_poke(out)?; },
             Token::Poke32 => { self.parse_poke32(out)?; },
             Token::Break => {
                 match self.loop_stack.last() { Some(label) => out.push_str(&format!("JMP {}\n", label.1)), None => return self.error("Break outside loop".to_string()) }
                 self.advance(); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Continue => {
                 match self.loop_stack.last() { Some(label) => out.push_str(&format!("JMP {}\n", label.0)), None => return self.error("Continue outside loop".to_string()) }
                 self.advance(); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Asm => {
                 self.advance(); if self.current_token != Token::LBrace { return self.error("Expected {".to_string()); } self.advance();
                 while self.current_token != Token::RBrace && self.current_token != Token::EOF {
                     if let Token::String(s) = &self.current_token { 
                         out.push_str(s); out.push('\n'); self.advance(); 
                     } else if let Token::Identifier(name) = &self.current_token {
                     if let Some((loc, _)) = self.resolve_var(name) {
                          match loc {
                              VarLocation::Local(idx) => {
                                  out.push_str(&format!("OP_GET_LOCAL {}\n", idx));
                              },
                              VarLocation::Global(addr) => {
                                  out.push_str(&format!("PUSH {}\nPEEK\n", addr));
                              }
                          }
                     } else {
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
                 self.advance();
             },
             Token::Spawn => {
                  self.advance();
                  match &self.current_token {
                      Token::Identifier(func_name) => { out.push_str(&format!("PUSH {}\nSPAWN\n", func_name)); },
                      _ => return self.error("Expected function name".to_string()),
                  }
                  self.advance(); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::Lock | Token::Unlock => {
                 self.advance(); if self.current_token == Token::LParen { self.advance(); } if let Token::Identifier(_) = self.current_token { self.advance(); } if self.current_token == Token::RParen { self.advance(); } if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::CamCapture => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_CAM_CAPTURE\nPOP\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgDraw => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance(); self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_IMG_DRAW\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFree => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_IMG_FREE\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFilter => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_IMG_FILTER\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgSet => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_IMG_SET\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgFill => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
                 self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("OP_IMG_FILL\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             Token::ImgGet => {
                  self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                  self.parse_expression(out)?; self.advance(); self.parse_expression(out)?; self.advance(); self.parse_expression(out)?; self.advance();
                  out.push_str("OP_IMG_GET\nPOP\n"); if self.current_token == Token::SemiColon { self.advance(); }
             },
             _ => { return self.error(self.format_unexpected_token(&self.current_token, "Unexpected token in statement:")); }
        }
        Ok(())
    }

    fn parse_var_decl(&mut self, out: &mut String, expected_type: Type) -> Result<(), CompileError> {
        self.advance();
        let name = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected variable name".to_string()) };
        self.advance();
        let mut constraint = None;
        if self.current_token == Token::Colon {
             self.advance(); 
             match &self.current_token { Token::Identifier(s) => if let Some(bounds) = self.bound_types.get(s) { constraint = Some(*bounds); }, _ => {} }
             self.advance(); 
        }
        let mut final_type = expected_type.clone();
        if self.current_token == Token::Eq {
             self.advance();
             let expr_type = self.parse_expression(out)?;
             if let Some((min, max)) = constraint { out.push_str(&format!("OP_CHECK_RANGE\n{}\n{}\n", min, max)); }
             if final_type == Type::Unknown || final_type == Type::Void { final_type = expr_type.clone(); }
             if final_type == Type::Float && expr_type == Type::Int { out.push_str("ITOF\n"); }
             else if final_type == Type::Int && expr_type == Type::Float { out.push_str("FTOI\n"); }
        } else {
            out.push_str("PUSH 0\n");
        }
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
        self.advance();
        let loc = self.declare_var(name, final_type);
        if let VarLocation::Global(addr) = loc { out.push_str(&format!("PUSH {}\nSWAP\nPOKE\n", addr)); }
        Ok(())
    }

    fn parse_print(&mut self, out: &mut String, newline: bool) -> Result<(), CompileError> {
        if self.current_token != Token::LParen { return self.error("Expected ( for print".to_string()); }
        self.advance();
        if self.current_token == Token::RParen { self.advance(); if newline { out.push_str("PUSH 10\nPRINT_CHAR\n"); } } 
        else {
             if let Token::String(s) = &self.current_token {
                for c in s.chars() { out.push_str(&format!("PUSH {}\nPRINT_CHAR\n", c as u32)); }
                self.advance();
             } else {
                let t = self.parse_expression(out)?;
                if t == Type::Float { out.push_str("PRINT_FLOAT\n"); } 
                else if t == Type::Char { out.push_str("PRINT_CHAR\n"); }
                else { out.push_str("PRINT_VAL\n"); }
             }
             if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
             self.advance();
             if newline { out.push_str("PUSH 10\nPRINT_CHAR\n"); }
        }
         if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
         self.advance();
         Ok(())
    }

    fn parse_poke(&mut self, out: &mut String) -> Result<(), CompileError> {
        self.advance();
        if self.current_token != Token::LParen { return self.error("Expected ( for poke".to_string()); } self.advance();
        self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
        self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); } self.advance();
        out.push_str("POKE\n");
        Ok(())
    }

    fn parse_poke32(&mut self, out: &mut String) -> Result<(), CompileError> {
        self.advance();
        if self.current_token != Token::LParen { return self.error("Expected ( for poke32".to_string()); } self.advance();
        self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); } self.advance();
        self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
        if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); } self.advance();
        out.push_str("OP_POKE32\n");
        Ok(())
    }
    
    fn parse_expression(&mut self, out: &mut String) -> Result<Type, CompileError> {
        self.parse_logical_or(out)
    }

    fn parse_logical_or(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_logical_xor(out)?;
        while self.current_token == Token::Or {
            self.advance();
            let _ = self.parse_logical_xor(out)?; out.push_str("OR\n"); left_type = Type::Bool;
        }
        Ok(left_type)
    }

    fn parse_logical_xor(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_logical_and(out)?;
        while self.current_token == Token::Xor || self.current_token == Token::Xand {
            let op = self.current_token.clone();
            self.advance();
            let _ = self.parse_logical_and(out)?;
            match op {
                Token::Xor => out.push_str("XOR\n"),
                Token::Xand => out.push_str("XAND\n"),
                _ => {}
            }
            // logical/bitwise result type
            if left_type == Type::Bool { left_type = Type::Bool; } 
            else { left_type = Type::Int; }
        }
        Ok(left_type)
    }

    fn parse_logical_and(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_equality(out)?;
        while self.current_token == Token::And {
            self.advance();
            let _ = self.parse_equality(out)?; out.push_str("AND\n"); left_type = Type::Bool; 
        }
        Ok(left_type)
    }
    
    fn parse_equality(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_comparison(out)?;
        while self.current_token == Token::EqEq || self.current_token == Token::NotEq {
            let op = self.current_token.clone(); self.advance();
            let _ = self.parse_comparison(out)?;
            match op { Token::EqEq => out.push_str("EQ\n"), Token::NotEq => out.push_str("NEQ\n"), _ => {} }
            left_type = Type::Bool;
        }
        Ok(left_type)
    }

    fn parse_comparison(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_term(out)?;
        while matches!(self.current_token, Token::Lt | Token::Gt | Token::LtEq | Token::GtEq) {
            let op = self.current_token.clone(); self.advance();
            let _ = self.parse_term(out)?;
            match op { Token::Lt => out.push_str("LT\n"), Token::Gt => out.push_str("GT\n"), Token::LtEq => out.push_str("LTE\n"), Token::GtEq => out.push_str("GTE\n"), _ => {} }
            left_type = Type::Bool;
        }
        Ok(left_type)
    }

    fn parse_term(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_factor(out)?;
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone(); self.advance();
            let mut right_out = String::new();
            let right_type = self.parse_factor(&mut right_out)?;
            if left_type == Type::Float || right_type == Type::Float {
                if left_type != Type::Float { out.push_str("ITOF\n"); }
                out.push_str(&right_out);
                if right_type != Type::Float { out.push_str("ITOF\n"); }
                match op { Token::Plus => out.push_str("FADD\n"), Token::Minus => out.push_str("FSUB\n"), _ => {} }
                left_type = Type::Float;
            } else {
                out.push_str(&right_out);
                match op { Token::Plus => out.push_str("ADD\n"), Token::Minus => out.push_str("SUB\n"), _ => {} }
            }
        }
        Ok(left_type)
    }

    fn parse_factor(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_power(out)?;
        while matches!(self.current_token, Token::Star | Token::Slash | Token::SlashSlash | Token::Percent) {
            let op = self.current_token.clone(); self.advance();
            let mut right_out = String::new();
            let right_type = self.parse_power(&mut right_out)?;
            if left_type == Type::Float || right_type == Type::Float {
                 if left_type != Type::Float { out.push_str("ITOF\n"); }
                 out.push_str(&right_out);
                 if right_type != Type::Float { out.push_str("ITOF\n"); }
                 match op {
                     Token::Star => out.push_str("FMUL\n"), Token::Slash => out.push_str("FDIV\n"), Token::SlashSlash => out.push_str("FFLOORDIV\n"),
                     Token::Percent => return self.error("Modulo not supported for floats".to_string()), _ => {} 
                 }
                 left_type = Type::Float;
            } else {
                out.push_str(&right_out);
                match op {
                    Token::Star => out.push_str("MUL\n"), Token::Slash => out.push_str("DIV\n"),
                    Token::SlashSlash => out.push_str("FLOORDIV\n"), Token::Percent => out.push_str("MOD\n"), _ => {}
                }
            }
        }
        Ok(left_type)
    }
    
    fn parse_power(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut left_type = self.parse_unary(out)?;
        if self.current_token == Token::StarStar {
            self.advance();
            let mut right_out = String::new();
            let right_type = self.parse_power(&mut right_out)?; 
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
                let neg_one: f64 = -1.0;
                let bits = neg_one.to_bits() as i64;
                out.push_str(&format!("PUSH {}\nFMUL\n", bits));
                return Ok(Type::Float);
            } else {
                out.push_str("PUSH 0\nSWAP\nSUB\n");
                return Ok(operand_type);
            }
        }
        if self.current_token == Token::Not {
            self.advance(); let _ = self.parse_unary(out)?; out.push_str("PUSH 0\nEQ\n"); return Ok(Type::Bool);
        }
        if self.current_token == Token::Xnot {
            self.advance(); 
            let t = self.parse_unary(out)?; 
            out.push_str("XNOT\n"); 
            return Ok(t);
        }
        self.parse_primary(out)
    }

    fn parse_primary(&mut self, out: &mut String) -> Result<Type, CompileError> {
        match &self.current_token {
            Token::New => {
                self.advance();
                let name = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected class name".to_string()) };
                self.advance();
                let size = if let Some(info) = self.classes.get(&name) { info.size as i32 } else { return self.error(format!("Undefined class '{}'", name)); };
                out.push_str(&format!("PUSH {}\nPUSH 1\nOP_IMG_ALLOC\n", size));
                if self.current_token == Token::LParen { 
                     self.advance();
                     if self.current_token != Token::RParen { loop { self.parse_expression(out)?; if self.current_token==Token::Comma{self.advance();}else{break;} } }
                     self.advance();
                }
                Ok(Type::Class(name)) 
            },
            Token::Input => {
                 self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); } self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                 out.push_str("INPUT\n"); Ok(Type::Int)
            },
            Token::KwInt => {
                 self.advance(); if self.current_token == Token::LParen { self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error("Expected )".to_string());} self.advance(); out.push_str("FTOI\n"); Ok(Type::Int) } else { return self.error("Expected (".to_string()); }
            },
            Token::KwFloat => {
                 self.advance(); if self.current_token == Token::LParen { self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error("Expected )".to_string());} self.advance(); out.push_str("ITOF\n"); Ok(Type::Float) } else { return self.error("Expected (".to_string()); }
            },
            Token::SysPlatform => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_SYS_PLATFORM\n"); Ok(Type::Int) },
            Token::CamCount => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_CAM_COUNT\n"); Ok(Type::Int) },
            Token::IsKeyDown => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IS_KEY_DOWN\n"); Ok(Type::Bool) },
            Token::Sin => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FSIN\n"); Ok(Type::Float) },
            Token::Cos => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FCOS\n"); Ok(Type::Float) },
            Token::Sqrt => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FSQRT\n"); Ok(Type::Float) },
            Token::CamCapture => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_CAM_CAPTURE\n"); Ok(Type::Int) },
            Token::ImgAlloc => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IMG_ALLOC\n"); Ok(Type::Int) },
            Token::ImgResize => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IMG_RESIZE\n"); Ok(Type::Int) },
            Token::ImgCrop => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IMG_CROP\n"); Ok(Type::Int) },
            Token::ImgGrayscale => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IMG_GRAYSCALE\n"); Ok(Type::Int) },
            Token::UpperCase => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_TO_UPPER\n"); Ok(Type::Int) },
            Token::LowerCase => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_TO_LOWER\n"); Ok(Type::Int) },
            Token::ImgGet => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_IMG_GET\n"); Ok(Type::Int) },
            Token::Number(n) => { let val = *n; self.advance(); out.push_str(&format!("PUSH {}\n", val)); Ok(Type::Int) },
            Token::Float(f) => { let val = *f; self.advance(); let bits = val.to_bits() as i64; out.push_str(&format!("PUSH {}\n", bits)); Ok(Type::Float) },
            Token::True => { self.advance(); out.push_str("PUSH 1\n"); Ok(Type::Bool) },
            Token::False => { self.advance(); out.push_str("PUSH 0\n"); Ok(Type::Bool) },
            Token::String(s) => { 
                let string_val = s.clone();
                self.advance(); 
                // Emit OP_PUSH_STR with length-prefixed string data
                out.push_str("OP_PUSH_STR\n");
                out.push_str(&format!("PUSH {}\n", string_val.len()));
                // Emit string bytes as individual BYTE instructions
                for byte in string_val.bytes() {
                    out.push_str(&format!("BYTE {}\n", byte));
                }
                Ok(Type::String) 
            },
            Token::Identifier(name) => {
                let part1 = name.clone(); self.advance();
                
                // Intrinsics
                if part1 == "sec_login" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_SEC_LOGIN\n");
                    return Ok(Type::Int);
                } else if part1 == "syscall" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_SYSCALL\n");
                    return Ok(Type::Int);
                } else if part1 == "peek_ptr" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_PEEK_PTR\n");
                    return Ok(Type::Int);
                } else if part1 == "sec_whoami" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_SEC_WHOAMI\n");
                    return Ok(Type::String);
                } else if part1 == "dm_get" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_DM_GET\n");
                    return Ok(Type::String);
                } else if part1 == "vbe_get_fb" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_VBE_GET_FB\n");
                    return Ok(Type::Int);
                } else if part1 == "vbe_get_key" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_VBE_GET_KEY\n");
                    return Ok(Type::Int);
                } else if part1 == "vbe_mouse_x" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_VBE_GET_MOUSE_X\n");
                    return Ok(Type::Int);
                } else if part1 == "vbe_mouse_y" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_VBE_GET_MOUSE_Y\n");
                    return Ok(Type::Int);
                } else if part1 == "vbe_mouse_down" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_VBE_GET_MOUSE_DOWN\n");
                    return Ok(Type::Int);
                } else if part1 == "dm_set" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_DM_SET\n");
                    return Ok(Type::Int);
                } else if self.current_token == Token::LParen {
                    self.advance(); let mut arg_count = 0;
                    if self.current_token != Token::RParen { loop { self.parse_expression(out)?; arg_count += 1; if self.current_token == Token::Comma { self.advance(); } else { break; } } }
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); } self.advance();
                    out.push_str(&format!("CALL {} {}\n", part1, arg_count)); Ok(Type::Int)
                } else {
                    let (loc, mut typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                    match loc { VarLocation::Global(addr) => { out.push_str(&format!("PUSH {}\nPEEK\n", addr)); }, VarLocation::Local(idx) => { out.push_str(&format!("OP_GET_LOCAL {}\n\n", idx)); } }
                    while self.current_token == Token::Dot {
                        self.advance();
                        let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                        self.advance();
                        if self.current_token == Token::LParen {
                             let cname = if let Type::Class(n) = &typ { n.clone() } else { 
                                 if let Some(ref cn) = self.current_class_name { cn.clone() } else { return self.error(format!("Variable is not an object")); }
                             };
                             self.advance();
                             let mut arg_count = 1;
                             if self.current_token != Token::RParen {
                                  loop {
                                      self.parse_expression(out)?; arg_count += 1;
                                      if self.current_token == Token::Comma { self.advance(); } else { break; }
                                  }
                             }
                             if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                             self.advance();
                             out.push_str(&format!("CALL {}_{} {}\n", cname, member, arg_count));
                             typ = Type::Int;
                        } else {
                            let offset = if let Type::Class(cname) = &typ {
                                 if let Some(cinfo) = self.classes.get(cname) { 
                                     if let Some(f) = cinfo.fields.get(&member) { *f } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                 } else { return self.error(format!("Unknown class '{}'", cname)); }
                            } else {
                                 if let Some(off) = self.current_class_fields.get(&member) {
                                     *off
                                 } else {
                                     let mut found = None; for (cname, cinfo) in &self.classes { if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); } }
                                     if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                                 }
                            };
                            out.push_str(&format!("PUSH {}\nOP_ADD\nPEEK\n", offset)); typ = Type::Unknown;
                        }
                    }
                    Ok(typ)
                }
            },
            Token::LParen => { self.advance(); let t = self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); Ok(t) },
            Token::Peek => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("PEEK\n"); Ok(Type::Int) },
            Token::Peek32 => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_PEEK32\n"); Ok(Type::Int) },
            _ => return self.error(self.format_unexpected_token(&self.current_token, "Unexpected token in expression:")),
        }
    }
}
