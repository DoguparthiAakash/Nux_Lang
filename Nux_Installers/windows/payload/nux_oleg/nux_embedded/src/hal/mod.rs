/// Hardware Abstraction Layer trait
/// Platform-specific implementations provide GPIO, timing, etc.

pub trait HardwareAbstraction {
    // GPIO
    fn gpio_write(&mut self, pin: u8, value: bool) -> Result<(), &'static str>;
    fn gpio_read(&mut self, pin: u8) -> Result<bool, &'static str>;
    fn gpio_set_mode(&mut self, pin: u8, mode: u8) -> Result<(), &'static str>;
    
    // Analog
    fn analog_read(&mut self, pin: u8) -> Result<u16, &'static str>;
    fn pwm_write(&mut self, pin: u8, duty: u16) -> Result<(), &'static str>;
    
    // Timing
    fn delay_ms(&mut self, ms: u32);
    fn delay_us(&mut self, us: u32);
    fn millis(&self) -> u32;
    fn micros(&self) -> u32;
    
    // I/O
    fn print_int(&mut self, value: i32);
    fn print_char(&mut self, ch: char);
}

// GPIO modes
pub const GPIO_INPUT: u8 = 0;
pub const GPIO_OUTPUT: u8 = 1;
pub const GPIO_INPUT_PULLUP: u8 = 2;
pub const GPIO_INPUT_PULLDOWN: u8 = 3;

#[cfg(feature = "esp32")]
pub mod esp32;

pub use traits::*;

mod traits {
    pub use super::HardwareAbstraction;
    pub use super::{GPIO_INPUT, GPIO_OUTPUT, GPIO_INPUT_PULLUP, GPIO_INPUT_PULLDOWN};
}
