import sys

def patch_compiler():
    # 1. FIX COMPILER.RS
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()
        
    intrinsics = [
        "sec_login", "syscall", "peek_ptr", "sec_whoami", "dm_get",
        "vbe_get_fb", "vbe_get_key", "vbe_mouse_x", "vbe_mouse_y", "vbe_mouse_down"
    ]
    
    for intri in intrinsics:
        find_str = f'if part1 == "{intri}" {{\n                    self.advance();'
        replace_str = f'if part1 == "{intri}" {{'
        if find_str in content:
            content = content.replace(find_str, replace_str)
            print(f"Patched {intri}")
        else:
            print(f"Failed to patch {intri}")

    find_str = 'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |'
    if find_str in content:
        content = content.replace(find_str, 'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Join |')
        print("Patched Spawn to Join in statement_or_expr")
    else:
        print("Failed to patch Spawn to Join in statement_or_expr")
    
    find_spawn_impl = r'''            Token::Spawn => {
                self.advance();
                let mut func_name = String::new();
                if let Token::Identifier(name) = &self.current_token {
                    func_name = name.clone();
                    self.advance();
                } else {
                    return self.error("Expected function name for spawn".to_string());
                }
                
                if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
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
                if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                self.advance();
                
                out.push_str(&format!("CALL {} {}\n", func_name, arg_count));
                out.push_str("OP_SPAWN\n");
            }'''
            
    replace_join_impl = r'''            Token::Join => {
                self.advance();
                self.parse_expression(out)?;
                if self.current_token != Token::SemiColon { return self.error("Expected ;".to_string()); }
                self.advance();
                out.push_str("OP_JOIN\n");
            }'''
    
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
                
                if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
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
