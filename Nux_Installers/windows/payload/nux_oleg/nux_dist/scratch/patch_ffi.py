import re

with open('src/assembler.rs', 'r', encoding='utf-8') as f:
    code = f.read()
code = code.replace('"OP_SPAWN" => ops.push(0xE0),', '"OP_SPAWN" => ops.push(0xE0),\n            "OP_FFI_PYTHON" => ops.push(0xE7),\n            "OP_FFI_C" => ops.push(0xE8),')
with open('src/assembler.rs', 'w', encoding='utf-8') as f:
    f.write(code)

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    code = f.read()
code = code.replace('0xE0 => {', '0xE7 => { // OP_FFI_PYTHON\n                    let code_ptr = self.stack.pop().unwrap();\n                    let code_str = self.read_string(code_ptr);\n                    let output = std::process::Command::new("python").arg("-c").arg(code_str).output();\n                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };\n                    let res_ptr = self.allocate_string(&res);\n                    self.stack.push(res_ptr);\n                },\n                0xE8 => { // OP_FFI_C\n                    let code_ptr = self.stack.pop().unwrap();\n                    let code_str = self.read_string(code_ptr);\n                    std::fs::write(".tmp_inline.c", code_str).unwrap();\n                    let _ = std::process::Command::new("gcc").arg(".tmp_inline.c").arg("-o").arg(".tmp_inline.exe").status();\n                    let output = std::process::Command::new("./.tmp_inline.exe").output();\n                    let res = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };\n                    let res_ptr = self.allocate_string(&res);\n                    self.stack.push(res_ptr);\n                },\n                0xE0 => {')
with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(code)

with open('src/compiler.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Add to parse_primary
new_primary = """            Token::Identifier(name) => {
                let name_clone = name.clone();
                self.advance();
                if name_clone == "python" && self.current_token == Token::LParen {
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token == Token::RParen { self.advance(); }
                    out.push_str("OP_FFI_PYTHON\\n");
                    return Ok(Type::String);
                } else if name_clone == "c" && self.current_token == Token::LParen {
                    self.advance();
                    self.parse_expression(out)?;
                    if self.current_token == Token::RParen { self.advance(); }
                    out.push_str("OP_FFI_C\\n");
                    return Ok(Type::String);
                }"""
code = code.replace('Token::Identifier(name) => {\n                let name_clone = name.clone();\n                self.advance();', new_primary)

with open('src/compiler.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print('Patched FFI')
