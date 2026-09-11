pub mod windows;
pub mod linux;

pub trait OSMachine {
    /// Initialize the OS-specific hardware layer
    fn init(&mut self);
    
    /// Allocate memory natively using the OS allocator
    fn sys_alloc(&mut self, size: usize) -> usize;
    
    /// Free memory using the OS allocator
    fn sys_free(&mut self, ptr: usize);
    
    /// Write data to a file descriptor (like stdout)
    fn sys_write(&mut self, fd: i32, ptr: usize, len: usize);
    
    /// JIT Execute a memory region directly on the CPU hardware
    fn jit_execute(&mut self, memory_region: &[u8]);
}

/// Returns the correct OSMachine implementation for the current platform
pub fn get_machine() -> Box<dyn OSMachine> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsMachine::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxMachine::new())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        panic!("Unsupported OS for hardware machine layer");
    }
}
