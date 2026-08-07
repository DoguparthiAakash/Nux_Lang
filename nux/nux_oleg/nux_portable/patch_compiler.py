import sys

def patch():
    with open('src/high_level.rs', 'r') as f:
        content = f.read()

    # 1. Early class registration
    old_class = """        let mut fields = HashMap::new();
        let mut offset = 0;
        
        // Inside class, we expect functions (methods)."""
    new_class = """        let mut fields = HashMap::new();
        let mut offset = 0;
        
        self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: 0 });
        
        // Inside class, we expect functions (methods)."""
    content = content.replace(old_class, new_class)

    # 2. Update class registration inside the loop
    old_var = """                fields.insert(field_name, offset);
                offset += 1; // All fields are 8 bytes (1 slot)
                
                // Optional initialization or type?"""
    new_var = """                fields.insert(field_name, offset);
                offset += 1; // All fields are 8 bytes (1 slot)
                self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: offset });
                
                // Optional initialization or type?"""
    content = content.replace(old_var, new_var)
    
    # 3. Fix parse_func arguments
    old_args = """                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                args.push(arg_name);
                
                if self.current_token == Token::Comma {"""
    new_args = """                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                
                if self.current_token == Token::Colon {
                    self.advance();
                    match self.current_token {
                        Token::Identifier(_) | Token::KwInt | Token::KwFloat | Token::KwByte | Token::KwShort | Token::KwLong | Token::KwChar | Token::KwString => {
                            self.advance();
                        },
                        _ => return self.error("Expected type after colon".to_string()),
                    }
                }
                
                args.push(arg_name);
                
                if self.current_token == Token::Comma {"""
    content = content.replace(old_args, new_args)

    with open('src/high_level.rs', 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch()
    print("Patched compiler!")
