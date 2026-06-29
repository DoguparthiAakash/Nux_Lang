import sys

with open('src/compiler.rs', 'r') as f:
    content = f.read()

idx = content.find('} else if part1 == "syscall" {')
if idx != -1:
    cux_code = """                  } else if part1 == "cux_load" {
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_CUX_LOAD\\n");
                      return Ok(Type::Int);
                  } else if part1 == "cux_call" {
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      // lib_id
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                      self.advance();
                      // func_name 
                      self.parse_expression(out)?;
                      
                      let mut arg_count = 0;
                      while self.current_token == Token::Comma {
                          self.advance();
                          self.parse_expression(out)?;
                          arg_count += 1;
                      }
                      
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str(&format!("PUSH {}\\n", arg_count));
                      out.push_str("OP_CUX_CALL\\n");
                      return Ok(Type::Int);
"""
    new_content = content[:idx] + cux_code + content[idx:]
    with open('src/compiler.rs', 'w') as f:
        f.write(new_content)
    print("Injected into parse_primary successfully!")
else:
    print("syscall not found!")
