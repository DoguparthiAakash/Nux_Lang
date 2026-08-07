import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target = '''          // Optional Type Constraint
          let mut constraint = None;
          if self.current_token == Token::Colon {
               self.advance(); // skip :
               match &self.current_token {
                   Token::Identifier(s) => {
                       if let Some(bounds) = self.bound_types.get(s) {
                           constraint = Some(*bounds);
                       }
                   },
                   _ => {}
               }
               self.advance(); // consume type
          }
          
          let mut final_type = expected_type.clone();'''

repl = '''          // Optional Type Constraint
          let mut constraint = None;
          let mut final_type = expected_type.clone();
          
          if self.current_token == Token::Colon {
               self.advance(); // skip :
               match &self.current_token {
                   Token::Identifier(s) => {
                       if let Some(bounds) = self.bound_types.get(s) {
                           constraint = Some(*bounds);
                       } else {
                           final_type = Type::Class(s.clone());
                       }
                   },
                   Token::KwInt => final_type = Type::Int,
                   Token::KwFloat => final_type = Type::Float,
                   Token::KwString => final_type = Type::String,
                   _ => {}
               }
               self.advance(); // consume type
          }'''

code = code.replace(target, repl)
code = code.replace(target.replace('\n', '\r\n'), repl)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched parse_var_decl type parsing.")
