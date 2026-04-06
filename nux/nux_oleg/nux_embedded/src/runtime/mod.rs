// Runtime initialization and entry point

pub fn init() {
    // Platform-specific initialization
}

pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // Custom panic handler for embedded
    loop {}
}
