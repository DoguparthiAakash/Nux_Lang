import os

compiler_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(compiler_path, "r", encoding="utf-8") as f:
    content = f.read()

tls_intrinsic = """                Token::NetListenTls => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_NET_LISTEN_TLS\\n"); Ok(Type::Int) },
"""

if "Token::NetListenTls" not in content:
    content = content.replace(
        "Token::NetListen =>",
        tls_intrinsic + "                Token::NetListen =>"
    )

with open(compiler_path, "w", encoding="utf-8") as f:
    f.write(content)

print("Compiler patched with NetListenTls")
