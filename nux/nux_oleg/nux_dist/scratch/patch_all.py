import re

# PATCH LEXER
with open('src/lexer.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Add to Token enum properly
code = code.replace('    Import,\n', '    Import,\n    From,\n    Use,\n    InlineLang(String),\n')
code = code.replace('"import" => Token::Import,', '"import" => Token::Import,\n                            "from" => Token::From,\n                            "use" => Token::Use,')

with open('src/lexer.rs', 'w', encoding='utf-8') as f:
    f.write(code)

# PATCH COMPILER
with open('src/compiler.rs', 'r', encoding='utf-8') as f:
    code = f.read()

import_block_start = code.find('Token::Import => {')
import_block_end = code.find('Token::SemiColon => self.advance(),', import_block_start)

if import_block_start != -1 and import_block_end != -1:
    new_import_block = """Token::Use | Token::Import => {
                    self.advance();
                    loop {
                        let raw_name = match &self.current_token {
                            Token::String(s) => s.clone(),
                            _ => {
                                self.errors.push(CompileError::new("Expected import path string".to_string(), self.current_span));
                                break;
                            }
                        };
                        self.advance();
                        
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

                        if self.current_token == Token::Comma {
                            self.advance();
                            continue;
                        } else {
                            break;
                        }
                    }
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
                Token::From => {
                    self.advance();
                    let raw_name = match &self.current_token {
                        Token::String(s) => s.clone(),
                        _ => {
                            self.errors.push(CompileError::new("Expected module path string".to_string(), self.current_span));
                            break; 
                        }
                    };
                    self.advance();
                    if self.current_token == Token::Import {
                        self.advance();
                    } else {
                        self.errors.push(CompileError::new("Expected 'import' after 'from ...'".to_string(), self.current_span));
                    }
                    
                    // Parse comma separated identifiers
                    loop {
                        match &self.current_token {
                            Token::Identifier(_) => {
                                self.advance();
                            },
                            _ => { break; }
                        }
                        if self.current_token == Token::Comma {
                            self.advance();
                            continue;
                        } else {
                            break;
                        }
                    }
                    
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
                        self.errors.push(CompileError::new(format!("Import not found: {}", file_name), self.prev_span));
                    }
                    
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
"""
    code = code[:import_block_start] + new_import_block + code[import_block_end:]
    with open('src/compiler.rs', 'w', encoding='utf-8') as f:
        f.write(code)
    print('Compiler imports patched.')
else:
    print('Could not find import block.')
