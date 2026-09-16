import sys

with open('src/vm.rs', 'r', encoding='utf-8') as f:
    text = f.read()

# Add a trace buffer to NuxVm struct
text = text.replace(
    'pub struct NuxVm {',
    'pub struct NuxVm {\n    pub trace: std::collections::VecDeque<(usize, u8)>,'
)

text = text.replace(
    'stack: Vec::new(),\n            code,',
    'stack: Vec::new(),\n            trace: std::collections::VecDeque::new(),\n            code,'
)

# Record the trace before match op {
text = text.replace(
    'match op {',
    'self.trace.push_back((self.ip - 1, op));\n                if self.trace.len() > 100 { self.trace.pop_front(); }\n                match op {'
)

# Intercept memory allocation error by checking size in OP_TENSOR_NEW
text = text.replace(
    '0x82 => { // OP_TENSOR_NEW\nlet size = self.stack.pop().unwrap_or(0) as usize;',
    '0x82 => { // OP_TENSOR_NEW\nlet size = self.stack.pop().unwrap_or(0) as usize;\nif size > 1000000 { println!("HUGE ALLOC AT IP {}", self.ip - 1); for &(ip, o) in &self.trace { println!("Trace: ip {} op {:X}", ip, o); } }'
)

with open('src/vm.rs', 'w', encoding='utf-8') as f:
    f.write(text)
