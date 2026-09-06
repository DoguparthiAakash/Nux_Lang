import re

with open("src/compiler.rs", "r") as f:
    content = f.read()

primary_addition = """                  } else if part1 == "ffi_python" {
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

# Find the exact ffi_load block and insert before it
pattern = re.compile(r'(\s*\} else if part1 == "ffi_load" \{)')
match = pattern.search(content)

if match:
    # Insert primary_addition before the match
    content = content[:match.start()] + primary_addition + content[match.start():]
    with open("src/compiler.rs", "w") as f:
        f.write(content)
    print("compiler.rs patched successfully")
else:
    print("Could not find ffi_load using regex")
