import os
from PIL import Image

SIZES = [16, 22, 24, 32, 48, 64, 128, 256, 512]
SRC = "logo.png"

if not os.path.exists(SRC):
    print(f"Error: {SRC} not found")
    exit(1)

img = Image.open(SRC)

for size in SIZES:
    target_name = f"logo_{size}x{size}.png"
    print(f"Generating {target_name}...")
    resized = img.resize((size, size), Image.LANCZOS)
    resized.save(target_name)

print("Icons resized successfully.")
