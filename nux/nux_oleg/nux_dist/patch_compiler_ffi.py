import re

with open("src/compiler.rs", "r") as f:
    content = f.read()

# We need to insert ffi_python and ffi_c into parse_statement_impl and parse_primary

stmt_addition = """
                 if part1 == "ffi_python" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     out.push_str("OP_FFI_PYTHON\\n");
                     return Ok(());
                 }
                 if part1 == "ffi_c" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     out.push_str("OP_FFI_C\\n");
                     return Ok(());
                 }
"""

primary_addition = """
                  } else if part1 == "ffi_python" {
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_FFI_PYTHON\\n");
                      return Ok(Type::Int);
                  } else if part1 == "ffi_c" {
                      self.advance();
                      if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                      self.advance();
                      self.parse_expression(out)?;
                      if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                      self.advance();
                      out.push_str("OP_FFI_C\\n");
                      return Ok(Type::Int);
"""

# Insert into parse_statement_impl near ffi_load or q_alloc
stmt_search = """                 if part1 == "q_alloc" {"""
if stmt_search in content:
    content = content.replace(stmt_search, stmt_addition + stmt_search)
else:
    print("Could not find q_alloc in parse_statement_impl")

# Insert into parse_primary near ffi_load
primary_search = """                  } else if part1 == "ffi_load" {"""
if primary_search in content:
    content = content.replace(primary_search, primary_addition + primary_search)
else:
    print("Could not find ffi_load in parse_primary")

with open("src/compiler.rs", "w") as f:
    f.write(content)

print("compiler.rs patched successfully")
