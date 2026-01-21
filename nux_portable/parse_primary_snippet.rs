    fn parse_primary(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {
        match &self.current_token {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                // Return constant, do NOT emit PUSH
                Ok((Type::Int, Some(ConstantValue::Int(val))))
            },
            Token::String(s) => {
                let val = s.clone();
                self.advance();
                // Constant string
                // Strings are tricky because they are ptrs. 
                // But for folding, if we concat strings?
                // For now, treat as constant value but we likely need to emit them as literals if used.
                // Actually, if we return Some, parent *must* eventually emit.
                Ok((Type::String, Some(ConstantValue::String(val))))
            },
            Token::True => {
                self.advance();
                Ok((Type::Bool, Some(ConstantValue::Bool(true))))
            },
            Token::False => {
                self.advance();
                Ok((Type::Bool, Some(ConstantValue::Bool(false))))
            },
            Token::Identifier(name_str) => {
                let name = name_str.clone();
                self.advance();
                
                // Function Call?
                if self.current_token == Token::LParen {
                    self.parse_call_args(out, &name)?;
                     // Function return value is unknown at compile time
                    // Unless we fold pure functions? (Too advanced for now)
                    // Lookup return type if possible?
                    // For now assume Int or unknown.
                    Ok((Type::Int, None))
                } else {
                    // Variable access
                    // Check if local or global
                    // ... (Variable resolution logic)
                    // We need to emit LOAD
                    self.emit_load(out, &name)?;
                    Ok((Type::Int, None)) // Assume Int for var
                }
            },
            Token::LParen => {
                self.advance();
                let res = self.parse_expression(out)?;
                if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                self.advance();
                Ok(res) // Pass through constant if inner was constant
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
            
            // --- Casts ---
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

            // --- Introspection ---
            Token::SysPlatform => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_SYS_PLATFORM\n");
                 Ok((Type::Int, None))
            },
            
            _ => { return self.error(format!("Unexpected token in expression: {:?}", self.current_token)); }
        }
    }
            Token::CamCount => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_CAM_COUNT\n");
                 Ok(Type::Int)
            },
            Token::IsKeyDown => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_IS_KEY_DOWN\n");
                 Ok(Type::Bool)
            },
            Token::Sin => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FSIN\n");
                 Ok(Type::Float)
            },
            Token::Cos => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FCOS\n");
                 Ok(Type::Float)
            },
            Token::Sqrt => {
                 self.advance();
                 if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                 self.advance();
                 self.parse_expression(out)?;
                 if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                 self.advance();
                 out.push_str("OP_FSQRT\n");
                 Ok(Type::Float)
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
                } else {
                    // Variable Access
                    let (loc, mut typ) = match self.resolve_var(&part1) {
                        Some(r) => r,
                        None => return self.error(format!("Undefined variable '{}'", part1)),
                    };
                    
                    match loc {
                        VarLocation::Global(addr) => {
                             out.push_str(&format!("PUSH {}\nPEEK\n", addr)); 
                        },
                        VarLocation::Local(idx) => {
                             out.push_str(&format!("OP_GET_LOCAL {}\n\n", idx));
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
                    
                    Ok(typ)
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
