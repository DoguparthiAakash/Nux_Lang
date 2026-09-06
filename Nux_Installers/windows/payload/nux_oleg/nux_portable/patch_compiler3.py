import sys

def patch():
    with open('src/high_level.rs', 'r') as f:
        content = f.read()

    old_var_type = """        // Optional Type Constraint
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
        
        let mut final_type = expected_type.clone();"""
        
    new_var_type = """        // Optional Type Constraint
        let mut constraint = None;
        let mut explicit_type = None;
        if self.current_token == Token::Colon {
             self.advance(); // skip :
             match &self.current_token {
                 Token::Identifier(s) => {
                     explicit_type = Some(Type::Class(s.clone()));
                     if let Some(bounds) = self.bound_types.get(s) {
                         constraint = Some(*bounds);
                     }
                     self.advance();
                 },
                 Token::KwInt => { explicit_type = Some(Type::Int); self.advance(); },
                 Token::KwFloat => { explicit_type = Some(Type::Float); self.advance(); },
                 Token::KwByte => { explicit_type = Some(Type::Byte); self.advance(); },
                 Token::KwShort => { explicit_type = Some(Type::Short); self.advance(); },
                 Token::KwLong => { explicit_type = Some(Type::Long); self.advance(); },
                 Token::KwChar => { explicit_type = Some(Type::Char); self.advance(); },
                 Token::KwString => { explicit_type = Some(Type::String); self.advance(); },
                 _ => { self.advance(); /* fallback */ }
             }
        }
        
        let mut final_type = if let Some(t) = explicit_type { t } else { expected_type.clone() };"""
        
    content = content.replace(old_var_type, new_var_type)
    
    with open('src/high_level.rs', 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch()
    print("Patched compiler 3!")
