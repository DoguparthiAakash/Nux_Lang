# Nux AI Framework - Simplified Architecture

**Focus**: C, C++, Assembly, Rust only
**Goal**: Build powerful AI models with clear, understandable code

## 🎯 Architecture Overview

```
┌─────────────────────────────────────────┐
│         Nux Language (High-level)       │
│   model.add(Linear(784, 128))           │
└────────────────┬────────────────────────┘
                 │ FFI
┌────────────────▼────────────────────────┐
│         C FFI Bindings                  │
└─┬──────┬──────┬──────┬──────────────────┘
  │      │      │      │
┌─▼──┐ ┌▼───┐ ┌▼───┐ ┌▼────┐
│ C  │ │C++ │ │ASM │ │Rust │
│Core│ │NN  │ │SIMD│ │Train│
└────┘ └────┘ └────┘ └─────┘
```

## 📁 Directory Structure

```
nux-ai/
├── core/               (C - Tensor operations)
│   ├── tensor.h
│   ├── tensor.c
│   └── memory.c
│
├── kernels/            (Assembly - SIMD)
│   ├── matmul_avx2.asm
│   ├── vector_ops.asm
│   └── activations.asm
│
├── nn/                 (C++ - Neural networks)
│   ├── layer.h
│   ├── layer.cpp
│   ├── linear.h
│   ├── linear.cpp
│   ├── activation.h
│   └── activation.cpp
│
├── training/           (Rust - Safe training)
│   ├── optimizer.rs
│   ├── trainer.rs
│   └── lib.rs
│
├── ffi/                (C - FFI bindings)
│   └── bindings.c
│
└── examples/           (Nux - Usage examples)
    ├── linear_regression.nux
    └── mnist_classifier.nux
```

## 🔧 Component Responsibilities

### C - Core Tensor
- Memory management
- Shape handling
- Basic operations
- Portability layer

### Assembly - SIMD Kernels
- Matrix multiplication (8x faster)
- Vector operations
- Activation functions
- Critical performance paths

### C++ - Neural Networks
- Layer abstraction
- Forward/backward passes
- Model composition
- Algorithm implementation

### Rust - Training
- Safe optimizer implementation
- Training loop
- Gradient management
- Memory safety guarantees

## 💡 Key Data Structures

### Tensor (C)
```c
typedef struct {
    float* data;      // Flat array
    int* shape;       // [rows, cols, ...]
    int ndim;         // Number of dimensions
    int size;         // Total elements
    int* strides;     // For indexing
} Tensor;
```

### Layer (C++)
```cpp
class Layer {
protected:
    Tensor* weights;
    Tensor* bias;
    Tensor* grad_weights;
    Tensor* grad_bias;
    
public:
    virtual Tensor* forward(Tensor* input) = 0;
    virtual Tensor* backward(Tensor* grad_output) = 0;
    virtual void update(float lr) = 0;
};
```

### Optimizer (Rust)
```rust
pub trait Optimizer {
    fn step(&mut self, params: &mut [f32], grads: &[f32]);
    fn zero_grad(&mut self);
}

pub struct Adam {
    lr: f32,
    beta1: f32,
    beta2: f32,
    m: Vec<f32>,  // First moment
    v: Vec<f32>,  // Second moment
}
```

## 🚀 Performance Strategy

| Component | Language | Speedup | Why |
|-----------|----------|---------|-----|
| Tensor ops | C | 2x | Direct memory access |
| Matrix mul | Assembly | 8x | AVX2 SIMD |
| Layers | C++ | 1x | Clean abstraction |
| Training | Rust | 1x | Safety + speed |

## 📊 Example: MNIST Classifier

### Nux Code (Simple!)
```nux
import "nux-ai";

// Build model
var model = new Model();
model.add(new Linear(784, 128));
model.add(new ReLU());
model.add(new Linear(128, 10));

// Train
var trainer = new Trainer(model, "adam", 0.001);
trainer.train(train_data, epochs=10);

// Predict
var pred = model.predict(test_image);
println("Predicted: " + argmax(pred));
```

### What Happens Under the Hood
1. `Linear(784, 128)` → C++ creates layer
2. `forward()` → C tensor operations
3. Matrix multiply → Assembly SIMD kernel
4. `train()` → Rust training loop
5. All safe, all fast!

## 🎯 Design Principles

1. **Simplicity First**
   - Clear variable names
   - Simple algorithms
   - Easy to debug

2. **Performance Where It Matters**
   - Assembly for hot paths
   - C for data structures
   - Rust for safety

3. **Easy to Understand**
   - ASCII diagrams
   - Inline comments
   - Example code

4. **Production Ready**
   - Memory safe
   - Well tested
   - Benchmarked

## 📖 Learning Path

1. **Start with C Tensor** - Understand data layout
2. **Add Assembly** - See performance gains
3. **Build C++ Layers** - Understand neural networks
4. **Use Rust Training** - Safe, concurrent training
5. **Write Nux Models** - High-level AI development

## 🏆 Goals

- ✅ 10-100x faster than Python
- ✅ Memory safe training
- ✅ Easy to understand
- ✅ Production ready
- ✅ Nux-native API
