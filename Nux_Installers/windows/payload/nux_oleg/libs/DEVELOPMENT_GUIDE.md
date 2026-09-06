# Nux Libraries - Development Guide

## 🎯 Development Philosophy

Each sub-library is:
- **Independent** - Can be developed and tested in isolation
- **Modular** - Clear interfaces and minimal dependencies
- **Testable** - Comprehensive unit tests
- **Documented** - README and API docs

## 📁 Sub-Library Structure

```
sub-library/
├── include/
│   └── nux_*/
│       └── module/
│           └── header.h
├── src/
│   └── implementation.cpp
├── tests/
│   └── test_module.cpp
├── examples/
│   └── example_usage.cpp
├── CMakeLists.txt
└── README.md
```

## 🔧 Creating a New Sub-Library

### 1. Create Directory Structure
```bash
cd libs/nux-array
mkdir -p new-module/{include/nux_array/new_module,src,tests,examples}
```

### 2. Create CMakeLists.txt
```cmake
# libs/nux-array/new-module/CMakeLists.txt
cmake_minimum_required(VERSION 3.15)
project(NuxArrayNewModule VERSION 1.0.0)

# Source files
set(SOURCES
    src/implementation.cpp
)

# Create library
add_library(nux_array_new_module ${SOURCES})

# Include directories
target_include_directories(nux_array_new_module PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Dependencies
target_link_libraries(nux_array_new_module
    nux_safe
)

# Tests
if(BUILD_TESTS)
    add_subdirectory(tests)
endif()

# Examples
if(BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()
```

### 3. Create Header
```cpp
// include/nux_array/new_module/feature.h
#ifndef NUX_ARRAY_NEW_MODULE_FEATURE_H
#define NUX_ARRAY_NEW_MODULE_FEATURE_H

namespace NuxArray {
namespace NewModule {

class Feature {
public:
    Feature();
    void DoSomething();
};

} // namespace NewModule
} // namespace NuxArray

#endif
```

### 4. Create Implementation
```cpp
// src/implementation.cpp
#include "nux_array/new_module/feature.h"

namespace NuxArray {
namespace NewModule {

Feature::Feature() {
}

void Feature::DoSomething() {
    // Implementation
}

} // namespace NewModule
} // namespace NuxArray
```

### 5. Create Tests
```cpp
// tests/test_feature.cpp
#include "nux_array/new_module/feature.h"
#include <cassert>
#include <iostream>

int main() {
    NuxArray::NewModule::Feature f;
    f.DoSomething();
    
    std::cout << "✓ All tests passed!" << std::endl;
    return 0;
}
```

### 6. Update Parent CMakeLists.txt
```cmake
# libs/nux-array/CMakeLists.txt
add_subdirectory(new-module)

target_link_libraries(nux_array INTERFACE
    nux_array_new_module
)
```

## 🧪 Testing Strategy

### Unit Tests
Test individual functions in isolation:
```cpp
void test_addition() {
    Tensor a({2, 2}, {1, 2, 3, 4});
    Tensor b({2, 2}, {5, 6, 7, 8});
    auto c = a + b;
    assert(c.At(0, 0) == 6);
}
```

### Integration Tests
Test module interactions:
```cpp
void test_integration() {
    // Use multiple modules together
    auto tensor = Core::Tensor({10, 10});
    auto result = LinAlg::SVD(tensor);
    assert(result.IsValid());
}
```

### Performance Tests
Benchmark critical paths:
```cpp
void benchmark_matmul() {
    auto start = std::chrono::high_resolution_clock::now();
    
    Tensor a({1000, 1000});
    Tensor b({1000, 1000});
    auto c = a.MatMul(b);
    
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    
    std::cout << "MatMul took: " << duration.count() << "ms" << std::endl;
}
```

## 📝 Documentation Standards

