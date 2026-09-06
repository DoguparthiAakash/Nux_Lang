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
        content = content.replace(find_str, replace_str)

    content = content.replace(
        'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Spawn |',
        'Token::If | Token::While | Token::Do | Token::For | Token::Asm | Token::Join |'
    )
    
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
    
    content = content.replace(find_spawn_impl, replace_join_impl)
    
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
            
    content = content.replace(find_primary_match, replace_spawn_primary)

    with open(path, 'w') as f:
        f.write(content)

    # 2. FIX ASSEMBLER.RS
    asm_path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\assembler.rs'
    with open(asm_path, 'r') as f:
        asm_content = f.read()
        
    asm_content = asm_content.replace('            "OP_SPAWN" => ops.push(0xE0),\n', '')
    asm_content = asm_content.replace('            "OP_SPAWN" => {', '            "SPAWN" => {')
    asm_content = asm_content.replace('OP_SPAWN missing label', 'SPAWN missing label')
    asm_content = asm_content.replace('ops.push(0xE0);', 'ops.push(0xE0);') # ensure 0xE0
    
    join_idx = asm_content.find('            "SPAWN" => {')
    if join_idx != -1:
        asm_content = asm_content[:join_idx] + '            "OP_JOIN" => ops.push(0xE1),\n' + asm_content[join_idx:]

    with open(asm_path, 'w') as f:
        f.write(asm_content)
        
    # 3. FIX BYTECODE.RS
    bc_path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\nvm\bytecode.rs'
    with open(bc_path, 'r') as f:
        bc_content = f.read()
        
    bc_content = bc_content.replace('SPAWN_THREAD = 0x90,', 'SPAWN_THREAD = 0xE0,')
    bc_content = bc_content.replace('JOIN_THREAD = 0x91,', 'JOIN_THREAD = 0xE1,')
    bc_content = bc_content.replace('0x90 => Some(Opcode::SPAWN_THREAD),', '0xE0 => Some(Opcode::SPAWN_THREAD),')
    bc_content = bc_content.replace('0x91 => Some(Opcode::JOIN_THREAD),', '0xE1 => Some(Opcode::JOIN_THREAD),')
    
    with open(bc_path, 'w') as f:
        f.write(bc_content)
        
    print("Patch applied successfully.")

if __name__ == '__main__':
    patch_compiler()
