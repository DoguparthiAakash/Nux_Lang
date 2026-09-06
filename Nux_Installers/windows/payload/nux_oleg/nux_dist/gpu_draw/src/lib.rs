use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn draw_3d_object(args: *const i64, num_args: usize, _state: *const c_void) -> i64 {
    if num_args < 3 {
        return -1;
    }
    
    let args_slice = std::slice::from_raw_parts(args, num_args);
    let width = args_slice[0] as usize;
    let height = args_slice[1] as usize;
    let time = (args_slice[2] as f32) / 10.0;
    
    println!("\x1B[H"); // Reset cursor to top-left to animate nicely
    
    // Simulate CUDA thread blocks with CPU rendering loop
    let mut output = String::with_capacity(width * height + height);
    
    for y in 0..height {
        for x in 0..width {
            let mut u = (x as f32 * 2.0) / (width as f32) - 1.0;
            let v = (y as f32 * 2.0) / (height as f32) - 1.0;
            
            // Adjust aspect ratio for terminal characters (~1:2)
            u *= (width as f32 / height as f32) * 0.5;
            
            // Ray origin and direction
            let ro = (0.0, 0.0, -3.0);
            let mut rd = (u, v, 1.0);
            let rd_len = (rd.0 * rd.0 + rd.1 * rd.1 + rd.2 * rd.2).sqrt();
            rd.0 /= rd_len; rd.1 /= rd_len; rd.2 /= rd_len;
            
            // Sphere
            let cx = time.sin() * 1.5;
            let cy = (time * 0.5).cos() * 0.5;
            let cz = 0.0;
            let r = 1.0;
            
            // Ray-sphere intersection
            let oc = (ro.0 - cx, ro.1 - cy, ro.2 - cz);
            let b = 2.0 * (oc.0 * rd.0 + oc.1 * rd.1 + oc.2 * rd.2);
            let c = (oc.0 * oc.0 + oc.1 * oc.1 + oc.2 * oc.2) - r * r;
            let a = 1.0;
            
            let discriminant = b * b - 4.0 * a * c;
            
            let mut pixel = ' ';
            if discriminant > 0.0 {
                let t = (-b - discriminant.sqrt()) / (2.0 * a);
                if t > 0.0 {
                    let hit = (ro.0 + t * rd.0, ro.1 + t * rd.1, ro.2 + t * rd.2);
                    let n = ((hit.0 - cx) / r, (hit.1 - cy) / r, (hit.2 - cz) / r);
                    
                    let mut light = (-1.0_f32, -1.0_f32, -1.0_f32);
                    let l_len = (light.0 * light.0 + light.1 * light.1 + light.2 * light.2).sqrt();
                    light.0 /= l_len; light.1 /= l_len; light.2 /= l_len;
                    
                    let mut diffuse = n.0 * light.0 + n.1 * light.1 + n.2 * light.2;
                    if diffuse < 0.0 { diffuse = 0.0; }
                    
                    let intensity = (diffuse * 11.0) as usize;
                    let shades = ".,-~:;=!*#$@".as_bytes();
                    let idx = if intensity > 11 { 11 } else { intensity };
                    pixel = shades[idx] as char;
                }
            }
            output.push(pixel);
        }
        output.push('\n');
    }
    
    print!("{}", output);
    
    return 0;
}
