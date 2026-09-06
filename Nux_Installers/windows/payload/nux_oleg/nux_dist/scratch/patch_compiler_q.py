import re

with open('src/compiler.rs', 'r', encoding='utf-8') as f:
    code = f.read()

patch = """
            Token::QAlloc => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_ALLOC\\n");
                Ok(Type::Void)
            },
            Token::QH => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_H\\n");
                Ok(Type::Void)
            },
            Token::QX => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_X\\n");
                Ok(Type::Void)
            },
            Token::QZ => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_Z\\n");
                Ok(Type::Void)
            },
            Token::QCx => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::Comma { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_CX\\n");
                Ok(Type::Void)
            },
            Token::QMeasure => {
                self.advance();
                if self.current_token == Token::LParen { self.advance(); }
                self.parse_expression(out)?;
                if self.current_token == Token::RParen { self.advance(); }
                out.push_str("OP_Q_MEASURE\\n");
                Ok(Type::Int)
            },
"""

code = code.replace('Token::TimeSleep => {', patch + '            Token::TimeSleep => {')

with open('src/compiler.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print('Compiler patched.')
