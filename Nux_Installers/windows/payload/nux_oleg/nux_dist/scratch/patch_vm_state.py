import os

vm_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\vm.rs"

with open(vm_path, "r", encoding="utf-8") as f:
    vm_data = f.read()

# Replace SharedVmState definition
old_struct = """pub struct SharedVmState {
    pub memory: RwLock<Vec<u8>>,
    pub heap_strings: RwLock<Vec<String>>,
    pub heap_arrays: RwLock<Vec<Vec<i64>>>,
    pub threads: Mutex<std::collections::HashMap<i64, std::thread::JoinHandle<i64>>>,
    pub next_thread_id: std::sync::atomic::AtomicI64,
}"""

new_struct = """pub struct SharedVmState {
    pub memory: RwLock<Vec<u8>>,
    pub heap_strings: RwLock<Vec<String>>,
    pub heap_arrays: RwLock<Vec<Vec<i64>>>,
    pub threads: Mutex<std::collections::HashMap<i64, std::thread::JoinHandle<i64>>>,
    pub next_thread_id: std::sync::atomic::AtomicI64,
    pub listeners: RwLock<Vec<std::net::TcpListener>>,
    pub connections: RwLock<Vec<std::net::TcpStream>>,
}"""

vm_data = vm_data.replace(old_struct, new_struct)

# Replace SharedVmState instantiation
old_init = """            shared: Arc::new(SharedVmState {
                memory: RwLock::new(vec![0u8; 64 * 1024]),
                heap_strings: RwLock::new(Vec::new()),
                heap_arrays: RwLock::new(Vec::new()),
                threads: Mutex::new(std::collections::HashMap::new()),
                next_thread_id: std::sync::atomic::AtomicI64::new(1),
            }),"""

new_init = """            shared: Arc::new(SharedVmState {
                memory: RwLock::new(vec![0u8; 64 * 1024]),
                heap_strings: RwLock::new(Vec::new()),
                heap_arrays: RwLock::new(Vec::new()),
                threads: Mutex::new(std::collections::HashMap::new()),
                next_thread_id: std::sync::atomic::AtomicI64::new(1),
                listeners: RwLock::new(Vec::new()),
                connections: RwLock::new(Vec::new()),
            }),"""

vm_data = vm_data.replace(old_init, new_init)

with open(vm_path, "w", encoding="utf-8") as f:
    f.write(vm_data)

print("SharedVmState patched")
