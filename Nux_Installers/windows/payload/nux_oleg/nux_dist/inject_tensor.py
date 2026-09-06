with open('src/compiler.rs', 'r') as f:
    content = f.read()

tensor_intrinsics = """                  } else if part1 == "tensor_new" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_NEW\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_free" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_FREE\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_matmul" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
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
                      out.push_str("OP_TENSOR_MATMUL\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_relu" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_RELU\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_softmax" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_SOFTMAX\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_rmsnorm" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_RMSNORM\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_scale" {
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_TENSOR_SCALE\\n");
                      return Ok(Type::Int);
                  } else if part1 == "tensor_embedding" {
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
                      out.push_str("OP_TENSOR_EMBEDDING\\n");
                      return Ok(Type::Int);
                  } else if part1 == "cux_load" {"""

content = content.replace('                  } else if part1 == "cux_load" {', tensor_intrinsics)

tensor_stmts = """                   if part1 == "tensor_free" {
                       self.advance();
                       if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                       self.advance();
                       self.parse_expression(out)?;
                       if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                       self.advance();
                       if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                       else if self.current_token == Token::SemiColon { self.advance(); }
                       out.push_str("OP_TENSOR_FREE\\n");
                       return Ok(());
                   }
                   if part1 == "cux_call" {"""

content = content.replace('                   if part1 == "cux_call" {', tensor_stmts)

with open('src/compiler.rs', 'w') as f:
    f.write(content)
