import sys

def patch_spawn():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()
        
    find_primary_match = r'''    fn parse_primary(&mut self, out: &mut String) -> Result<Type, CompileError> {
        match &self.current_token {
            Token::New => {'''
            
    replace_spawn_primary = r'''    fn parse_primary(&mut self, out: &mut String) -> Result<Type, CompileError> {
        match &self.current_token.clone() {
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
            Token::New => {'''
            
    if find_primary_match in content:
        content = content.replace(find_primary_match, replace_spawn_primary)
        print("Patched Spawn in parse_primary")
    else:
        print("Failed to patch Spawn in parse_primary")

    with open(path, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch_spawn()
