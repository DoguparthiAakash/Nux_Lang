import sys

path = r'e:\nux\Nux_Lang\nux\nux_oleg\nux_portable\src\high_level.rs'
with open(path, 'r', encoding='utf-8') as f:
    code = f.read()

target1 = '''                        if let Some(opcode) = self.get_intrinsic(&part1) {
                            // Found intrinsic opcode
                            out.push_str(&format!("{}\\n", opcode));
                        } else {
                            // No intrinsic matched
                            out.push_str(&format!("CALL {} {}\\n", part1, arg_count));
                        }'''

repl1 = '''                        if let Some(opcode) = self.get_intrinsic(&part1) {
                            // Found intrinsic opcode
                            out.push_str(&format!("{}\\n", opcode));
                        } else if let Some(cinfo) = self.classes.get(&part1).cloned() {
                            out.push_str(&format!("PUSH {}\\n", cinfo.size));
                            out.push_str("OP_ALLOC\\n");
                        } else {
                            // No intrinsic matched
                            out.push_str(&format!("CALL {} {}\\n", part1, arg_count));
                        }'''

code = code.replace(target1, repl1)
code = code.replace(target1.replace('\n', '\r\n'), repl1)

target2 = '''               // Intrinsics managed via parse_call_args (statement context) are followed by POP.
               // Ensure all intrinsics push a value (even if dummy) to allow POP.
          } else {
               out.push_str(&format!("CALL {} {}\\n", func_name, arg_count));
          }'''

repl2 = '''               // Intrinsics managed via parse_call_args (statement context) are followed by POP.
               // Ensure all intrinsics push a value (even if dummy) to allow POP.
          } else if let Some(cinfo) = self.classes.get(func_name).cloned() {
               out.push_str(&format!("PUSH {}\\n", cinfo.size));
               out.push_str("OP_ALLOC\\n");
          } else {
               out.push_str(&format!("CALL {} {}\\n", func_name, arg_count));
          }'''

code = code.replace(target2, repl2)
code = code.replace(target2.replace('\n', '\r\n'), repl2)

target3 = '''        // Program entry point
        self.emit("__start_execution:");
        if !main_body.trim().is_empty() {
            self.emit("CALL __main 0");
            self.emit("POP ; Discard __main return value");
        }
        self.emit("EXIT");
        
        Ok(self.asm_output.clone())'''

repl3 = '''        // Program entry point
        self.emit("__start_execution:");
        if !main_body.trim().is_empty() {
            self.emit("CALL __main 0");
            self.emit("POP ; Discard __main return value");
        }
        self.emit("EXIT");
        
        eprintln!("END OF PARSING: classes={:?}", self.classes.keys());
        Ok(self.asm_output.clone())'''

code = code.replace(target3, repl3)
code = code.replace(target3.replace('\n', '\r\n'), repl3)

target_dbg1 = '''                        } else if let Some(cinfo) = self.classes.get(&part1).cloned() {'''
repl_dbg1 = '''                        } else if let Some(cinfo) = { eprintln!("DEBUG part1 check: {}, classes={:?}", part1, self.classes.keys()); self.classes.get(&part1).cloned() } {'''
code = code.replace(target_dbg1, repl_dbg1)
code = code.replace(target_dbg1.replace('\n', '\r\n'), repl_dbg1)

with open(path, 'w', encoding='utf-8') as f:
    f.write(code)
print("Patched.")
