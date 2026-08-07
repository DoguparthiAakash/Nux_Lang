import sys

def patch():
    with open('src/high_level.rs', 'r') as f:
        content = f.read()

    old = """                          } else {
                               // If not resolved, assume opcode/label
                               out.push_str(name); out.push('\\n');
                          }
                          self.advance();"""

    new = """                          } else {
                               // If not resolved, assume opcode/label
                               out.push_str(name); 
                               self.advance();
                               let upper = name.to_ascii_uppercase();
                               if upper == "PUSH" || upper == "GET_LOCAL" || upper == "SET_LOCAL" 
                                   || upper == "OP_GET_LOCAL" || upper == "OP_SET_LOCAL"
                                   || upper == "GET_GLOBAL" || upper == "SET_GLOBAL"
                                   || upper == "JMP" || upper == "JE" || upper == "CALL" {
                                   
                                   if upper == "CALL" {
                                        if let crate::lexer::Token::Identifier(lbl) = &self.current_token {
                                            out.push_str(&format!(" {}", lbl));
                                            self.advance();
                                        }
                                        if let crate::lexer::Token::Number(num) = &self.current_token {
                                            out.push_str(&format!(" {}", num));
                                            self.advance();
                                        }
                                   } else {
                                       if let crate::lexer::Token::Number(num) = &self.current_token {
                                           out.push_str(&format!(" {}", num));
                                           self.advance();
                                       } else if let crate::lexer::Token::Identifier(arg) = &self.current_token {
                                           out.push_str(&format!(" {}", arg));
                                           self.advance();
                                       }
                                   }
                               }
                               out.push('\\n');
                               continue;
                          }
                          self.advance();"""
    
    if old not in content:
        print("ERROR: old text not found in src/high_level.rs!")
        sys.exit(1)
        
    content = content.replace(old, new)
    
    with open('src/high_level.rs', 'w') as f:
        f.write(content)
    print("Patched successfully!")

if __name__ == '__main__':
    patch()
