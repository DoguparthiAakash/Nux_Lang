# Nux Libraries - Build Guide

## Quick Start

### Build All Libraries
```bash
cd libs
mkdir build && cd build
cmake ..
make -j$(nproc)
```

### Build Specific Library
```bash
cd libs
mkdir build && cd build
cmake -DBUILD_ARRAY=ON -DBUILD_FRAME=OFF -DBUILD_LEARN=OFF ..
make
```

### Build Specific Sub-Module
```bash
cd libs/nux-array/core
mkdir build && cd build
cmake ..
make
```

## Build Options

### Main Libraries
- `BUILD_ARRAY` - NuxArray (default: ON)
- `BUILD_FRAME` - NuxFrame (default: ON)
- `BUILD_LEARN` - NuxLearn (default: ON)
- `BUILD_AI` - NuxAI (default: ON)
- `BUILD_VISION` - NuxVision (default: ON)
- `BUILD_NLP` - NuxNLP (default: ON)
- `BUILD_STATS` - NuxStats (default: ON)
- `BUILD_PLOT` - NuxPlot (default: ON)
- `BUILD_GUI` - NuxGUI (default: ON)
- `BUILD_DISTRIBUTED` - NuxDistributed (default: ON)
- `BUILD_QUANTUM` - NuxQuantum (default: ON)
- `BUILD_BLOCKCHAIN` - NuxBlockchain (default: ON)
- `BUILD_CRYPTO` - NuxCrypto (default: ON)
- `BUILD_SAFE` - NuxSafe (default: ON)

### Feature Options
- `ENABLE_GPU` - Enable GPU acceleration (default: OFF)
- `ENABLE_OPENMP` - Enable OpenMP parallelization (default: ON)
- `BUILD_TESTS` - Build unit tests (default: ON)
- `BUILD_EXAMPLES` - Build examples (default: ON)

## Examples

### Minimal Build (Core Only)
```bash
cmake -DBUILD_ARRAY=ON -DBUILD_SAFE=ON \
      -DBUILD_FRAME=OFF -DBUILD_LEARN=OFF \
      -DBUILD_AI=OFF -DBUILD_VISION=OFF \
      -DBUILD_NLP=OFF -DBUILD_STATS=OFF \
      -DBUILD_PLOT=OFF -DBUILD_GUI=OFF \
      -DBUILD_DISTRIBUTED=OFF -DBUILD_QUANTUM=OFF \
      -DBUILD_BLOCKCHAIN=OFF -DBUILD_CRYPTO=OFF ..
```

### ML/AI Stack
```bash
cmake -DBUILD_ARRAY=ON -DBUILD_FRAME=ON \
      -DBUILD_LEARN=ON -DBUILD_AI=ON \
      -DBUILD_STATS=ON -DBUILD_PLOT=ON \
      -DBUILD_SAFE=ON ..
```

### Computer Vision Stack
```bash
cmake -DBUILD_ARRAY=ON -DBUILD_VISION=ON \
      -DBUILD_AI=ON -DBUILD_PLOT=ON \
      -DBUILD_SAFE=ON ..
```

### With GPU Support
```bash
cmake -DENABLE_GPU=ON ..
make
```

## Installation

```bash
sudo make install
```

This installs:
- Headers to `/usr/local/include/nux_*/`
- Libraries to `/usr/local/lib/`
- CMake configs to `/usr/local/lib/cmake/`

## Usage in Your Project

### CMake
```cmake
find_package(NuxArray REQUIRED)
find_package(NuxAI REQUIRED)

add_executable(myapp main.cpp)
target_link_libraries(myapp NuxArray::nux_array NuxAI::nux_ai)
```

### Manual Compilation
```bash
g++ -std=c++17 main.cpp \
    -I/usr/local/include \
    -L/usr/local/lib \
    -lnux_array -lnux_ai \
    -o myapp
```

## Development Workflow

### 1. Work on Sub-Module
```bash
cd libs/nux-ai/nn/conv
# Edit files
mkdir build && cd build
cmake ..
make
./tests/conv_test
```

### 2. Test Integration
```bash
cd libs/nux-ai
mkdir build && cd build
cmake ..
make
./tests/integration_test
```

### 3. Full Build
```bash
cd libs
mkdir build && cd build
cmake ..
make -j$(nproc)
ctest  # Run all tests
```

## Troubleshooting

### Missing Dependencies
```bash
# Install build tools
sudo apt-get install build-essential cmake

# Install optional dependencies
sudo apt-get install libopenblas-dev  # For BLAS
sudo apt-get install nvidia-cuda-toolkit  # For GPU
```

### Clean Build
```bash
rm -rf build
mkdir build && cd build
cmake ..
make clean
make
```
