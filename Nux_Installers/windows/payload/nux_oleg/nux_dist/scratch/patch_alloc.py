import sys

file_path = 'src/vm.rs'
with open(file_path, 'r') as f:
    content = f.read()

# Add heap_ptr to SharedVmState
content = content.replace(
    'pub memory: RwLock<Vec<u8>>,\n    pub heap_strings: RwLock<Vec<String>>',
    'pub memory: RwLock<Vec<u8>>,\n    pub heap_ptr: std::sync::atomic::AtomicUsize,\n    pub heap_strings: RwLock<Vec<String>>'
)

# Initialize heap_ptr in NuxVm::new
content = content.replace(
    'memory: RwLock::new(vec![0u8; 64 * 1024]),\n                heap_strings: RwLock::new(Vec::new()),',
    'memory: RwLock::new(vec![0u8; 64 * 1024]),\n                heap_ptr: std::sync::atomic::AtomicUsize::new(8),\n                heap_strings: RwLock::new(Vec::new()),'
)

# Fix OP_IMG_ALLOC
old_alloc = '''                0x31 => { // OP_IMG_ALLOC
                    let _h = self.stack.pop().unwrap() as usize;
                    let _w = self.stack.pop().unwrap() as usize;
                    self.stack.push(0); 
                },'''

new_alloc = '''                0x31 => { // OP_IMG_ALLOC
                    let h = self.stack.pop().unwrap() as usize;
                    let w = self.stack.pop().unwrap() as usize;
                    let bytes = w * h * 8; // Each field/pixel is 8 bytes
                    let ptr = self.shared.heap_ptr.fetch_add(bytes, std::sync::atomic::Ordering::SeqCst);
                    {
                        let mut memory = self.shared.memory.write().unwrap();
                        if ptr + bytes > memory.len() {
                            let new_len = (ptr + bytes).max(memory.len() * 2);
                            memory.resize(new_len, 0);
                        }
                    }
                    self.stack.push(ptr as i64);
                },'''

if old_alloc in content:
    content = content.replace(old_alloc, new_alloc)
else:
    print("Could not find OP_IMG_ALLOC to replace")

with open(file_path, 'w') as f:
    f.write(content)

print("Patched vm.rs")
