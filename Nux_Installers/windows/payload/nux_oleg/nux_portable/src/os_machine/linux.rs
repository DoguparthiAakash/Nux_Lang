use super::OSMachine;

pub struct LinuxMachine {
    // Linux-specific hardware state
}

impl LinuxMachine {
    pub fn new() -> Self {
        LinuxMachine {}
    }
}

impl OSMachine for LinuxMachine {
    fn init(&mut self) {
        println!("[Linux HAL] Initializing Linux OS Machine Layer...");
    }

    fn sys_alloc(&mut self, size: usize) -> usize {
        // Here we will eventually bind to mmap/sbrk
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        unsafe { std::alloc::alloc(layout) as usize }
    }

    fn sys_free(&mut self, ptr: usize) {
        // Eventually munmap
        println!("[Linux HAL] Freeing pointer: {:#X}", ptr);
    }

    fn sys_write(&mut self, _fd: i32, ptr: usize, len: usize) {
        // Bridge to write syscall
        unsafe {
            let slice = std::slice::from_raw_parts(ptr as *const u8, len);
            let s = std::str::from_utf8_unchecked(slice);
            print!("{}", s);
        }
    }

    fn jit_execute(&mut self, _memory_region: &[u8]) {
        println!("[Linux HAL] Executing JIT region using mprotect PROT_EXEC...");
    }
}
