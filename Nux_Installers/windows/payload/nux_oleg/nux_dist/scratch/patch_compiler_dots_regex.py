import sys
import re

def patch_compiler():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()

    # Find the block starting at `} else if self.current_token == Token::Dot {`
    # and ending at `} else {` before `if let Token::Identifier(ref s) = self.current_token {`
    
    start_str = '} else if self.current_token == Token::Dot {'
    end_str = '} else {\n                       if let Token::Identifier(ref s) = self.current_token {'
    
    start_idx = content.find(start_str)
    end_idx = content.find(end_str)
    
    if start_idx == -1 or end_idx == -1:
        print("Could not find start or end index!")
        return
        
    replace_str = r'''} else if self.current_token == Token::Dot {
                      let (loc, mut typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                      match loc {
                          VarLocation::Global(addr) => { out.push_str(&format!("PUSH {}\nPEEK\n", addr)); },
                          VarLocation::Local(idx) => { out.push_str(&format!("OP_GET_LOCAL {}\n", idx)); }
                      }

                      loop {
                          self.advance(); // consume Dot
                          let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                          self.advance();

                          if self.current_token == Token::Dot {
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
                              out.push_str(&format!("PUSH {}\nOP_ADD\nPEEK\n", offset));
                              typ = Type::Unknown;
                          } else if self.current_token == Token::Eq {
                              let offset = if let Type::Class(cname) = &typ {
                                  if let Some(cinfo) = self.classes.get(cname) { *cinfo.fields.get(&member).unwrap() } 
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
                              out.push_str(&format!("PUSH {}\nOP_ADD\n", offset));
                              self.advance(); 
                              self.parse_expression(out)?;
                              if expect_semi && self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                              else if self.current_token == Token::SemiColon { self.advance(); }
                              out.push_str("POKE\n");
                              break;
                          } else if self.current_token == Token::LParen {
                              let cname = if let Type::Class(n) = &typ { 
                                  n.clone() 
                              } else {
                                  if let Some(ref cn) = self.current_class_name {
                                      cn.clone()
                                  } else {
                                      let mut found = None;
                                      for (name, cinfo) in &self.classes {
                                          if cinfo.methods.contains_key(&member) { found = Some(name.clone()); }
                                      }
                                      if let Some(n) = found { n } else {
                                          return self.error(format!("Method '{}' not found", member));
                                      }
                                  }
                              };
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
                              break;
                          } else {
                              return self.error("Expected =, (, or . after member name".to_string());
                          }
                      }
'''
    
    new_content = content[:start_idx] + replace_str + content[end_idx:]
    with open(path, 'w') as f:
        f.write(new_content)
    print("Patched multiple dots support via index slice")

if __name__ == '__main__':
    patch_compiler()
