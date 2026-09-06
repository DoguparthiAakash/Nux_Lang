import os

lexer_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\lexer.rs"
compiler_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

# Patch lexer
with open(lexer_path, "r", encoding="utf-8") as f:
    lex_data = f.read()

if "PrintStr" not in lex_data:
    lex_data = lex_data.replace("Print,", "Print, PrintStr, PrintVal,")

if "\"print_str\" => Token::PrintStr" not in lex_data:
    lex_data = lex_data.replace(
        "\"print\" => Token::Print,",
        "\"print\" => Token::Print,\n            \"print_str\" => Token::PrintStr,\n            \"print_val\" => Token::PrintVal,"
    )

with open(lexer_path, "w", encoding="utf-8") as f:
    f.write(lex_data)

# Patch compiler
with open(compiler_path, "r", encoding="utf-8") as f:
    comp_data = f.read()

if "Token::PrintStr" not in comp_data:
    # Add to the initial check in parse_statement
    comp_data = comp_data.replace(
        "Token::Print | Token::Println",
        "Token::Print | Token::Println | Token::PrintStr | Token::PrintVal"
    )

    # Add to parse_statement_impl
    print_stmt = """                Token::PrintStr => {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("PRINT_STR\\n");
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
                Token::PrintVal => {
                    self.advance();
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("PRINT_VAL\\n");
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
"""
    comp_data = comp_data.replace(
        "Token::Println => {",
        print_stmt + "                Token::Println => {"
    )

with open(compiler_path, "w", encoding="utf-8") as f:
    f.write(comp_data)

print("print_str and print_val patched")
