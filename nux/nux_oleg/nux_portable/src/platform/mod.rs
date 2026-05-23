pub mod headless;
pub mod desktop;
pub mod ainux;

pub trait Platform {
    fn init(&mut self);
    fn create_window(&mut self, width: usize, height: usize, title: &str) -> Result<(), String>;
    fn update_window(&mut self, buffer: &[u32], width: usize, height: usize) -> Result<(), String>;
    fn list_cameras(&self) -> Vec<String>;
    fn capture_cam(&mut self, cam_id: usize) -> Option<(usize, usize, Vec<u32>)>;
    fn sys_info(&self) -> String;
    fn platform_type(&self) -> u8; // 0=Unk, 1=Linux, 2=Win, 3=Mac, 4=Ainux
    fn is_key_down(&self, key: usize) -> bool;
}
