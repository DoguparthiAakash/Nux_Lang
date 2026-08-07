import re
import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

def make_regex(text):
    escaped = re.escape(text.strip())
    return re.sub(r'\\?[ \t\r\n]+', r'\\s+', escaped)

# Fix 1: OP_IMG_ALLOC size
t1 = '''out.push_str(&format!("PUSH {}\\nPUSH 1\\nOP_IMG_ALLOC\\n", size));'''
r1 = '''out.push_str(&format!("PUSH {}\\nPUSH 1\\nOP_IMG_ALLOC\\n", size * 8));'''
code = re.sub(make_regex(t1), r1, code)

# Fix 2: parse_var_decl type annotations
t2 = '''let mut constraint = None;
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

let mut final_type = expected_type.clone();'''
r2 = '''let mut constraint = None;
let mut final_type = expected_type.clone();
if self.current_token == Token::Colon {
     self.advance(); // skip :
     match &self.current_token {
         Token::Identifier(s) => {
             if let Some(bounds) = self.bound_types.get(s) {
                 constraint = Some(*bounds);
             } else {
                 final_type = Type::Class(s.clone());
             }
         },
         Token::KwInt => final_type = Type::Int,
         Token::KwFloat => final_type = Type::Float,
         Token::KwString => final_type = Type::String,
         _ => {}
     }
     self.advance(); // consume type
}'''
code = re.sub(make_regex(t2), r2, code)

# Fix 3: parse_func args
t3 = '''let mut args = Vec::new();
if self.current_token != Token::RParen {
    loop {
        let arg_name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected argument name".to_string()),
        };
        self.advance();
        args.push(arg_name);
        
        if self.current_token == Token::Comma {
            self.advance();
        } else {
            break;
        }
    }
}
if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
self.advance();

if self.current_token != Token::LBrace { return self.error("Expected '{'".to_string()); }'''
r3 = '''let mut args = Vec::new();
if self.current_token != Token::RParen {
    loop {
        let arg_name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected argument name".to_string()),
        };
        self.advance();
        
        let mut arg_type = Type::Unknown;
        if self.current_token == Token::Colon {
            self.advance(); // consume :
            match &self.current_token {
                Token::Identifier(tname) => arg_type = Type::Class(tname.clone()),
                Token::KwInt => arg_type = Type::Int,
                Token::KwFloat => arg_type = Type::Float,
                Token::KwString => arg_type = Type::String,
                _ => {}
            }
            self.advance(); // consume type
        }
        
        args.push((arg_name, arg_type));
        
        if self.current_token == Token::Comma {
            self.advance();
        } else {
            break;
        }
    }
}
if self.current_token != Token::RParen { return self.error("Expected ')'".to_string()); }
self.advance();

if self.current_token == Token::Minus {
     self.advance();
     if self.current_token == Token::Gt {
          self.advance();
          self.advance(); // consume return type identifier/keyword
     }
}

if self.current_token != Token::LBrace { return self.error("Expected '{'".to_string()); }'''
code = re.sub(make_regex(t3), r3, code)

# Fix 4: parse_func arg scopes
t4 = '''for (i, arg) in args.iter().enumerate() {
     // Arguments are at positive offsets 0, 1, 2, ...
     // If class method, they shift by 1.
     let offset = (i + arg_start) as i64;
     let loc = VarLocation::Local(offset);
     if let Some(scope) = self.scopes.last_mut() {
         scope.insert(arg.clone(), (loc, Type::Int));
     }
}'''
r4 = '''for (i, (arg_name, arg_type)) in args.iter().enumerate() {
     // Arguments are at positive offsets 0, 1, 2, ...
     // If class method, they shift by 1.
     let offset = (i + arg_start) as i64;
     let loc = VarLocation::Local(offset);
     if let Some(scope) = self.scopes.last_mut() {
         let ty = if *arg_type == Type::Unknown { Type::Int } else { arg_type.clone() };
         scope.insert(arg_name.clone(), (loc, ty));
     }
}'''
code = re.sub(make_regex(t4), r4, code)

