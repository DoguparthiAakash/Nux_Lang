from PIL import Image
import sys

# Block characters for "thick" art
# We want to map dark pixels to solid blocks
# 0 (black/dark) -> █ 
# 255 (white/light) -> space
ASCII_CHARS = ["█", "▓", "▒", "░", " ", " "]

def resize_image(image, new_width=60):
    width, height = image.size
    ratio = height / width / 1.8 
    new_height = int(new_width * ratio)
    resized_image = image.resize((new_width, new_height))
    return resized_image

def to_ascii(image):
    if image.mode in ('RGBA', 'LA'):
        background = Image.new(image.mode[:-1], image.size, (255, 255, 255))
        background.paste(image, image.split()[-1])
        image = background
    
    image = image.convert("L")
    
    # Increase contrast
    from PIL import ImageEnhance
    enhancer = ImageEnhance.Contrast(image)
    image = enhancer.enhance(2.0)
    
    pixels = image.getdata()
    
    ascii_str = ""
    for pixel in pixels:
        # Bias towards dark blocks
        # 0..255. We want 0..~100 to range through the blocks rapidly
        # and >200 to be space.
        
        if pixel > 200:
            ascii_str += " "
        elif pixel > 150:
            ascii_str += "░"
        elif pixel > 100:
            ascii_str += "▒"
        elif pixel > 50:
            ascii_str += "▓"
        else:
            ascii_str += "█"
            
    return ascii_str

def main(new_width=50):
    try:
        image = Image.open("nux_dist/logo.png")
    except Exception:
        print("Error: logo.png not found")
        return

    resized = resize_image(image, new_width)
    ascii_str = to_ascii(resized)
    
    pixel_count = len(ascii_str)
    ascii_img = "\n".join([ascii_str[index:(index+new_width)] for index in range(0, pixel_count, new_width)])
    
    # Filter and center
    clean_lines = [line for line in ascii_img.split('\n') if line.strip()] 
    if clean_lines:
        print("\n".join(clean_lines))

if __name__ == "__main__":
    width = 50
    if len(sys.argv) > 1:
        width = int(sys.argv[1])
    main(width)
