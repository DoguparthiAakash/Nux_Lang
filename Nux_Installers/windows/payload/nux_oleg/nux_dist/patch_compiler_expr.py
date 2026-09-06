import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

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

if 'part1 == "ffi_python"' not in content[content.find('Token::Identifier(name) => {'):]:
    content = content.replace('} else if part1 == "syscall" {', expr_code + '} else if part1 == "syscall" {')
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("Patched compiler.rs with FFI expressions!")
else:
    print("FFI expressions already exist!")