# Fix 5: parse_class pre-registration and increment registration
t5 = '''let mut fields = HashMap::new();
let mut offset = 0;

// Inside class, we expect functions (methods).
while self.current_token != Token::RBrace && self.current_token != Token::EOF {
    if self.current_token == Token::Func {
        self.parse_func(out, &name, AccessModifier::Public)?;
    } else if self.current_token == Token::Var {
        // Field Declaration: var x: type;
        self.advance();
        let field_name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected field name".to_string())
        };
        self.advance();
        
        // Add to fields
        fields.insert(field_name, offset);
        offset += 1; // All fields are 8 bytes (1 slot)
        
        // Optional initialization or type?
        // Expect : Type
        if self.current_token == Token::Colon {
            self.advance();
            // Consume type
            // Identifier or KwType
            self.advance(); 
        }
        
        // For now expect ;
        if self.current_token == Token::SemiColon { self.advance(); }
    } else {
        return self.error("Only functions/fields allowed in classes for now".to_string());
    }
}

// Register Class
self.classes.insert(name, ClassInfo { fields, size: offset });'''
r5 = '''let mut fields = HashMap::new();
let mut offset = 0;

self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: offset });

// Inside class, we expect functions (methods).
while self.current_token != Token::RBrace && self.current_token != Token::EOF {
    if self.current_token == Token::Func {
        self.parse_func(out, &name, AccessModifier::Public)?;
    } else if self.current_token == Token::Var {
        // Field Declaration: var x: type;
        self.advance();
        let field_name = match &self.current_token {
            Token::Identifier(s) => s.clone(),
            _ => return self.error("Expected field name".to_string())
        };
        self.advance();
        
        // Add to fields
        fields.insert(field_name, offset);
        offset += 1; // All fields are 8 bytes (1 slot)
        self.classes.insert(name.clone(), ClassInfo { fields: fields.clone(), size: offset });
        
        // Optional initialization or type?
        // Expect : Type
        if self.current_token == Token::Colon {
            self.advance();
            // Consume type
            // Identifier or KwType
            self.advance(); 
        }
        
        // For now expect ;
        if self.current_token == Token::SemiColon { self.advance(); }
    } else {
        return self.error("Only functions/fields allowed in classes for now".to_string());
    }
}'''
code = re.sub(make_regex(t5), r5, code)

# Fix 6: obj.method() call resolution
t6 = '''// Resolve "this"/object again
let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };

match loc {
     VarLocation::Local(_) | VarLocation::Global(_) => {
         out.push_str(&format!("CALL {}_{} {}\\nPOP\\n", part1, member, arg_count));
     }
}'''
r6 = '''// Resolve "this"/object again
let (loc, typ) = if let Some(r) = self.resolve_var(&part1) { r } else { return self.error(format!("Undefined variable '{}'", part1)); };

let cname = if let Type::Class(c) = typ { c.clone() } else { return self.error(format!("Cannot call method '{}' on variable '{}' of non-class type", member, part1)); };

match loc {
     VarLocation::Local(_) | VarLocation::Global(_) => {
         out.push_str(&format!("CALL {}_{} {}\\nPOP\\n", cname, member, arg_count));
     }
}'''
code = re.sub(make_regex(t6), r6, code)

# Fix 7: OP_ADD for fields
t7 = '''// Add field offset
// Need class info
if let Type::Class(cname) = typ {
    if let Some(info) = self.classes.get(&cname) {
         if let Some(&foffset) = info.fields.get(&member) {
              out.push_str(&format!("PUSH {}\\nOP_ADD\\n", foffset));
         } else {
              return self.error(format!("Field '{}' not found in class '{}'", member, cname));
         }
    }
} else {
     // Fallback offset (hack)
     out.push_str(&format!("PUSH 0\\nOP_ADD\\n"));
}'''
r7 = '''// Add field offset
// Need class info
if let Type::Class(cname) = typ {
    if let Some(info) = self.classes.get(&cname) {
         if let Some(&foffset) = info.fields.get(&member) {
              out.push_str(&format!("PUSH {}\\nOP_ADD\\n", foffset * 8));
         } else {
              return self.error(format!("Field '{}' not found in class '{}'", member, cname));
         }
    }
} else {
     // Fallback offset (hack)
     // Try finding the field in any class
     let mut found_offset = None;
     for class_info in self.classes.values() {
         if let Some(&foffset) = class_info.fields.get(&member) {
             found_offset = Some(foffset * 8);
             break;
         }
     }
     if let Some(foffset) = found_offset {
         out.push_str(&format!("PUSH {}\\nOP_ADD\\n", foffset));
     } else {
         return self.error(format!("Field '{}' not found in any class (variable '{}' type unknown)", member, part1));
     }
}'''
code = re.sub(make_regex(t7), r7, code)

# Fix 8: POKE needs OP_SWAP
t8 = '''// Now stack has Address
out.push_str("POKE\\n");'''
r8 = '''// Now stack has Address
out.push_str("OP_SWAP\\nPOKE\\n");'''
code = re.sub(make_regex(t8), r8, code)


with open(path, 'w', encoding='utf-8') as f:
    f.write(code)

print("Applied patches.")
