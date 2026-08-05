#!/usr/bin/env bash
# ============================================================================
# Nux Programming Language - macOS PKG Builder
# Creates a native .pkg installer with GUI (double-click to install)
# ============================================================================
set -e

VERSION="1.0.0"
IDENTIFIER="com.nuxlang.nux"
SCRIPTS_DIR="pkg_scripts"
PAYLOAD_DIR="pkg_root"
OUTPUT_PKG="nux-${VERSION}-macos.pkg"
DIST_XML="distribution.xml"

echo "Building macOS Package: $OUTPUT_PKG"

# --- Clean & Prepare ---
rm -rf "$PAYLOAD_DIR" "$SCRIPTS_DIR" "$OUTPUT_PKG" "$DIST_XML"

mkdir -p "$PAYLOAD_DIR/usr/local/lib/nux/lib"
mkdir -p "$PAYLOAD_DIR/usr/local/bin"
mkdir -p "$SCRIPTS_DIR"

# --- Copy Binaries ---
# NOTE: Place compiled macOS nux binary and lib/*.nux files before running this script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if [ -d "$SCRIPT_DIR/payload" ]; then
    cp -r "$SCRIPT_DIR/payload/"* "$PAYLOAD_DIR/usr/local/lib/nux/" 2>/dev/null || true
fi

# --- Create symlink via postinstall script ---
cat <<'POSTINSTALL' > "$SCRIPTS_DIR/postinstall"
#!/bin/bash
ln -sf /usr/local/lib/nux/nux /usr/local/bin/nux
chmod +x /usr/local/bin/nux 2>/dev/null || true
chmod +x /usr/local/lib/nux/nux 2>/dev/null || true

# Set environment variables in shell profiles
for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
    if [ -f "$profile" ]; then
        if ! grep -q "NUX_HOME" "$profile" 2>/dev/null; then
            echo "" >> "$profile"
            echo "# Nux Programming Language" >> "$profile"
            echo 'export NUX_HOME="/usr/local/lib/nux"' >> "$profile"
            echo 'export NUX_LIB_PATH="/usr/local/lib/nux/lib"' >> "$profile"
        fi
    fi
done
echo "Nux installed successfully!"
POSTINSTALL
chmod +x "$SCRIPTS_DIR/postinstall"

# --- Build the component package ---
if command -v pkgbuild &> /dev/null; then
    pkgbuild \
        --root "$PAYLOAD_DIR" \
        --identifier "$IDENTIFIER" \
        --version "$VERSION" \
        --scripts "$SCRIPTS_DIR" \
        --install-location "/" \
        "nux-core.pkg"

    # --- Create Distribution XML for productbuild (GUI installer) ---
    cat <<EOF > "$DIST_XML"
<?xml version="1.0" encoding="utf-8"?>
<installer-gui-script minSpecVersion="2">
    <title>Nux Programming Language ${VERSION}</title>
    <welcome file="welcome.html" />
    <license file="license.txt" />
    <options customize="allow" require-scripts="false" />
    <choices-outline>
        <line choice="nux-core"/>
    </choices-outline>
    <choice id="nux-core" title="Nux Core + Standard Library" description="The Nux compiler, runtime, and standard library." start_selected="true" start_enabled="true" start_visible="true">
        <pkg-ref id="$IDENTIFIER"/>
    </choice>
    <pkg-ref id="$IDENTIFIER" version="$VERSION">nux-core.pkg</pkg-ref>
</installer-gui-script>
EOF

    # Create welcome and license files
    cat <<'WELCOME' > welcome.html
<html>
<body>
<h1>Welcome to Nux</h1>
<p>This installer will guide you through installing the <b>Nux Programming Language</b> on your Mac.</p>
<p>Nux is a fast, lightweight, Write-Once-Run-Anywhere language with built-in support for ML, quantum computing, 3D engines, and more.</p>
<p>After installation, open Terminal and type <code>nux --version</code> to verify.</p>
</body>
</html>
WELCOME

    cat <<'LICENSE' > license.txt
MIT License

Copyright (c) 2026 NuxLang

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
LICENSE

    if command -v productbuild &> /dev/null; then
        productbuild \
            --distribution "$DIST_XML" \
            --package-path "." \
            --resources "." \
            "$OUTPUT_PKG"
        echo "Successfully built $OUTPUT_PKG (GUI installer)"
    else
        mv "nux-core.pkg" "$OUTPUT_PKG"
        echo "Built $OUTPUT_PKG (component package, no productbuild available)"
    fi

    # Cleanup
    rm -rf "$PAYLOAD_DIR" "$SCRIPTS_DIR" "$DIST_XML" "nux-core.pkg" "welcome.html" "license.txt"
else
    echo "ERROR: pkgbuild not found. This script must be run on macOS."
    exit 1
fi
