import sys

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    text = f.read()

# Intercept PEEK memory allocation error
old_peek = """0x40 => { // PEEK
                    let addr = self.stack.pop().unwrap() as usize;
                    let mut memory = self.shared.memory.write().unwrap();
                    if addr + 8 > memory.len() {
                        let new_len = (addr + 8).max(memory.len() * 2);
                        memory.resize(new_len, 0);
                    }"""

new_peek = """0x40 => { // PEEK
                    let addr = self.stack.pop().unwrap() as usize;
                    if addr > 1000000 {
                        println!("HUGE PEEK ADDR {} AT IP {}", addr, self.ip - 1);
                        println!("Top of stack before pop: {:?}", self.stack.last());
                    }
                    let mut memory = self.shared.memory.write().unwrap();
                    if addr + 8 > memory.len() {
                        let new_len = (addr + 8).max(memory.len() * 2);
                        memory.resize(new_len, 0);
                    }"""

text = text.replace(old_peek, new_peek)

with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(text)
