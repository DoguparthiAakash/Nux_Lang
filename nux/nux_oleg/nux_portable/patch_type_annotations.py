import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target = '''                  self.advance();
                  args.push(arg_name);
                  
                  if self.current_token == Token::Comma {'''

repl = '''                  self.advance();
                  args.push(arg_name);
                  
                  if self.current_token == Token::Colon {
                      self.advance(); // consume :
                      self.advance(); // consume type
                  }
                  
                  if self.current_token == Token::Comma {'''

code = code.replace(target, repl)
code = code.replace(target.replace('\n', '\r\n'), repl)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched parse_func to support type annotations.")
