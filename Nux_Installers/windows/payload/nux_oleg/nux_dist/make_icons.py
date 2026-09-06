import os
from PIL import Image, ImageDraw, ImageEnhance

def create_document_icon(logo_path, output_name, is_dark=False):
    size = 256
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Document coordinates
    doc_width = 180
    doc_height = 220
    doc_x = (size - doc_width) // 2
    doc_y = (size - doc_height) // 2

    fold_size = 50

    # Colors
    if is_dark:
        doc_color = (40, 44, 52, 255)
        fold_color = (30, 33, 39, 255)
        outline_color = (80, 80, 80, 255)
        line_color = (100, 100, 100, 255)
    else:
        doc_color = (255, 255, 255, 255)
        fold_color = (240, 240, 240, 255)
        outline_color = (200, 200, 200, 255)
        line_color = (200, 200, 200, 255)

    # Draw shadow
    shadow_offset = 6
    shadow_color = (0, 0, 0, 60)
    shadow_poly = [
        (doc_x + shadow_offset, doc_y + shadow_offset),
        (doc_x + doc_width - fold_size + shadow_offset, doc_y + shadow_offset),
        (doc_x + doc_width + shadow_offset, doc_y + fold_size + shadow_offset),
        (doc_x + doc_width + shadow_offset, doc_y + doc_height + shadow_offset),
        (doc_x + shadow_offset, doc_y + doc_height + shadow_offset)
    ]
    draw.polygon(shadow_poly, fill=shadow_color)

    # Draw document body
    doc_poly = [
        (doc_x, doc_y),
        (doc_x + doc_width - fold_size, doc_y),
        (doc_x + doc_width, doc_y + fold_size),
        (doc_x + doc_width, doc_y + doc_height),
        (doc_x, doc_y + doc_height)
    ]
    draw.polygon(doc_poly, fill=doc_color, outline=outline_color, width=2)

    # Draw folded corner
    fold_poly = [
        (doc_x + doc_width - fold_size, doc_y),
        (doc_x + doc_width - fold_size, doc_y + fold_size),
        (doc_x + doc_width, doc_y + fold_size)
    ]
    draw.polygon(fold_poly, fill=fold_color, outline=outline_color, width=2)
    # subtle fold line
    draw.line([(doc_x + doc_width - fold_size, doc_y), (doc_x + doc_width, doc_y + fold_size)], fill=outline_color, width=2)

    # Draw lines on the document to make it look like code/binary
    line_x = doc_x + 30
    line_y = doc_y + 40
    line_width = doc_width - 60
    
    if is_dark: # Binary style lines
        draw.line([(line_x, line_y), (line_x + 40, line_y)], fill=line_color, width=4)
        draw.line([(line_x + 50, line_y), (line_x + line_width, line_y)], fill=line_color, width=4)
        draw.line([(line_x, line_y + 15), (line_x + line_width, line_y + 15)], fill=line_color, width=4)
        draw.line([(line_x, line_y + 30), (line_x + 60, line_y + 30)], fill=line_color, width=4)
    else: # Code style lines
        draw.line([(line_x, line_y), (line_x + line_width - 20, line_y)], fill=line_color, width=4)
        draw.line([(line_x, line_y + 15), (line_x + line_width, line_y + 15)], fill=line_color, width=4)
        draw.line([(line_x, line_y + 30), (line_x + line_width - 40, line_y + 30)], fill=line_color, width=4)

    # Load logo
    if os.path.exists(logo_path):
        try:
            logo = Image.open(logo_path).convert("RGBA")
            logo_size = 120
            logo = logo.resize((logo_size, logo_size), Image.Resampling.LANCZOS)
            
            if is_dark:
                # Darken the logo slightly for the compiled file
                enhancer = ImageEnhance.Brightness(logo)
                logo = enhancer.enhance(0.7)
            
            # Position logo in center/bottom half
            logo_x = (size - logo_size) // 2
            logo_y = doc_y + (doc_height - logo_size) // 2 + 20
            
            img.alpha_composite(logo, (logo_x, logo_y))
        except Exception as e:
            print(f"Error loading logo: {e}")

    # Save as PNG
    png_path = f"{output_name}.png"
    img.save(png_path, "PNG")
    
    # Save as ICO
    ico_path = f"{output_name}.ico"
    img.save(ico_path, format="ICO", sizes=[(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)])
    print(f"Created {png_path} and {ico_path}")

if __name__ == "__main__":
    create_document_icon("logo.png", "nux_file_icon", is_dark=False)
    create_document_icon("logo.png", "nuxc_file_icon", is_dark=True)
