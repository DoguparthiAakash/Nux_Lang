#!/bin/bash
# Nux App Runner (Engine App)
# Launches a Nux application with necessary environment setup

if [ -z "$1" ]; then
    echo "Usage: ./app_runner.sh <app_file.nux>"
    exit 1
fi

APP_FILE="$1"

# In a real scenario, this runner would:
# 1. Initialize the display server (X11/Wayland/Framebuffer)
# 2. Set up audio drivers
# 3. Launch the Nux VM with specific flags for graphics acceleration

echo "----------------------------------------"
echo "Nux Engine App Runner v1.0"
echo "Loading: $APP_FILE"
echo "Initializing Graphics [Mock Mode]..."
echo "Initializing Audio [Mock Mode]..."
echo "----------------------------------------"

# Check if nux binary exists
# Prioritize local binaries for development
if [ -f "../bin/nux" ]; then
    NUX_BIN="../bin/nux"
elif [ -f "bin/nux" ]; then
    NUX_BIN="bin/nux"
elif [ -f "/usr/local/bin/nux" ]; then
    NUX_BIN="/usr/local/bin/nux"
else
    echo "Error: nux interpreter not found."
    exit 1
fi

# Set NUX_LIB to local development version if available
# This ensures we use the new libraries (gui, sql, markdown) even if not deployed system-wide
if [ -d "nux/lib" ]; then
    export NUX_LIB="$(pwd)/nux/lib"
    echo "Using Local Libs: $NUX_LIB"
elif [ -d "../lib" ]; then
    export NUX_LIB="$(pwd)/../lib"
    echo "Using Local Libs: $NUX_LIB"
elif [ -d "lib" ]; then
    export NUX_LIB="$(pwd)/lib"
    echo "Using Local Libs: $NUX_LIB"
fi

if [ -f "bin/nux" ]; then
     NUX_BIN="bin/nux"
fi

# Run the app
"$NUX_BIN" run "$APP_FILE"

echo "----------------------------------------"
echo "App Terminated."
