# NuxArray Core

Core tensor operations and memory management.

## Features
- Multi-dimensional arrays
- Basic arithmetic operations
- Broadcasting
- Indexing and slicing
- Memory management

## API
```cpp
#include <nux_array/core/tensor.h>

Tensor a({2, 3});  // 2x3 tensor
Tensor b = a + 5;  // Broadcasting
auto c = a.MatMul(b.Transpose());
```

## Dependencies
- C++17 standard library
- NuxSafe (memory safety)

## Build
```bash
cd core
mkdir build && cd build
cmake ..
make
```
