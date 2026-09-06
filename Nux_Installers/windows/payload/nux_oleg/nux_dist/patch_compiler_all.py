import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

stmt_code = """
                 if part1 == "q_alloc" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_Q_ALLOC\\n");
                     return Ok(());
                 }
                 if part1 == "q_h" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_Q_H\\n");
                     return Ok(());
                 }
                 if part1 == "q_x" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_Q_X\\n");
                     return Ok(());
                 }
                 if part1 == "q_z" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_Q_Z\\n");
                     return Ok(());
                 }
                 if part1 == "q_cx" {
                     self.advance();
                     if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                     self.advance();
                     self.parse_expression(out)?;
                     if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                     self.advance();
                     if expect_semi && self.current_token == Token::SemiColon { self.advance(); }
                     else if self.current_token == Token::SemiColon { self.advance(); }
                     out.push_str("OP_Q_CX\\n");
                     return Ok(());
                 }
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

expr_code = """} else if part1 == "q_measure" {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_Q_MEASURE\\n");
                    return Ok(Type::Int);
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

if "OP_Q_ALLOC" not in content:
    # Insert statement patch before vbe_set_mode (which is in parse_statement_impl)
    content = content.replace('if part1 == "vbe_set_mode" {', stmt_code + '\n                   if part1 == "vbe_set_mode" {')
    # Insert expression patch before dm_get (which is in parse_primary)
    content = content.replace('} else if part1 == "dm_get" {', expr_code + '} else if part1 == "dm_get" {')
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("Patched compiler.rs with ALL opcodes!")
else:
    print("Opcodes already exist!")
