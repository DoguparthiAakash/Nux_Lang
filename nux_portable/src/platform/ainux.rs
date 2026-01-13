
use super::Platform;

pub struct AinuxPlatform;

impl AinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Platform for AinuxPlatform {
    fn init(&mut self) {
        println!("Ainux Platform Initialized");
    }

    fn create_window(&mut self, width: usize, height: usize, title: &str) -> Result<(), String> {
        // TODO: syscall_create_window
        println!("SYSCALL: create_window({}, {}, '{}')", width, height, title);
        Ok(())
    }

    fn update_window(&mut self, _buffer: &[u32], width: usize, height: usize) -> Result<(), String> {
        // TODO: syscall_draw_buffer
        // println!("SYSCALL: update_window buffer size {}", buffer.len());
        Ok(())
    }

    fn list_cameras(&self) -> Vec<String> {
        vec!["/dev/video0".to_string()]
    }

    fn capture_cam(&mut self, _cam_id: usize) -> Option<(usize, usize, Vec<u32>)> {
        // TODO: syscall_read_file("/dev/video0")
        Some((320, 240, vec![0xFFFFFFFF; 320*240])) // White screen
    }

    fn sys_info(&self) -> String {
        "Ainux Native".to_string()
    }

    fn platform_type(&self) -> u8 {
        4
    }
}
