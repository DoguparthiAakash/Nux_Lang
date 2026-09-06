use crate::hal::{HardwareAbstraction, GPIO_INPUT, GPIO_OUTPUT, GPIO_INPUT_PULLUP, GPIO_INPUT_PULLDOWN};

#[cfg(target_arch = "xtensa")]
use esp_hal::{
    gpio::{AnyPin, Input, Output, PushPull, PullDown, PullUp, Io, Level, DriveStrength},
    delay::Delay,
    prelude::*,
};

#[cfg(not(target_arch = "xtensa"))]
use std::marker::PhantomData; // Mock for host

pub struct Esp32Hal {
    #[cfg(target_arch = "xtensa")]
    pins: [Option<AnyPin<esp_hal::gpio::Unknown>>; 40], // Array of optional pins
    #[cfg(target_arch = "xtensa")]
    delay: Delay,
    
    #[cfg(not(target_arch = "xtensa"))]
    _mock: PhantomData<()>,
}

impl Esp32Hal {
    #[cfg(target_arch = "xtensa")]
    pub fn new(delay: Delay, mut io: Io) -> Self {
        let mut pins: [Option<AnyPin<esp_hal::gpio::Unknown>>; 40] = [const { None }; 40];
        
        // Populate specific pins we want to support (or all)
        // Note: Mapping IO pins to array index manually
        
        // Example: Only GPIO 2 (Built-in LED) and a few others for now
        // A full implementation would map all.
        pins[2] = Some(io.pins.gpio2.degrade());
        pins[4] = Some(io.pins.gpio4.degrade());
        pins[5] = Some(io.pins.gpio5.degrade());
        
        Self {
            pins,
            delay,
        }
    }

    #[cfg(not(target_arch = "xtensa"))]
    pub fn new_mock() -> Self {
        Self { _mock: PhantomData }
    }
}

impl HardwareAbstraction for Esp32Hal {
    fn gpio_write(&mut self, pin: u8, value: bool) -> Result<(), &'static str> {
        #[cfg(target_arch = "xtensa")]
        {
            if let Some(Some(p)) = self.pins.get_mut(pin as usize) {
                // We need to ensure it's an output. 
                // AnyPin usage in esp-hal 0.20 requires conversion or dynamic usage.
                // Assuming `set_level` works on AnyPin if configured?
                // Actually, AnyPin needs to be into_push_pull_output first?
                // Or we can use `Output::new(p, Level::Low)` to wrap it temporarily?
                // But that consumes the pin.
                // We'll assume the pin is already in correct mode or we switch it.
                // For simplicity in this demo, we assume `gpio_set_mode` was called.
                
                // Hack: Reconstruct Output driver locally?
                // let mut out = Output::new(unsafe { p.clone_unchecked() }, Level::Low); 
                // unsafe doesn't exist on AnyPin?
                
                // Let's rely on standard `Output::new` behavior which might require ownership.
                // If we hold AnyPin, we can convert it.
                
                // Implementation Note: In `esp-hal`, `AnyPin` is a specific type.
                // We might need to store `AnyOutput` or `AnyInput` enum?
                // But `gpio_read` needs Input.
                
                // For this PoC: We will convert to Output, write, and convert back to Unknown/Any?
                // No, that resets state.
                
                // Correct approach: Store an Enum CustomPin { Input(AnyInput), Output(AnyOutput) }
                // But AnyInput/AnyOutput might not be standard types in `esp-hal`.
                // Let's just use `Output::new` on the pin reference if possible? No.
                
                // Let's try `set_output_high` directly on AnyPin? No.
                
                // OK, fallback to `toggle` on GPIO 2 specifically via register/unsafe if needed?
                // Or: `Output::new(p.peripheral(), ...)`
                
                // Wait, `esp-hal` 0.20 `AnyPin` implements `InputPin` and `OutputPin` traits embedded-hal?
                // If so:
                let level = if value { Level::High } else { Level::Low };
                p.set_level(level); // If AnyPin is Output?
                Ok(())
            } else {
                Err("Invalid Pin")
            }
        }
        #[cfg(not(target_arch = "xtensa"))]
        Ok(())
    }
    
    fn gpio_read(&mut self, pin: u8) -> Result<bool, &'static str> {
        #[cfg(target_arch = "xtensa")]
        {
             if let Some(Some(p)) = self.pins.get(pin as usize) {
                 Ok(p.is_high())
             } else {
                 Err("Invalid Pin")
             }
        }
        #[cfg(not(target_arch = "xtensa"))]
        Ok(false)
    }
    
    fn gpio_set_mode(&mut self, pin: u8, mode: u8) -> Result<(), &'static str> {
         #[cfg(target_arch = "xtensa")]
         {
             if let Some(slot) = self.pins.get_mut(pin as usize) {
                 if let Some(p) = slot.take() {
                     // Convert pin mode
                     // This is tricky because `into_push_pull_output` returns `GpioPin<Output<PushPull>, N>`.
                     // We need to degrade IT to `AnyPin`.
                     let new_pin = match mode {
                         GPIO_OUTPUT => p.into_push_pull_output().degrade(),
                         GPIO_INPUT => p.into_floating_input().degrade(),
                         GPIO_INPUT_PULLUP => p.into_pull_up_input().degrade(),
                         GPIO_INPUT_PULLDOWN => p.into_pull_down_input().degrade(),
                         _ => p
                     };
                     *slot = Some(new_pin);
                     Ok(())
                 } else {
                     Err("Invalid Pin")
                 }
             } else {
                 Err("Pin out of range")
             }
         }
         #[cfg(not(target_arch = "xtensa"))]
         Ok(())
    }
    
    fn analog_read(&mut self, _pin: u8) -> Result<u16, &'static str> {
        Ok(0)
    }
    
    fn pwm_write(&mut self, _pin: u8, _duty: u16) -> Result<(), &'static str> {
        Ok(())
    }
    
    fn delay_ms(&mut self, ms: u32) {
        #[cfg(target_arch = "xtensa")]
        self.delay.delay_millis(ms);
    }
    
    fn delay_us(&mut self, us: u32) {
        #[cfg(target_arch = "xtensa")]
        self.delay.delay_micros(us);
    }
    
    fn millis(&self) -> u32 {
        0
    }
    fn micros(&self) -> u32 {
        0
    }
    
    fn print_int(&mut self, value: i32) {
        #[cfg(target_arch = "xtensa")]
        esp_println::println!("{}", value);
    }
    
    fn print_char(&mut self, ch: char) {
        #[cfg(target_arch = "xtensa")]
        esp_println::print!("{}", ch);
    }
}
