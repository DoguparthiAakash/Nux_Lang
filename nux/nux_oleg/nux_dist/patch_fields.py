import re

with open('src/compiler.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'pub fields: BTreeMap<String, u32>,',
    'pub fields: BTreeMap<String, (u32, Type)>,'
)

content = content.replace(
    'current_class_fields: BTreeMap<String, u32>,',
    'current_class_fields: BTreeMap<String, (u32, Type)>,'
)

content = content.replace(
'''                self.current_class_fields.insert(field_name.clone(), offset);
                fields.insert(field_name, offset);
                offset += 1; 
                if self.current_token == Token::Colon {
                    self.advance();
                    self.advance(); 
                }
                if self.current_token == Token::SemiColon { self.advance(); }''',
'''                let mut field_type = Type::Unknown;
                if self.current_token == Token::Colon {
                    self.advance();
                    match &self.current_token {
                        Token::Identifier(s) => field_type = Type::Class(s.clone()),
                        Token::KwInt => field_type = Type::Int,
                        Token::KwFloat => field_type = Type::Float,
                        Token::KwChar => field_type = Type::Char,
                        Token::KwString => field_type = Type::String,
                        _ => {}
                    }
                    self.advance(); 
                }
                self.current_class_fields.insert(field_name.clone(), (offset, field_type.clone()));
                fields.insert(field_name, (offset, field_type));
                offset += 1; 
                if self.current_token == Token::SemiColon { self.advance(); }'''
)

content = content.replace(
'''                          if self.current_token == Token::Eq {
                               let offset = if let Type::Class(cname) = &typ {
                                   if Some(cname.clone()) == self.current_class_name {
                                       if let Some(off) = self.current_class_fields.get(&member) { *off } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
                                   } else if let Some(cinfo) = self.classes.get(cname) {
                                       if let Some(off) = cinfo.fields.get(&member) { *off } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                   } else { return self.error(format!("Unknown class '{}'", cname)); }
                               } else {
                                   if let Some(off) = self.current_class_fields.get(&member) {
                                       *off
                                   } else {
                                       let mut found = None; for (cname, cinfo) in &self.classes { if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); } }
                                       if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                                   }
                               };''',
'''                          if self.current_token == Token::Eq {
                               let (offset, _field_typ) = if let Type::Class(cname) = &typ {
                                   if Some(cname.clone()) == self.current_class_name {
                                       if let Some(&(off, ref t)) = self.current_class_fields.get(&member) { (off, t.clone()) } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
                                   } else if let Some(cinfo) = self.classes.get(cname) {
                                       if let Some(&(off, ref t)) = cinfo.fields.get(&member) { (off, t.clone()) } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                   } else { return self.error(format!("Unknown class '{}'", cname)); }
                               } else {
                                   if let Some(&(off, ref t)) = self.current_class_fields.get(&member) {
                                       (off, t.clone())
                                   } else {
                                       let mut found = None; for (cname, cinfo) in &self.classes { if let Some(&(off, ref t)) = cinfo.fields.get(&member) { found = Some((off, t.clone())); } }
                                       if let Some(res) = found { res } else { return self.error(format!("Field '{}' not found", member)); }
                                   }
                               };'''
)

content = content.replace(
'''                          } else if self.current_token == Token::Dot {
                               let offset = if let Type::Class(cname) = &typ {
                                   if Some(cname.clone()) == self.current_class_name {
                                       if let Some(off) = self.current_class_fields.get(&member) { *off } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
                                   } else if let Some(cinfo) = self.classes.get(cname) {
                                       if let Some(off) = cinfo.fields.get(&member) { *off } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                   } else { return self.error(format!("Unknown class '{}'", cname)); }
                               } else {
                                   if let Some(off) = self.current_class_fields.get(&member) {
                                       *off
                                   } else {
                                       let mut found = None; for (cname, cinfo) in &self.classes { if let Some(off) = cinfo.fields.get(&member) { found = Some(*off); } }
                                       if let Some(off) = found { off } else { return self.error(format!("Field '{}' not found", member)); }
                                   }
                               };
                               out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8));
                               typ = Type::Unknown;''',
'''                          } else if self.current_token == Token::Dot {
                               let (offset, field_typ) = if let Type::Class(cname) = &typ {
                                   if Some(cname.clone()) == self.current_class_name {
                                       if let Some(&(off, ref t)) = self.current_class_fields.get(&member) { (off, t.clone()) } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
                                   } else if let Some(cinfo) = self.classes.get(cname) {
                                       if let Some(&(off, ref t)) = cinfo.fields.get(&member) { (off, t.clone()) } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                   } else { return self.error(format!("Unknown class '{}'", cname)); }
                               } else {
                                   if let Some(&(off, ref t)) = self.current_class_fields.get(&member) {
                                       (off, t.clone())
                                   } else {
                                       let mut found = None; for (cname, cinfo) in &self.classes { if let Some(&(off, ref t)) = cinfo.fields.get(&member) { found = Some((off, t.clone())); } }
                                       if let Some(res) = found { res } else { return self.error(format!("Field '{}' not found", member)); }
                                   }
                               };
                               out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8));
                               typ = field_typ;'''
)

content = content.replace(
'''                         } else {
                             let offset = if let Type::Class(cname) = &typ {
                                  if Some(cname.clone()) == self.current_class_name {
                                      if let Some(f) = self.current_class_fields.get(&member) { *f } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
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
                             out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8)); typ = Type::Unknown;
                         }''',
'''                         } else {
                             let (offset, field_typ) = if let Type::Class(cname) = &typ {
                                  if Some(cname.clone()) == self.current_class_name {
                                      if let Some(&(f, ref t)) = self.current_class_fields.get(&member) { (f, t.clone()) } else { return self.error(format!("Field '{}' not found in current class '{}'", member, cname)); }
                                  } else if let Some(cinfo) = self.classes.get(cname) { 
                                      if let Some(&(f, ref t)) = cinfo.fields.get(&member) { (f, t.clone()) } else { return self.error(format!("Field '{}' not found in '{}'", member, cname)); }
                                  } else { return self.error(format!("Unknown class '{}'", cname)); }
                             } else {
                                  if let Some(&(off, ref t)) = self.current_class_fields.get(&member) {
                                      (off, t.clone())
                                  } else {
                                      let mut found = None; for (cname, cinfo) in &self.classes { if let Some(&(off, ref t)) = cinfo.fields.get(&member) { found = Some((off, t.clone())); } }
                                      if let Some(res) = found { res } else { return self.error(format!("Field '{}' not found", member)); }
                                  }
                             };
                             out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8)); typ = field_typ;
                         }'''
)

with open('src/compiler.rs', 'w') as f:
    f.write(content)

print("Patched compiler.rs successfully")
