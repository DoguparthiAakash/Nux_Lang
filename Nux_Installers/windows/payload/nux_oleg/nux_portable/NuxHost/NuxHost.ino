#include <Arduino.h>

// Map Nux Micro VM hooks to Arduino hardware API
#define NUX_PIN_MODE(pin, mode) pinMode(pin, (mode == 1) ? OUTPUT : INPUT)
#define NUX_DIGITAL_WRITE(pin, val) digitalWrite(pin, val)
#define NUX_DELAY_MS(ms) delay(ms)
#define NUX_PUTC(c) Serial.write(c)
#define NUX_HALT() while(1) { yield(); }

// Include the Nux byte-code and interpreter
#include "test_blink.micro.h"

void setup() {
    Serial.begin(115200);
    delay(1000);
    Serial.println("Starting Nux Micro VM on ESP32...");
}

void loop() {
    // Execute Nux Program
    nux_run();
    
    // If the program finishes, halt here
    while(1) { yield(); }
}
