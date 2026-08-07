import re

with open('src/high_level.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Replace the block
target = '''                      } else if self.current_token == Token::LParen {
                          // Method Call
                          // Resolve "this"/object again
                          let (loc, _) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };'''

replacement = '''                      } else if self.current_token == Token::LParen {
                          // Method Call
                          // Resolve "this"/object again
                          let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };
                          let cname = if let Type::Class(c) = typ { c.clone() } else { return self.error(format!("Cannot call method '{}' on variable '{}' of non-class type", member, part1)); };'''

code = code.replace(target, replacement)

target2 = '''                          if expect_semi {
                               if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                               self.advance();
                          } else if self.current_token == Token::SemiColon { self.advance(); }
                          
                          out.push_str(&format!("CALL {}_{} {}\\nPOP\\n", part1, member, arg_count));'''

replacement2 = '''                          if expect_semi {
                               if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                               self.advance();
                          } else if self.current_token == Token::SemiColon { self.advance(); }
                          
                          out.push_str(&format!("CALL {}_{} {}\\nPOP\\n", cname, member, arg_count));'''

code = code.replace(target2, replacement2)

with open('src/high_level.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print("Done")
