import re

with open("src/compiler.rs", "r") as f:
    content = f.read()

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

# Find the exact ffi_load block in parse_statement_impl and insert before it
pattern_stmt = re.compile(r'(\s*if part1 == "ffi_load" \{)')
match_stmt = pattern_stmt.search(content)
if match_stmt:
    content = content[:match_stmt.start()] + stmt_addition + content[match_stmt.start():]
else:
    print("Could not find ffi_load in parse_statement_impl")

# Find the exact ffi_load block in parse_primary and insert before it
pattern_primary = re.compile(r'(\s*\} else if part1 == "ffi_load" \{)')
match_primary = pattern_primary.search(content)
if match_primary:
    content = content[:match_primary.start()] + primary_addition + content[match_primary.start():]
else:
    print("Could not find ffi_load in parse_primary")

with open("src/compiler.rs", "w") as f:
    f.write(content)

print("compiler.rs patched successfully")
