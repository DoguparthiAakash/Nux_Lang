with open('src/compiler.rs', 'r') as f:
    content = f.read()

stmt_injection = """                   if part1 == "syscall" {
                       self.advance();
                       if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                       self.advance();
                       self.parse_expression(out)?;
                       if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                       self.advance();
                       if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                       else if self.current_token == Token::SemiColon { self.advance(); }
                       out.push_str("OP_SYSCALL\\n");
                       out.push_str("POP\\n");
                       return Ok(());
                   }
                   if part1 == "cux_call" {
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
                       if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                       else if self.current_token == Token::SemiColon { self.advance(); }
                       let actual_args = if arg_count >= 2 { arg_count - 2 } else { 0 };
                       out.push_str(&format!("PUSH {}\\n", actual_args));
                       out.push_str("OP_CUX_CALL\\n");
                       out.push_str("POP\\n");
                       return Ok(());
                   }"""

content = content.replace(
    """                   if part1 == "syscall" {
                       self.advance();
                       if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                       self.advance();
                       self.parse_expression(out)?;
                       if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                       self.advance();
                       if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                       else if self.current_token == Token::SemiColon { self.advance(); }
                       out.push_str("OP_SYSCALL\\n");
                       out.push_str("POP\\n");
                       return Ok(());
                   }""",
    stmt_injection
)

expr_injection = """                  } else if part1 == "syscall" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_SYSCALL\\n");
                      return Ok(Type::Int);
                  } else if part1 == "cux_load" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_CUX_LOAD\\n");
                      return Ok(Type::Int);
                  } else if part1 == "cux_call" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      let mut arg_count = 0;
                      if self.current_token != Token::RParen {
                          loop {
                              self.parse_expression(out)?;
                              arg_count += 1;
                              if self.current_token == Token::Comma {
                                  self.advance();
                              } else {
                                  break;
                              }
                          }
                      }
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      let actual_args = if arg_count >= 2 { arg_count - 2 } else { 0 };
                      out.push_str(&format!("PUSH {}\\n", actual_args));
                      out.push_str("OP_CUX_CALL\\n");
                      return Ok(Type::Int);"""

content = content.replace(
    """                  } else if part1 == "syscall" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_SYSCALL\\n");
                      return Ok(Type::Int);""",
    expr_injection
)

with open('src/compiler.rs', 'w') as f:
    f.write(content)
