import re

with open("src/vm.rs", "r") as f:
    content = f.read()

python_impl = """
                0xE7 => {
                    // OP_FFI_PYTHON
                    if let Some(code_val) = self.stack.pop() {
                        if let Some(code_str) = self.read_string(code_val) {
                            use std::process::Command;
                            match Command::new("python").arg("-c").arg(&code_str).output() {
                                Ok(output) => {
                                    if output.status.success() {
                                        let out_str = String::from_utf8_lossy(&output.stdout);
                                        // Push result back as string (using allocate_string)
                                        let addr = self.allocate_string(&out_str);
                                        self.stack.push(addr as i32);
                                    } else {
                                        let err_str = String::from_utf8_lossy(&output.stderr);
                                        println!("Python Error: {}", err_str);
                                        self.stack.push(0);
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to execute python: {}", e);
                                    self.stack.push(0);
                                }
                            }
                        } else {
                            self.stack.push(0);
                        }
                    } else {
                        self.stack.push(0);
                    }
                },
"""

c_impl = """
                0xE8 => {
                    // OP_FFI_C
                    if let Some(code_val) = self.stack.pop() {
                        if let Some(code_str) = self.read_string(code_val) {
                            use std::process::Command;
                            use std::fs;
                            use std::path::Path;
                            
                            let c_file = "temp_ffi.c";
                            let exe_file = if cfg!(windows) { "temp_ffi.exe" } else { "./temp_ffi" };
                            
                            // Wrap code in main if it doesn't contain main
                            let full_code = if code_str.contains("main") {
                                code_str.clone()
                            } else {
                                format!("#include <stdio.h>\\n#include <stdlib.h>\\nint main() {{ {} return 0; }}", code_str)
                            };
                            
                            fs::write(c_file, full_code).unwrap_or(());
                            
                            // Compile
                            match Command::new("gcc").arg(c_file).arg("-o").arg(exe_file).output() {
                                Ok(output) => {
                                    if output.status.success() {
                                        // Execute
                                        match Command::new(exe_file).output() {
                                            Ok(run_out) => {
                                                let out_str = String::from_utf8_lossy(&run_out.stdout);
                                                let addr = self.allocate_string(&out_str);
                                                self.stack.push(addr as i32);
                                            },
                                            Err(_) => { self.stack.push(0); }
                                        }
                                    } else {
                                        println!("C Compilation Error: {}", String::from_utf8_lossy(&output.stderr));
                                        self.stack.push(0);
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to compile C code: {}", e);
                                    self.stack.push(0);
                                }
                            }
                            // Cleanup
                            let _ = fs::remove_file(c_file);
                            let _ = fs::remove_file(if cfg!(windows) { "temp_ffi.exe" } else { "temp_ffi" });
                        } else {
                            self.stack.push(0);
                        }
                    } else {
                        self.stack.push(0);
                    }
                },
"""

# Replace the unimplemented blocks
search_py = """                0xE7 => {
                    // OP_FFI_PYTHON
                    // Unimplemented in prototype
                },"""
content = content.replace(search_py, python_impl.strip("\n"))

search_c = """                0xE8 => {
                    // OP_FFI_C
                    // Unimplemented in prototype
                },"""
content = content.replace(search_c, c_impl.strip("\n"))

with open("src/vm.rs", "w") as f:
    f.write(content)

print("vm.rs patched successfully")
