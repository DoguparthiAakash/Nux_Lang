
#[cfg(feature = "gui")]
use minifb::{Window, WindowOptions, Scale};
use super::Platform;

#[cfg(feature = "gui")]
pub struct DesktopPlatform {
    window: Option<Window>,
}

#[cfg(feature = "gui")]
impl DesktopPlatform {
    pub fn new() -> Self {
        Self { window: None }
    }
}

#[cfg(feature = "gui")]
impl Platform for DesktopPlatform {
    fn init(&mut self) {
    }

    fn create_window(&mut self, width: usize, height: usize, title: &str) -> Result<(), String> {
        let mut opts = WindowOptions::default();
        opts.scale = Scale::X2;
        
        match Window::new(title, width, height, opts) {
            Ok(mut win) => {
                win.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps
                self.window = Some(win);
                Ok(())
            },
            Err(e) => Err(format!("Minifb Error: {}", e))
        }
    }

    fn update_window(&mut self, buffer: &[u32], width: usize, height: usize) -> Result<(), String> {
        if let Some(win) = &mut self.window {
            if !win.is_open() { return Err("Window Closed".to_string()); }
            win.update_with_buffer(buffer, width, height).map_err(|e| format!("{}", e))
        } else {
            Err("Window not created".to_string())
        }
    }

    fn list_cameras(&self) -> Vec<String> {
        // TODO: Use rscam or nokhwa. For now return mock.
        vec!["Integrated Camera".to_string(), "USB Camera".to_string()]
    }

    fn capture_cam(&mut self, _cam_id: usize) -> Option<(usize, usize, Vec<u32>)> {
        // Return Test Pattern for now (Gradient)
        // 320x240
        let w = 320;
        let h = 240;
        let mut data = vec![0u32; w*h];
        for y in 0..h {
            for x in 0..w {
                // ARGB
                let r = (x * 255 / w) as u32;
                let g = (y * 255 / h) as u32;
                let b = 128;
                data[y*w + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
        Some((w, h, data))
    }

    fn sys_info(&self) -> String {
        #[cfg(target_os = "linux")]
        return "Linux Desktop".to_string();
        #[cfg(target_os = "windows")]
        return "Windows Desktop".to_string();
        #[cfg(target_os = "macos")]
        return "macOS Desktop".to_string();
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return "Unknown Desktop".to_string();
    }

    fn platform_type(&self) -> u8 {
        #[cfg(target_os = "linux")]
        return 1;
        #[cfg(target_os = "windows")]
        return 2;
        #[cfg(target_os = "macos")]
        return 3;
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return 0;
    }

    fn is_key_down(&self, _key: usize) -> bool {
        // Basic mapping or just return false for now
        false
    }
}


// Stub logic for when GUI feature is disabled but Desktop is selected?
// Actually we should just fallback to Headless at instantiation time if feature is missing.
// But to satisfy trait bound if struct is compiled out?
#[cfg(not(feature = "gui"))]
pub struct DesktopPlatform;

#[cfg(not(feature = "gui"))]
impl DesktopPlatform {
    pub fn new() -> Self { Self }
}

#[cfg(not(feature = "gui"))]
impl Platform for DesktopPlatform {
    fn init(&mut self) {}
    fn create_window(&mut self, _w: usize, _h: usize, _t: &str) -> Result<(), String> { Err("GUI Disabled".to_string()) }
    fn update_window(&mut self, _b: &[u32], _w: usize, _h: usize) -> Result<(), String> { Err("GUI Disabled".to_string()) }
    fn list_cameras(&self) -> Vec<String> { vec![] }
    fn capture_cam(&mut self, _id: usize) -> Option<(usize, usize, Vec<u32>)> { None }
    fn sys_info(&self) -> String { "Desktop (GUI Disabled)".to_string() }
    fn platform_type(&self) -> u8 { 0 }
    fn is_key_down(&self, _key: usize) -> bool { false }
}
