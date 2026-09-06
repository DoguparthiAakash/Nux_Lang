import sys

def patch():
    with open('src/high_level.rs', 'r') as f:
        content = f.read()

    # 1. Change args to Vec<(String, Type)>
    old_args_decl = "let mut args = Vec::new();"
    new_args_decl = "let mut args: Vec<(String, Type)> = Vec::new();"
    content = content.replace(old_args_decl, new_args_decl)

    # 2. Fix arg parsing
    old_arg_parse = """                let arg_name = match &self.current_token {
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
                
                args.push(arg_name);"""
                
    new_arg_parse = """                let arg_name = match &self.current_token {
                    Token::Identifier(s) => s.clone(),
                    _ => return self.error("Expected argument name".to_string()),
                };
                self.advance();
                
                let mut arg_type = Type::Unknown;
                if self.current_token == Token::Colon {
                    self.advance();
                    match &self.current_token {
                        Token::Identifier(s) => { arg_type = Type::Class(s.clone()); self.advance(); },
                        Token::KwInt => { arg_type = Type::Int; self.advance(); },
                        Token::KwFloat => { arg_type = Type::Float; self.advance(); },
                        Token::KwByte => { arg_type = Type::Byte; self.advance(); },
                        Token::KwShort => { arg_type = Type::Short; self.advance(); },
                        Token::KwLong => { arg_type = Type::Long; self.advance(); },
                        Token::KwChar => { arg_type = Type::Char; self.advance(); },
                        Token::KwString => { arg_type = Type::String; self.advance(); },
                        _ => return self.error("Expected type after colon".to_string()),
                    }
                }
                
                args.push((arg_name, arg_type));"""
    content = content.replace(old_arg_parse, new_arg_parse)

    # 3. Add to scope
    old_scope = """        for (i, arg) in args.iter().enumerate() {
             // Arguments are at positive offsets 0, 1, 2, ...
             // If class method, they shift by 1.
             let offset = (i + arg_start) as i64;
             let loc = VarLocation::Local(offset);
             if let Some(scope) = self.scopes.last_mut() {
                 scope.insert(arg.clone(), (loc, Type::Int));
             }
        }"""
    new_scope = """        for (i, (arg, arg_typ)) in args.iter().enumerate() {
             // Arguments are at positive offsets 0, 1, 2, ...
             // If class method, they shift by 1.
             let offset = (i + arg_start) as i64;
             let loc = VarLocation::Local(offset);
             if let Some(scope) = self.scopes.last_mut() {
                 scope.insert(arg.clone(), (loc, arg_typ.clone()));
             }
        }"""
    content = content.replace(old_scope, new_scope)

    # 4. Method calls in parse_primary
    old_dot = """                    // Handle Chain: .x.y.z
                    while self.current_token == Token::Dot {
                        self.advance();
                        let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                        self.advance();
                        
                        let offset = if let Type::Class(cname) = &typ {"""
    
    new_dot = """                    // Handle Chain: .x.y.z
                    while self.current_token == Token::Dot {
                        self.advance();
                        let member = match &self.current_token { Token::Identifier(s) => s.clone(), _ => return self.error("Expected member name".to_string()) };
                        self.advance();
                        
                        if self.current_token == Token::LParen {
                            self.advance();
                            let mut arg_count = 1;
                            if self.current_token != Token::RParen {
                                loop {
                                    self.parse_expression_and_push(out)?;
                                    arg_count += 1;
                                    if self.current_token == Token::Comma { self.advance(); } else { break; }
                                }
                            }
                            if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                            self.advance();
                            
                            let cname = if let Type::Class(cname) = &typ {
                                cname.clone()
                            } else {
                                return self.error(format!("Method '{}' called on non-class type {:?}", member, typ));
                            };
                            
                            out.push_str(&format!("CALL {}_{} {}\\n", cname, member, arg_count));
                            typ = Type::Unknown; // Methods return unknown for now
                            continue;
                        }
                        
                        let offset = if let Type::Class(cname) = &typ {"""
    content = content.replace(old_dot, new_dot)
    
    with open('src/high_level.rs', 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch()
    print("Patched compiler 2!")
