import struct
import time
import os

BRIDGE_FILE = "/tmp/nux_cam.bin"
WIDTH = 32
HEIGHT = 16

def run():
    print(f"Mock Camera Bridge Running... writing to {BRIDGE_FILE}")
    print(f"Target: {WIDTH}x{HEIGHT}")
    
    frame_counter = 0
    
    while True:
        # Generate a solid color cycle: Red -> Green -> Blue
        mode = (frame_counter // 10) % 3
        
        pixels = []
        for i in range(WIDTH * HEIGHT):
            if mode == 0:
                val = 0xFFFF0000 # Red (ARGB) - Python ints need care
            elif mode == 1:
                val = 0xFF00FF00 # Green
            else:
                val = 0xFF0000FF # Blue
            pixels.append(val)
            
        # Write binary
        with open(BRIDGE_FILE, "wb") as f:
            # Struct format: I (u32)
            # Width, Height, Counter
            f.write(struct.pack("<III", WIDTH, HEIGHT, frame_counter))
            # Pixels
            for p in pixels:
                f.write(struct.pack("<I", p))
                
        frame_counter += 1
        time.sleep(0.1)

if __name__ == "__main__":
    try:
        run()
    except KeyboardInterrupt:
        print("Stopped.")
