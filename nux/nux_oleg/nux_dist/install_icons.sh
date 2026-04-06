#!/bin/bash
set -e

# Define paths
MIME_DIR="$HOME/.local/share/mime/packages"
ICON_BASE="$HOME/.local/share/icons/hicolor"

# Ensure directories exist
mkdir -p "$MIME_DIR"

# Install MIME type
echo "Installing MIME type definition..."
cp nux.xml "$MIME_DIR/"

# Resize icons first
echo "Resizing icons..."
python3 resize_icons.py

echo "Installing icons..."
for SIZE in "${SIZES[@]}"; do
    TARGET_DIR="$ICON_BASE/$SIZE/mimetypes"
    mkdir -p "$TARGET_DIR"
    # Use the resized version
    cp "logo_${SIZE}.png" "$TARGET_DIR/text-x-nux.png"
    cp "logo_${SIZE}.png" "$TARGET_DIR/application-x-nux.png" 
done

# Legacy/Fallback install to ~/.icons (use 48x48 usually best for legacy)
mkdir -p "$HOME/.icons"
cp "logo_48x48.png" "$HOME/.icons/text-x-nux.png"
cp logo.png "$HOME/.icons/application-x-nux.png" 
# No gtk-update-icon-cache for ~/.icons usually needed or might fail if no index.theme

# Also install to scalable (if SVG available, otherwise PNG here is sometimes ignored but worth a try)
# mkdir -p "$ICON_BASE/scalable/mimetypes"
# cp logo.png "$ICON_BASE/scalable/mimetypes/text-x-nux.png"

# Update Databases
echo "Updating MIME database..."
update-mime-database "$HOME/.local/share/mime"

echo "Updating Icon Cache..."
gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" || echo "Warning: gtk-update-icon-cache failed."

echo "✓ Nux file association installed! Please restart Nautilus: 'nautilus -q'"
