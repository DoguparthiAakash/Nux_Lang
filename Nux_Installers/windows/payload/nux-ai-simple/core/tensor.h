// C - Simple, Clear Tensor Implementation
// Easy to understand, fast, portable

#ifndef NUX_TENSOR_H
#define NUX_TENSOR_H

#include <stddef.h>
#include <stdbool.h>

// Simple tensor structure
typedef struct {
    float* data;        // Flat array of data
    int* shape;         // Dimensions [rows, cols, ...]
    int* strides;       // For multi-dim indexing
    int ndim;           // Number of dimensions
    int size;           // Total number of elements
    bool requires_grad; // For autograd
} Tensor;

// ============================================
// Creation & Destruction
// ============================================

// Create tensor with given shape
Tensor* tensor_create(int* shape, int ndim);

// Create tensor filled with zeros
Tensor* tensor_zeros(int* shape, int ndim);

// Create tensor filled with ones
Tensor* tensor_ones(int* shape, int ndim);

// Create tensor with random values
Tensor* tensor_random(int* shape, int ndim);

// Free tensor memory
void tensor_free(Tensor* t);

// ============================================
// Basic Operations (Element-wise)
// ============================================

// Add two tensors: C = A + B
Tensor* tensor_add(Tensor* a, Tensor* b);

// Subtract: C = A - B
Tensor* tensor_sub(Tensor* a, Tensor* b);

// Multiply: C = A * B (element-wise)
Tensor* tensor_mul(Tensor* a, Tensor* b);

// Divide: C = A / B
Tensor* tensor_div(Tensor* a, Tensor* b);

// ============================================
// Scalar Operations
// ============================================

// Add scalar: C = A + scalar
Tensor* tensor_add_scalar(Tensor* a, float scalar);

// Multiply by scalar: C = A * scalar
Tensor* tensor_mul_scalar(Tensor* a, float scalar);

// ============================================
// Matrix Operations
// ============================================

// Matrix multiplication: C = A @ B
Tensor* tensor_matmul(Tensor* a, Tensor* b);

// Transpose: B = A^T
Tensor* tensor_transpose(Tensor* a);

// Dot product (1D tensors)
float tensor_dot(Tensor* a, Tensor* b);

// ============================================
// Reduction Operations
// ============================================

// Sum all elements
float tensor_sum(Tensor* t);

// Mean of all elements
float tensor_mean(Tensor* t);

// Maximum element
float tensor_max(Tensor* t);

// Minimum element
float tensor_min(Tensor* t);

// ============================================
// Shape Operations
// ============================================

// Reshape tensor
Tensor* tensor_reshape(Tensor* t, int* new_shape, int new_ndim);

// Get element at index
float tensor_get(Tensor* t, int* indices);

// Set element at index
void tensor_set(Tensor* t, int* indices, float value);

// ============================================
// Utility Functions
// ============================================

// Print tensor (for debugging)
void tensor_print(Tensor* t);

// Copy tensor
Tensor* tensor_copy(Tensor* t);

// Check if shapes are compatible
bool tensor_shapes_equal(Tensor* a, Tensor* b);

#endif // NUX_TENSOR_H
