import sys

content = open('src/compiler.rs', 'r', encoding='utf-8').read()

patch = """
                  Token::Class => {
                      if let Err(e) = sub_parser.parse_class(definitions) {
                           eprintln!("Import Parse Error (Class): {:?}", e);
                           break;
                      }
                      for err in &sub_parser.errors {
                          eprintln!("Import Error: {:?}", err);
                      }
                  },
                  Token::Func => {
                      if let Err(e) = sub_parser.parse_func(definitions, "") {
                           eprintln!("Import Parse Error (Func): {:?}", e);
                           break;
                      }
                      for err in &sub_parser.errors {
                          eprintln!("Import Error: {:?}", err);
                      }
                  },
"""

# Find the block we want to replace
target = """                  Token::Class => {
                      if let Err(e) = sub_parser.parse_class(definitions) {
                           eprintln!("Import Parse Error (Class): {:?}", e);
                           break;
                      }
                  },
                  Token::Func => {
                      if let Err(e) = sub_parser.parse_func(definitions, "") {
                           eprintln!("Import Parse Error (Func): {:?}", e);
                           break;
                      }
                  },"""

content = content.replace(target, patch)

open('src/compiler.rs', 'w', encoding='utf-8').write(content)
