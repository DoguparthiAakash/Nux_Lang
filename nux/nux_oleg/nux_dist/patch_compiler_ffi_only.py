import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

stmt_code = """
                 if part1 == "ffi_python" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
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
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_FFI_C\\n");
                     return Ok(());
                 }
"""

expr_code = """} else if part1 == "ffi_python" {
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

if "OP_FFI_PYTHON" not in content:
    content = content.replace('if part1 == "vbe_set_mode" {', stmt_code + '\n                   if part1 == "vbe_set_mode" {')
    content = content.replace('} else if part1 == "dm_get" {', expr_code + '} else if part1 == "dm_get" {')
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("Patched compiler.rs with FFI opcodes!")
else:
    print("FFI already exists!")
