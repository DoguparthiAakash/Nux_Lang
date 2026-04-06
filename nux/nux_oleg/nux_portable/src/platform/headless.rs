
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
        // Clear screen and move cursor to top
        print!("\x1b[2J\x1b[H");
        
        println!("╔{}╗", "═".repeat(width.min(80)));
        
        // Better ASCII character mapping (more gradients)
        let chars = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
        
        let step_y = if height > 40 { height / 40 } else { 1 };
        let step_x = if width > 80 { width / 80 } else { 1 };

        for y in (0..height).step_by(step_y) {
            print!("║");
            for x in (0..width).step_by(step_x) {
                let pixel = buffer[y * width + x];
                
                // Extract RGB
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                
                // Calculate brightness
                let brightness = ((r as usize + g as usize + b as usize) / 3);
                let char_idx = (brightness * (chars.len() - 1)) / 255;
                
                // Use ANSI color codes for better visualization
                if r > 200 && g < 100 && b < 100 {
                    // Red
                    print!("\x1b[31m{}\x1b[0m", chars[char_idx] as char);
                } else if g > 200 && r < 100 && b < 100 {
                    // Green
                    print!("\x1b[32m{}\x1b[0m", chars[char_idx] as char);
                } else if b > 200 && r < 100 && g < 100 {
                    // Blue
                    print!("\x1b[34m{}\x1b[0m", chars[char_idx] as char);
                } else if brightness > 200 {
                    // White/bright
                    print!("\x1b[37;1m{}\x1b[0m", chars[char_idx] as char);
                } else {
                    // Default
                    print!("{}", chars[char_idx] as char);
                }
            }
            println!("║");
        }
        
        println!("╚{}╝", "═".repeat(width.min(80)));
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

    fn is_key_down(&self, _key: usize) -> bool {
        false
    }
}
