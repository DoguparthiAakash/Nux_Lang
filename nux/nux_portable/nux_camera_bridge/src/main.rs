use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use std::fs::File;
use std::io::{Seek, Write};
use std::thread;
use std::time::Duration;
use byteorder::{LittleEndian, WriteBytesExt};

const TARGET_WIDTH: u32 = 64;
const TARGET_HEIGHT: u32 = 48; // 4:3 Aspect roughly, fits in console
const BRIDGE_FILE: &str = "/tmp/nux_cam.bin";

fn main() {
    println!("--- Nux Camera Bridge ---");
    println!("Output: {}", BRIDGE_FILE);
    println!("Target: {}x{}", TARGET_WIDTH, TARGET_HEIGHT);

    let first_camera = CameraIndex::Index(0);
    
    // Request 640x480 or similar, we will resize manually if needed or let cam do it
    let format = RequestedFormat::new(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(first_camera, format).expect("Could not find camera! Is V4L2 installed?");
    camera.open_stream().expect("Could not open stream");

    println!("Camera Opened: {}", camera.info().human_name());
    
    let mut frame_counter = 0u32;

    loop {
        let frame = camera.frame().expect("Failed to capture frame");
        
        // Resize to target small resolution for Nux
        // Nux VM is not optimized for HD video yet.
        let resized = image::imageops::resize(
            &frame,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            image::imageops::FilterType::Nearest,
        );

        // Write to file
        // Format: [Width: u32][Height: u32][Counter: u32][Pixels: u32... (ARGB)]
        // We use a temporary buffer to write atomically-ish (OS buffers usually handle small writes well enough)
        // Or seek to 0.
        
        // Retry logic for file
        if let Ok(mut file) = File::create(BRIDGE_FILE) {
             let _ = file.write_u32::<LittleEndian>(TARGET_WIDTH);
             let _ = file.write_u32::<LittleEndian>(TARGET_HEIGHT);
             let _ = file.write_u32::<LittleEndian>(frame_counter);
             
             for pixel in resized.pixels() {
                 let r = pixel[0] as u32;
                 let g = pixel[1] as u32;
                 let b = pixel[2] as u32;
                 // Packed ARGB: 255 << 24 | r << 16 | g << 8 | b
                 let val = 0xFF000000 | (r << 16) | (g << 8) | b;
                 let _ = file.write_u32::<LittleEndian>(val);
             }
             let _ = file.flush();
        }

        frame_counter = frame_counter.wrapping_add(1);
        if frame_counter % 30 == 0 {
            println!("Captured {} frames...", frame_counter);
        }
        
        // Cap at ~30 FPS
        thread::sleep(Duration::from_millis(33));
    }
}
