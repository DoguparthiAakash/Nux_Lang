import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target = '''                      // Handle Chain: .x.y.z
                      while self.current_token == Token::Dot {
                          self.advance();
                          let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                          self.advance();
                          
                          let offset = if let Type::Class(cname) = &typ {
                               if let Some(cinfo) = self.classes.get(cname) {
                                    if let Some(off) = cinfo.fields.get(&member) {
                                        *off
                                    } else { return self.error(format!("Class '{}' has no field '{}'", cname, member)); }
                               } else { eprintln!("DEBUG UNKNOWN CLASS: cname={:?} (len {}), keys={:?}", cname, cname.len(), self.classes.keys()); return self.error(format!("Unknown class '{}'", cname)); }
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
                          
                          out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset));
                          // Typ becomes Unknown unless we track field types
                          typ = Type::Unknown;
                      }'''

repl = '''                      // Handle Chain: .x.y.z
                      while self.current_token == Token::Dot {
                          self.advance();
                          let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                          self.advance();
                          
                          if self.current_token == Token::LParen {
                              // Method Call in expression
                              self.advance(); // Skip (
                              let mut arg_count = 1; // 'this' counts as 1 argument
                              if self.current_token != Token::RParen {
                                   loop {
                                       self.parse_expression_and_push(out)?;
                                       arg_count += 1;
                                       if self.current_token == Token::Comma { self.advance(); } else { break; }
                                   }
                              }
                              if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                              self.advance();
                              
                              let full_name = if let Type::Class(cname) = &typ {
                                  format!("{}_{}", cname, member)
                              } else {
                                  return self.error(format!("Cannot call method '{}' on variable '{}' of unknown type", member, part1));
                              };
                              
                              out.push_str(&format!("CALL {} {}\\n", full_name, arg_count));
                              typ = Type::Unknown; // Methods return unknown by default for now
                          } else {
                              // Field Access
                              let offset = if let Type::Class(cname) = &typ {
                                   if let Some(cinfo) = self.classes.get(cname) {
                                        if let Some(off) = cinfo.fields.get(&member) {
                                            *off
                                        } else { return self.error(format!("Class '{}' has no field '{}'", cname, member)); }
                                   } else { eprintln!("DEBUG UNKNOWN CLASS: cname={:?} (len {}), keys={:?}", cname, cname.len(), self.classes.keys()); return self.error(format!("Unknown class '{}'", cname)); }
                              } else {
                                   // Fallback: Search all classes for field
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
                                       return self.error(format!("Field '{}' not found in any class (variable '{}' type unknown)", member, part1));
                                   }
                              };
                              
                              out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset));
                              typ = Type::Unknown;
                          }
                      }'''

code = code.replace(target, repl)
code = code.replace(target.replace('\n', '\r\n'), repl)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched parse_primary method call logic.")
