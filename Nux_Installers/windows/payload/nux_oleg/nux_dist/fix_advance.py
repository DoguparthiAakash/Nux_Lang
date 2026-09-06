import re

with open('src/compiler.rs', 'r') as f:
    content = f.read()

# For cux_load
content = content.replace(
    '} else if part1 == "cux_load" {\n                      self.advance();\n                      if self.current_token != Token::LParen',
    '} else if part1 == "cux_load" {\n                      if self.current_token != Token::LParen'
)

# For cux_call
content = content.replace(
    '} else if part1 == "cux_call" {\n                      self.advance();\n                      if self.current_token != Token::LParen',
    '} else if part1 == "cux_call" {\n                      if self.current_token != Token::LParen'
)

with open('src/compiler.rs', 'w') as f:
    f.write(content)
print("Removed extra advance calls!")
