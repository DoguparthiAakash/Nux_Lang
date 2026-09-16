import sys

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    text = f.read()

text = text.replace(
    '0x82 => { // OP_TENSOR_NEW\nlet size = self.stack.pop().unwrap_or(0) as usize;',
    '0x82 => { // OP_TENSOR_NEW\nlet size_val = self.stack.pop().unwrap_or(0);\nprintln!("DEBUG OP_TENSOR_NEW: popped {}", size_val);\nlet size = size_val as usize;'
)

with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(text)
