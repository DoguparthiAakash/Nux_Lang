import os
from PIL import Image, ImageEnhance

def create_raw_icons(logo_path):
    if not os.path.exists(logo_path):
        print(f"File not found: {logo_path}")
        return
        
    try:
        # Load the original logo
        logo = Image.open(logo_path).convert("RGBA")
        
        # Save as nux_file_icon.png (exact copy but ensured to be PNG)
        png_path = "nux_file_icon.png"
        logo.save(png_path, "PNG")
        
        # Save as nux_file_icon.ico
        ico_path = "nux_file_icon.ico"
        logo.save(ico_path, format="ICO", sizes=[(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)])
        
        # Create nuxc (compiled) version - make it grayscale or slightly darker
        # Convert to grayscale to differentiate the compiled binary
        gray_logo = logo.convert("LA").convert("RGBA")
        
        # Or maybe just darken it? Let's darken the original slightly
        enhancer = ImageEnhance.Brightness(logo)
        dark_logo = enhancer.enhance(0.5)
        
        pngc_path = "nuxc_file_icon.png"
        gray_logo.save(pngc_path, "PNG")
        
        icoc_path = "nuxc_file_icon.ico"
        gray_logo.save(icoc_path, format="ICO", sizes=[(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)])
        
        print(f"Created {png_path}, {ico_path}, {pngc_path}, {icoc_path} from raw logo.")
    except Exception as e:
        print(f"Error processing logo: {e}")

if __name__ == "__main__":
    create_raw_icons("logo.png")
