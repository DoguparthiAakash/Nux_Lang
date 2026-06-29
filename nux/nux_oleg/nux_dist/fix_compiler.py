with open('src/compiler.rs', 'r', encoding='utf-8') as f:
    code = f.read()

code = code.replace('self.compile_expression();', 'let mut dummy = String::new(); self.parse_expression(&mut dummy)?;')
code = code.replace('if self.current_token == Token::SemiColon { self.advance(); }\n            },', 'if self.current_token == Token::SemiColon { self.advance(); }\n                Ok(())\n            },')
code = code.replace('self.emit("OP_Q_MEASURE");\n            },', 'self.emit("OP_Q_MEASURE");\n                Ok(Type::Int)\n            },')

with open('src/compiler.rs', 'w', encoding='utf-8') as f:
    f.write(code)
print('Compiler fixed.')