### README.md Template
```markdown
# LibraryName - ModuleName

Brief description of what this module does.

## Features
- Feature 1
- Feature 2

## API
\`\`\`cpp
#include <nux_*/module/header.h>

// Example usage
\`\`\`

## Dependencies
- Dependency 1
- Dependency 2

## Build
\`\`\`bash
mkdir build && cd build
cmake ..
make
\`\`\`
```

### Code Documentation
Use Doxygen-style comments:
```cpp
/**
 * @brief Computes matrix multiplication
 * @param other The matrix to multiply with
 * @return Result of matrix multiplication
 * @throws std::invalid_argument if dimensions don't match
 */
Tensor MatMul(const Tensor& other) const;
```

## 🔄 Development Workflow

### 1. Feature Branch
```bash
git checkout -b feature/nux-array-fft
```

### 2. Implement
```bash
cd libs/nux-array/fft
# Edit files
```

### 3. Build & Test
```bash
mkdir build && cd build
cmake ..
make
./tests/fft_test
```

### 4. Integration Test
```bash
cd libs/nux-array
mkdir build && cd build
cmake ..
make
ctest
```

### 5. Commit
```bash
git add .
git commit -m "feat(nux-array): Add FFT module"
```

## 🎨 Code Style

### Naming Conventions
- **Classes**: PascalCase (`Tensor`, `LinearRegression`)
- **Functions**: PascalCase (`MatMul`, `Predict`)
- **Variables**: camelCase (`numSamples`, `learningRate`)
- **Constants**: UPPER_CASE (`MAX_ITERATIONS`)
- **Private members**: m_PascalCase (`m_Data`, `m_Size`)

### File Organization
```cpp
// 1. System includes
#include <vector>
#include <string>

// 2. Third-party includes
#include <external/lib.h>

// 3. Project includes
#include "nux_array/core/tensor.h"

// 4. Namespace
namespace NuxArray {
namespace Core {

// 5. Implementation

} // namespace Core
} // namespace NuxArray
```

## 🚀 Performance Guidelines

1. **Use move semantics** for large objects
2. **Avoid unnecessary copies** - use const references
3. **Preallocate memory** when size is known
4. **Use SIMD** for vectorizable operations
5. **Profile before optimizing** - measure, don't guess

## 📦 Dependency Management

### Internal Dependencies
```cmake
# Sub-library depends on another sub-library
target_link_libraries(nux_ai_conv
    nux_ai_core
    nux_array_core
)
```

### External Dependencies
```cmake
# Optional external dependency
find_package(OpenBLAS)
if(OpenBLAS_FOUND)
    target_link_libraries(nux_array_linalg OpenBLAS::OpenBLAS)
    target_compile_definitions(nux_array_linalg PRIVATE USE_OPENBLAS)
endif()
```

## 🎯 Module Examples

### NuxArray Modules
- **core**: Tensor, basic ops
- **linalg**: Matrix decomposition, solve
- **fft**: Fast Fourier Transform
- **random**: Random number generation
- **gpu**: CUDA acceleration

### NuxAI Modules
- **core**: Tensor with autograd
- **nn/linear**: Dense layers
- **nn/conv**: Convolutional layers
- **nn/recurrent**: RNN, LSTM, GRU
- **nn/attention**: Self-attention, transformers
- **optim**: Optimizers (SGD, Adam, etc.)
- **loss**: Loss functions

### NuxLearn Modules
- **supervised/linear**: Linear/Logistic regression
- **supervised/tree**: Decision trees, Random Forest
- **supervised/svm**: Support Vector Machines
- **unsupervised/clustering**: K-Means, DBSCAN
- **unsupervised/decomposition**: PCA, ICA
- **preprocessing**: Scalers, encoders
- **metrics**: Accuracy, F1, etc.

## 🏆 Best Practices

1. **One responsibility per module**
2. **Clear, minimal interfaces**
3. **Comprehensive error handling**
4. **Input validation**
5. **Memory safety**
6. **Thread safety where needed**
7. **Extensive testing**
8. **Performance benchmarks**
9. **Good documentation**
10. **Semantic versioning**
