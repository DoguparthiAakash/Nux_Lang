import sys
with open('src/compiler.rs', 'r') as f:
    lines = f.readlines()

content = ''.join(lines)
target1 = '''                } else if part1 == "dm_set" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_DM_SET\\n");
                    return Ok(Type::Int);'''

replacement1 = target1 + '''
                } else if part1 == "arr_new" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_ARRAY_NEW\\n");
                    return Ok(Type::Int);
                } else if part1 == "arr_get" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_ARRAY_GET\\n");
                    return Ok(Type::Int);
                } else if part1 == "arr_set" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_ARRAY_SET\\n");
                    return Ok(Type::Int);
                } else if part1 == "arr_len" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_ARRAY_LEN\\n");
                    return Ok(Type::Int);
                } else if part1 == "arr_fill" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_ARRAY_FILL\\n");
                    return Ok(Type::Int);
                } else if part1 == "ffi_load" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str("OP_FFI_LOAD\\n");
                    return Ok(Type::Int);
                } else if part1 == "ffi_invoke" {
                    if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }
                    self.advance();
                    self.parse_expression(out)?; 
                    if self.current_token != Token::Comma { return self.error("Expected ,".to_string()); }
                    self.advance();
                    self.parse_expression(out)?; 
                    
                    let mut arg_count = 0;
                    while self.current_token == Token::Comma {
                        self.advance();
                        self.parse_expression(out)?;
                        arg_count += 1;
                    }
                    if self.current_token != Token::RParen { return self.error("Expected )".to_string()); }
                    self.advance();
                    out.push_str(&format!("PUSH {}\\n", arg_count));
                    out.push_str("OP_FFI_CALL\\n");
                    return Ok(Type::Int);'''

if target1 in content:
    content = content.replace(target1, replacement1)
    print('Replaced part1 successfully')
else:
    print('target1 not found')
    
target2 = '''                            let offset = if let Type::Class(cname) = &typ {
                                 if let Some(cinfo) = self.classes.get(cname) { 
                                     if let Some(f) = cinfo.fields.get(&member) { *f } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                 } else { return self.error(format!("Unknown class '{}'", cname)); }
                             } else {
                                 if let Some(off) = self.current_class_fields.get(&member) {
                                     *off
                                 } else {
                                     let mut found = None; for (cname, cinfo) in &self.classes { if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); } }
                                     if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                                 }
                            };
                            out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset)); typ = Type::Unknown;'''

replacement2 = '''                            let offset = if let Type::Class(cname) = &typ {
                                 if Some(cname) == self.current_class_name.as_ref() {
                                     if let Some(off) = self.current_class_fields.get(&member) { *off }
                                     else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                 } else if let Some(cinfo) = self.classes.get(cname) { 
                                     if let Some(f) = cinfo.fields.get(&member) { *f } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                 } else { return self.error(format!("Unknown class '{}'", cname)); }
                             } else {
                                 if let Some(off) = self.current_class_fields.get(&member) {
                                     *off
                                 } else {
                                     let mut found = None; for (cname, cinfo) in &self.classes { if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); } }
                                     if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                                 }
                            };
                            out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8)); typ = Type::Unknown;'''

if target2 in content:
    content = content.replace(target2, replacement2)
    print('Replaced part2 successfully')
else:
    print('target2 not found')

target3 = '''        for (k, v) in sub_parser.classes {
            self.classes.insert(k, v);
        }'''

replacement3 = '''        if !sub_parser.errors.is_empty() {
             for e in &sub_parser.errors {
                 eprintln!("Import Parse Error: {:?}", e);
             }
        }
        for (k, v) in sub_parser.classes {
            self.classes.insert(k, v);
        }
        for (k, v) in sub_parser.enums {
            self.enums.insert(k, v);
        }
        for (k, v) in sub_parser.traits {
            self.traits.insert(k, v);
        }'''

if target3 in content:
    content = content.replace(target3, replacement3)
    print('Replaced part3 successfully')
else:
    print('target3 not found')

with open('src/compiler.rs', 'w') as f:
    f.write(content)
