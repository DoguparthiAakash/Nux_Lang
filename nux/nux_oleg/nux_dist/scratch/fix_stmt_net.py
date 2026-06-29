import os

compiler_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(compiler_path, "r", encoding="utf-8") as f:
    content = f.read()

stmt_patch = """                Token::NetListen | Token::NetAccept | Token::NetRead | Token::NetWrite | Token::NetClose | Token::NetListenTls => {
                    self.parse_expression(out)?;
                    out.push_str("POP\\n");
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
"""

if "Token::NetClose | Token::NetListenTls =>" not in content:
    content = content.replace(
        "Token::ImgGet => {",
        stmt_patch + "               Token::ImgGet => {"
    )

with open(compiler_path, "w", encoding="utf-8") as f:
    f.write(content)

print("Compiler patched with net statements")
