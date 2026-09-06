with open('src/vm.rs', 'r') as f:
    content = f.read()

replacement = """
            if self.stack.len() > 1000 {
                panic!("Stack overflow at ip {}, op {:X}", self.ip, op);
            }
"""

content = content.replace("let op = self.code[self.ip];", "let op = self.code[self.ip];\n" + replacement)

with open('src/vm.rs', 'w') as f:
    f.write(content)
