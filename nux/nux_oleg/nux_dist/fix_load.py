with open('src/compiler.rs', 'r') as f:
    content = f.read()

missing_code = """                  } else if part1 == "cux_load" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_CUX_LOAD\\n");
                      return Ok(Type::Int);
                  } else if part1 == "cux_call" {"""

content = content.replace('                  } else if part1 == "cux_call" {', missing_code)

with open('src/compiler.rs', 'w') as f:
    f.write(content)
