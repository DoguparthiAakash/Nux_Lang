import re

with open('src/compiler.rs', 'r') as f:
    content = f.read()

# Fix 1: Add LBracket to parse_primary
array_literal_code = """            Token::Peek32 => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_PEEK32\\n"); Ok(Type::Int) },
            Token::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.current_token != Token::RBracket {
                    loop {
                        let mut elem_code = String::new();
                        let typ = self.parse_expression(&mut elem_code)?;
                        elements.push((elem_code, typ));
                        if self.current_token == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                if self.current_token != Token::RBracket { return self.error("Expected ']'".to_string()); }
                self.advance();
                
                out.push_str(&format!("PUSH {}\\nOP_TENSOR_NEW\\n", elements.len()));
                for (i, (code, typ)) in elements.iter().enumerate() {
                    out.push_str("DUP\\n");
                    out.push_str(&format!("PUSH {}\\n", i));
                    out.push_str(code);
                    if *typ == Type::Int {
                        out.push_str("ITOF\\n");
                    }
                    out.push_str("OP_TENSOR_SET\\n");
                }
                Ok(Type::Int)
            },"""

content = content.replace(
    """            Token::Peek32 => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_PEEK32\\n"); Ok(Type::Int) },""",
    array_literal_code
)

# Fix 2: cux_call in statements
cux_call_stmt = """                   if part1 == "cux_call" {
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

content = re.sub(
    r'                   if part1 == "cux_call" \{.*?out\.push_str\(&format!\("PUSH \{\}\\n", arg_count\)\);\s+out\.push_str\("OP_CUX_CALL\\n"\);\s+out\.push_str\("POP\\n"\);\s+return Ok\(\(\)\);\s+\}',
    cux_call_stmt,
    content,
    flags=re.DOTALL
)

# Fix 3: cux_call in expressions
cux_call_expr = """                  } else if part1 == "cux_call" {
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

content = re.sub(
    r'                  \} else if part1 == "cux_call" \{.*?out\.push_str\(&format!\("PUSH \{\}\\n", arg_count\)\);\s+out\.push_str\("OP_CUX_CALL\\n"\);\s+return Ok\(Type::Int\);',
    cux_call_expr,
    content,
    flags=re.DOTALL
)

with open('src/compiler.rs', 'w') as f:
    f.write(content)
