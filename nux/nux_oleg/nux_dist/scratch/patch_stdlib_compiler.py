import sys

def patch_file(filepath, replacements):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    
    for old, new in replacements:
        if old in content:
            content = content.replace(old, new)
        else:
            print(f"Warning: Could not find snippet in {filepath}:\n{old[:100]}...")
            
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(content)

# 1. Update Compiler
compiler_replacements = [
    (
        'Token::NetListen => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_NET_LISTEN\\n"); Ok(Type::Int) },',
        """Token::FsRead => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FS_READ\\n"); Ok(Type::Int) },
                Token::FsWrite => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::Comma{return self.error(",".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FS_WRITE\\n"); Ok(Type::Int) },
                Token::FsExists => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_FS_EXISTS\\n"); Ok(Type::Int) },
                Token::OsEnv => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_OS_ENV\\n"); Ok(Type::Int) },
                Token::OsCwd => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_OS_CWD\\n"); Ok(Type::Int) },
                Token::OsExec => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_OS_EXEC\\n"); Ok(Type::Int) },
                Token::TimeNow => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_TIME_NOW\\n"); Ok(Type::Int) },
                Token::TimeSleep => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_TIME_SLEEP\\n"); Ok(Type::Int) },
                Token::NetListen => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} self.advance(); self.parse_expression(out)?; if self.current_token!=Token::RParen{return self.error(")".to_string());} self.advance(); out.push_str("OP_NET_LISTEN\\n"); Ok(Type::Int) },"""
    )
]

patch_file("src/compiler.rs", compiler_replacements)

# 2. Update Assembler
assembler_replacements = [
    (
        '"OP_NET_LISTEN_TLS" => ops.push(0xB5),',
        '"OP_NET_LISTEN_TLS" => ops.push(0xB5),\n            "OP_FS_READ" => ops.push(0xC0),\n            "OP_FS_WRITE" => ops.push(0xC1),\n            "OP_FS_EXISTS" => ops.push(0xC2),\n            "OP_OS_ENV" => ops.push(0xC5),\n            "OP_OS_CWD" => ops.push(0xC6),\n            "OP_OS_EXEC" => ops.push(0xC7),\n            "OP_TIME_NOW" => ops.push(0xCA),\n            "OP_TIME_SLEEP" => ops.push(0xCB),'
    )
]

patch_file("src/assembler.rs", assembler_replacements)

print("Compiler and assembler patched for stdlib opcodes!")
