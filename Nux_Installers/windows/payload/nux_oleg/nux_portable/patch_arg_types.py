import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target1 = '''                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                args.push(arg_name);
                
                if self.current_token == Token::Colon {
                    self.advance(); // consume :
                    self.advance(); // consume type
                }'''

repl1 = '''                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                
                let mut arg_type = Type::Unknown;
                if self.current_token == Token::Colon {
                    self.advance(); // consume :
                    match &self.current_token {
                        Token::Identifier(tname) => {
                            arg_type = Type::Class(tname.clone());
                        },
                        Token::KwInt => arg_type = Type::Int,
                        Token::KwFloat => arg_type = Type::Float,
                        Token::KwBool => arg_type = Type::Bool,
                        Token::KwString => arg_type = Type::String,
                        _ => {}
                    }
                    self.advance(); // consume type
                }
                args.push((arg_name, arg_type));'''

target2 = '''          for (i, arg) in args.iter().enumerate() {
               // Arguments are at positive offsets 0, 1, 2, ...
               // If class method, they shift by 1.
               let offset = (i + arg_start) as i64;
               let loc = VarLocation::Local(offset);
               if let Some(scope) = self.scopes.last_mut() {
                   scope.insert(arg.clone(), (loc, Type::Int));'''

repl2 = '''          for (i, (arg, typ)) in args.iter().enumerate() {
               // Arguments are at positive offsets 0, 1, 2, ...
               // If class method, they shift by 1.
               let offset = (i + arg_start) as i64;
               let loc = VarLocation::Local(offset);
               if let Some(scope) = self.scopes.last_mut() {
                   scope.insert(arg.clone(), (loc, typ.clone()));'''

code = code.replace(target1, repl1)
code = code.replace(target1.replace('\n', '\r\n'), repl1)
code = code.replace(target2, repl2)
code = code.replace(target2.replace('\n', '\r\n'), repl2)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched parse_func to store argument types.")
