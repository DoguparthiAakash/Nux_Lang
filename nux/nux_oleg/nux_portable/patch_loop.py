import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target1 = '''          for (i, arg) in args.iter().enumerate() {
               // Arguments are at positive offsets 0, 1, 2, ...
               // If class method, they shift by 1.
               let offset = (i + arg_start) as i64;
               let loc = VarLocation::Local(offset);
               if let Some(scope) = self.scopes.last_mut() {
                   scope.insert(arg.clone(), (loc, Type::Int));
               }
          }'''

repl1 = '''          for (i, (arg_name, arg_type)) in args.iter().enumerate() {
               // Arguments are at positive offsets 0, 1, 2, ...
               // If class method, they shift by 1.
               let offset = (i + arg_start) as i64;
               let loc = VarLocation::Local(offset);
               if let Some(scope) = self.scopes.last_mut() {
                   scope.insert(arg_name.clone(), (loc, arg_type.clone()));
               }
          }'''

code = code.replace(target1, repl1)
code = code.replace(target1.replace('\n', '\r\n'), repl1)

target2 = '''                        Token::KwBool => arg_type = Type::Bool,'''
repl2 = '''                        // No Token::KwBool currently'''

code = code.replace(target2, repl2)
code = code.replace(target2.replace('\n', '\r\n'), repl2)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched loop and KwBool.")
