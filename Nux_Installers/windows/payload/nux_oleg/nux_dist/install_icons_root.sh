#!/bin/bash
set -e

if [ "$EUID" -ne 0 ]; then 
  echo "Please run as root (sudo ./install_icons_root.sh)"
  exit 1
fi

# Define paths (System-wide)
MIME_DIR="/usr/share/mime/packages"
ICON_BASE="/usr/share/icons/hicolor"

# Python resize script needs to run as regular user if PIL is not installed for root,
# but usually it's better to just run it here. If it fails, we assume icons are already resized by previous run.
if [ -f "resize_icons.py" ]; then
    echo "Resizing icons..."
    # Try running with python3. If it fails (e.g. missing PIL for root), warn but continue if files exist
    python3 resize_icons.py || echo "Warning: Icon resize failed (missing PIL?). Using existing resized images if available."
fi

# Install MIME type
echo "Installing MIME type definition to $MIME_DIR..."
cp nux.xml "$MIME_DIR/"

# Install Icons
SIZES=("16x16" "22x22" "24x24" "32x32" "48x48" "64x64" "128x128" "256x256" "512x512")

echo "Installing icons to $ICON_BASE..."
for SIZE in "${SIZES[@]}"; do
    TARGET_DIR="$ICON_BASE/$SIZE/mimetypes"
    # Ensure dir exists (it should on standard systems)
    if [ -d "$ICON_BASE/$SIZE" ]; then
        mkdir -p "$TARGET_DIR"
        
        # Check if source file exists before copying
        if [ -f "logo_${SIZE}.png" ]; then
            cp "logo_${SIZE}.png" "$TARGET_DIR/text-x-nux.png"
            cp "logo_${SIZE}.png" "$TARGET_DIR/application-x-nux.png"
            chmod 644 "$TARGET_DIR/text-x-nux.png"
            chmod 644 "$TARGET_DIR/application-x-nux.png"
        else
            echo "Warning: logo_${SIZE}.png not found. Skipping size $SIZE."
        fi
    fi
done

# Update Databases
echo "Updating MIME database..."
update-mime-database /usr/share/mime

echo "Updating Icon Cache..."
gtk-update-icon-cache -f -t /usr/share/icons/hicolor

echo "✓ System-wide Nux file association installed!"
