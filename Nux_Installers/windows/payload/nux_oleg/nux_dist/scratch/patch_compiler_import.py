import sys

def patch_compiler():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()

    start_str = 'Token::Import => {'
    end_str = '} else {\n                        self.errors.push(CompileError::new(format!("Module \'{}\' not found", raw_name), self.prev_span));\n                    }\n                },'

    start_idx = content.find(start_str)
    end_idx = content.find(end_str)

    if start_idx == -1 or end_idx == -1:
        print("Could not find import logic string indices")
        return

    replace_str = r'''Token::Import => { 
                    self.advance();
                    let mut path_parts = Vec::new();
                    let mut valid = true;
                    loop {
                        match &self.current_token {
                            Token::Identifier(s) => {
                                path_parts.push(s.clone());
                                self.advance();
                                if self.current_token == Token::Dot {
                                    self.advance();
                                } else {
                                    self.errors.push(CompileError::new("Expected '.' after namespace".to_string(), self.current_span));
                                    valid = false;
                                    break;
                                }
                            },
                            Token::String(s) => {
                                path_parts.push(s.clone());
                                self.advance();
                                break;
                            },
                            _ => {
                                self.errors.push(CompileError::new("Expected import path string or namespace".to_string(), self.current_span));
                                valid = false;
                                break;
                            }
                        }
                    }
                    
                    if !valid {
                        self.synchronize();
                        continue;
                    }
                    
                    if self.current_token == Token::SemiColon { self.advance(); }
                    
                    let is_wildcard = path_parts.last().map(|s| s.as_str()) == Some("*");
                    let mut dir_path = String::new();
                    let mut file_path = String::new();
                    
                    if is_wildcard {
                        if path_parts.len() > 1 {
                            dir_path = path_parts[..path_parts.len()-1].join("/");
                        }
                    } else {
                        let raw_name = path_parts.join("/");
                        file_path = format!("{}.nux", raw_name.replace(".", "/"));
                    }
                    
                    let mut paths_to_load = Vec::new();
                    
                    if is_wildcard {
                        let mut search_dirs = Vec::new();
                        if let Ok(env_path) = std::env::var("NUX_LIB_PATH") {
                            search_dirs.push(std::path::Path::new(&env_path).join(&dir_path));
                        }
                        search_dirs.push(std::path::Path::new("lib").join(&dir_path));
                        
                        let mut found_any = false;
                        for dir in search_dirs {
                            if dir.exists() && dir.is_dir() {
                                if let Ok(entries) = std::fs::read_dir(dir) {
                                    for entry in entries.flatten() {
                                        if let Ok(file_type) = entry.file_type() {
                                            if file_type.is_file() {
                                                if let Some(ext) = entry.path().extension() {
                                                    if ext == "nux" {
                                                        paths_to_load.push(entry.path());
                                                        found_any = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !found_any {
                            self.errors.push(CompileError::new(format!("No modules found in namespace '{}'", dir_path), self.prev_span));
                        }
                    } else {
                        let mut found_path = None;
                        if let Ok(env_path) = std::env::var("NUX_LIB_PATH") {
                            let path = std::path::Path::new(&env_path).join(&file_path);
                            if path.exists() {
                                found_path = Some(path);
                            }
                        }
                        if found_path.is_none() {
                            let path = std::path::Path::new("lib").join(&file_path);
                            if path.exists() {
                                found_path = Some(path);
                            }
                        }
                        if let Some(p) = found_path {
                            paths_to_load.push(p);
                        } else {
                            self.errors.push(CompileError::new(format!("Module '{}' not found", file_path), self.prev_span));
                        }
                    }
                    
                    for p in paths_to_load {
                        if let Ok(content) = std::fs::read_to_string(&p) {
                            self.parse_imported_source(&content, &mut definitions);
                        }
                    }
                },'''

    content = content[:start_idx] + replace_str + content[end_idx + len(end_str):]
    print("Patched import logic")

    with open(path, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch_compiler()
