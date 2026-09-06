import re

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    code = f.read()

bad_ffi_python = """0xE7 => { // OP_FFI_PYTHON
                    let code_ptr = self.stack.pop().unwrap();
                    let code_str = self.read_string(code_ptr);
                    let output = std::process::Command::new("python").arg("-c").arg(code_str).output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    let res_ptr = self.allocate_string(&res);
                    self.stack.push(res_ptr);
                },"""

bad_ffi_c = """0xE8 => { // OP_FFI_C
                    let code_ptr = self.stack.pop().unwrap();
                    let code_str = self.read_string(code_ptr);
                    std::fs::write(".tmp_inline.c", code_str).unwrap();
                    let _ = std::process::Command::new("gcc").arg(".tmp_inline.c").arg("-o").arg(".tmp_inline.exe").status();
                    let output = std::process::Command::new("./.tmp_inline.exe").output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    let res_ptr = self.allocate_string(&res);
                    self.stack.push(res_ptr);
                },"""

good_ffi_python = """0xE7 => { // OP_FFI_PYTHON
                    let code_ptr = self.stack.pop().unwrap() as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    let code_str = if code_ptr < heap_strings.len() { heap_strings[code_ptr].clone() } else { String::new() };
                    drop(heap_strings);
                    
                    let output = std::process::Command::new("python").arg("-c").arg(code_str).output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    
                    let mut heap_strings_mut = self.shared.heap_strings.write().unwrap();
                    heap_strings_mut.push(res);
                    self.stack.push((heap_strings_mut.len() - 1) as i64);
                },"""

good_ffi_c = """0xE8 => { // OP_FFI_C
                    let code_ptr = self.stack.pop().unwrap() as usize;
                    let heap_strings = self.shared.heap_strings.read().unwrap();
                    let code_str = if code_ptr < heap_strings.len() { heap_strings[code_ptr].clone() } else { String::new() };
                    drop(heap_strings);
                    
                    std::fs::write(".tmp_inline.c", code_str).unwrap();
                    let _ = std::process::Command::new("gcc").arg(".tmp_inline.c").arg("-o").arg(".tmp_inline.exe").status();
                    let output = std::process::Command::new("./.tmp_inline.exe").output();
                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                    
                    let mut heap_strings_mut = self.shared.heap_strings.write().unwrap();
                    heap_strings_mut.push(res);
                    self.stack.push((heap_strings_mut.len() - 1) as i64);
                },"""

code = code.replace(bad_ffi_python, good_ffi_python)
code = code.replace(bad_ffi_c, good_ffi_c)

with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(code)
print('Fixed VM string allocation')
