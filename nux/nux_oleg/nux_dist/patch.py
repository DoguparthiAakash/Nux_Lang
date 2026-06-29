import sys
content = open('src/compiler.rs', 'r').read()
target = '''                _ => { sub_parser.advance(); }
            }
        }
    }'''

replacement = '''                _ => { sub_parser.advance(); }
            }
        }
        
        if !sub_parser.errors.is_empty() {
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
        }
    }'''

if target in content:
    content = content.replace(target, replacement)
    open('src/compiler.rs', 'w').write(content)
    print('Replaced successfully')
else:
    print('Target not found')
