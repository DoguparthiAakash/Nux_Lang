import sys

def patch_compiler():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()
        
    find_str = 'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |'
    if find_str in content:
        content = content.replace(find_str, 'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Join |')
    
    find_spawn_impl = r'''             Token::Spawn => {
                  self.advance();
                  match &self.current_token {
                      Token::Identifier(func_name) => { out.push_str(&format!("PUSH {}\nSPAWN\n", func_name)); },
                      _ => return self.error("Expected function name".to_string()),
                  }
                  self.advance(); if self.current_token == Token::SemiColon { self.advance(); }
             },'''
            
    replace_join_impl = r'''             Token::Join => {
                  self.advance();
                  self.parse_expression(out)?;
                  if self.current_token == Token::SemiColon { self.advance(); }
                  out.push_str("OP_JOIN\n");
             },'''
    
    if find_spawn_impl in content:
        content = content.replace(find_spawn_impl, replace_join_impl)
        print("Patched Spawn to Join in parse_statement_impl")
    else:
        print("Failed to patch Spawn to Join in parse_statement_impl")
    
    find_primary_match = r'''        match self.current_token.clone() {
            Token::IntLiteral(n) => {'''
            
    replace_spawn_primary = r'''        match self.current_token.clone() {
            Token::Spawn => {
                self.advance();
                let mut func_name = String::new();
                if let Token::Identifier(name) = &self.current_token {
                    func_name = name.clone();
                    self.advance();
                } else {
                    return self.error("Expected function name for spawn".to_string());
                }
                
                if self.current_token == Token::LParen {
                    self.advance();
                    let mut arg_count = 0;
                    if self.current_token != Token::RParen {
                        loop {
                            self.parse_expression(out)?;
                            arg_count += 1;
                            if self.current_token == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str(&format!("SPAWN {} {}\n", func_name, arg_count));
                } else {
                    out.push_str(&format!("PUSH {}\nSPAWN\n", func_name));
                }
                Ok(Type::Int)
            },
            Token::IntLiteral(n) => {'''
            
    if find_primary_match in content:
        content = content.replace(find_primary_match, replace_spawn_primary)
        print("Patched Spawn in parse_primary")
    else:
        print("Failed to patch Spawn in parse_primary")

    with open(path, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch_compiler()
