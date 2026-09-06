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
            Token::String(ref s) => {
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
                    // Emit CALL with arg count
                    out.push_str(&format!("CALL {} {}\n", part1, arg_count));
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
}
