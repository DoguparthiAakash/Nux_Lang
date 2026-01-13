
use super::Platform;

pub struct HeadlessPlatform;

impl HeadlessPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Platform for HeadlessPlatform {
    fn init(&mut self) {
        // No-op
    }

    fn create_window(&mut self, width: usize, height: usize, title: &str) -> Result<(), String> {
        println!("Helper: Virtual Window '{}' created ({}x{}). Output will be ASCII.", title, width, height);
        Ok(())
    }

    fn update_window(&mut self, buffer: &[u32], width: usize, height: usize) -> Result<(), String> {
        // ASCII Logic
        println!("--- Image Dump ({}x{}) ---", width, height);
        let chars = b" .:-=+*#%@";
        // Simple downscaling/sampling for console? Or full dump?
        // Let's print full for small images, skip lines for big?
        
        let step_y = if height > 64 { height / 32 } else { 1 };
        let step_x = if width > 64 { width / 32 } else { 1 };

        for y in (0..height).step_by(step_y) {
            for x in (0..width).step_by(step_x) {
                let pixel = buffer[y * width + x];
                // Extract Blue channel (or average) as brightness for grayscale input
                // ARGB: 0xAARRGGBB.
                let val = (pixel & 0xFF) as usize; 
                let char_idx = (val * (chars.len() - 1)) / 255;
                print!("{}", chars[char_idx] as char);
            }
            println!("");
        }
        println!("-------------------------");
        Ok(())
    }

    fn list_cameras(&self) -> Vec<String> {
        vec!["Virtual ASCII Cam".to_string()]
    }

    fn capture_cam(&mut self, _cam_id: usize) -> Option<(usize, usize, Vec<u32>)> {
        // Return dummy static noise or gradient (320x240)
        let w = 320;
        let h = 240;
        let mut buffer = vec![0u32; w * h];
        for i in 0..buffer.len() {
            buffer[i] = 0xFF808080; // Gray
        }
        Some((w, h, buffer))
    }

    fn sys_info(&self) -> String {
        "Headless / Safe Fallback".to_string()
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
}
