use minifb::{Window, WindowOptions};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::cell::RefCell;

struct UiContext {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

thread_local! {
    static CONTEXT: RefCell<Option<UiContext>> = RefCell::new(None);
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_window_create(title_ptr: i64, width: i64, height: i64) -> i64 {
    let title = if title_ptr == 0 {
        "Nux UI Window".to_string()
    } else {
        unsafe {
            CStr::from_ptr(title_ptr as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let mut window = Window::new(
        &title,
        width as usize,
        height as usize,
        WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let buffer = vec![0; (width * height) as usize];

    CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(UiContext {
            window,
            buffer,
            width: width as usize,
            height: height as usize,
        });
    });
    
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_window_update() -> i64 {
    CONTEXT.with(|ctx| {
        if let Some(c) = ctx.borrow_mut().as_mut() {
            if c.window.is_open() && !c.window.is_key_down(minifb::Key::Escape) {
                c.window.update_with_buffer(&c.buffer, c.width, c.height).unwrap();
                return 1;
            }
        }
        0
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_draw_rect(x: i64, y: i64, w: i64, h: i64, color: i64) -> i64 {
    CONTEXT.with(|ctx| {
        if let Some(c) = ctx.borrow_mut().as_mut() {
            let color_u32 = color as u32;
            for r in y..(y+h) {
                for col in x..(x+w) {
                    if r >= 0 && (r as usize) < c.height && col >= 0 && (col as usize) < c.width {
                        c.buffer[(r as usize) * c.width + (col as usize)] = color_u32;
                    }
                }
            }
            return 1;
        }
        0
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_window_close() -> i64 {
    CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = None;
    });
    1
}
