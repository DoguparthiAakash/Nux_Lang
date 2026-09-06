use std::time::Duration;

/// Deploy a compiled Nux binary payload to a hardware device over serial.
/// Requires the `serial` feature: cargo build --features serial
pub fn deploy_to_port(port_name: &str, binary_payload: &[u8]) -> Result<(), String> {
    println!("Connecting to {}...", port_name);

    #[cfg(feature = "serial")]
    {
        let mut port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(5000))
            .open()
            .map_err(|e| format!("Failed to open serial port: {}", e))?;

        println!("Port opened. Flashing {} bytes...", binary_payload.len());

        // Synchronize with bootloader
        use std::io::Write;
        port.write_all(b"NUX_SYNC").map_err(|e| e.to_string())?;

        // Send payload in 256-byte chunks
        for chunk in binary_payload.chunks(256) {
            port.write_all(chunk).map_err(|e| e.to_string())?;
        }

        println!("Flash complete!");
        Ok(())
    }

    #[cfg(not(feature = "serial"))]
    {
        // Silence unused-variable warnings when serial feature is off
        let _ = (port_name, binary_payload, Duration::from_millis(0));
        Err(
            "Serial deploy is not enabled. Rebuild Nux with:\n  cargo build --release --features serial\n(requires libudev on Linux)"
                .to_string(),
        )
    }
}
