#!/bin/bash
# Self-Contained Nux Snake Launcher
# Located in nux_portable/
# Builds and Runs the C Runtime + Nux Code

set -e

# Ensure we are in the script's directory
cd "$(dirname "$0")"

if [ ! -f "./nuxc" ]; then
    echo "Error: nuxc compiler binary missing in nux_portable/."
    exit 1
fi

echo "=== Building Nux Runtime (C) ==="
# Compile the VM (Source in runtime_c/)
# Compile the Vision Module (C++)
g++ -c runtime_c/vision/vision.cpp -o runtime_c/vision.o -I runtime_c

# Compile Main and VM (C)
gcc -c runtime_c/main.c -o runtime_c/main.o -I runtime_c
gcc -c runtime_c/vm.c -o runtime_c/vm.o -I runtime_c

# Link everything
g++ -o nux runtime_c/main.o runtime_c/vm.o runtime_c/vision.o -lm

echo "=== Running Snake Game ==="
# ./nux will invoke ./nuxc to compile snake.nux
./nux snake.nux
