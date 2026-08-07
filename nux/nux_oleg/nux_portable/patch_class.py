import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target = '''          let mut fields = HashMap::new();
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
          self.classes.insert(name, ClassInfo { fields, size: offset });'''

repl = '''          let mut fields = HashMap::new();
          let mut offset = 0;
          
          // Pre-register class to allow method bodies to reference fields
          self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: offset });
          
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
                  
                  // Update class registration incrementally
                  self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: offset });
                  
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
          }'''

code = code.replace(target, repl)
code = code.replace(target.replace('\n', '\r\n'), repl)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched parse_class.")
