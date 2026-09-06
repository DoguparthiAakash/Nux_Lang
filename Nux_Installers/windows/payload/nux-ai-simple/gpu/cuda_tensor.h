// CUDA - GPU Accelerated Tensor Operations
// Massive performance gains over CPU

#ifndef NUX_CUDA_TENSOR_H
#define NUX_CUDA_TENSOR_H

#include "../core/tensor.h"

#ifdef __cplusplus
extern "C" {
#endif

// GPU Tensor structure
typedef struct {
    float* device_data;     // GPU memory
    float* host_data;       // CPU memory (cached)
    int* shape;
    int ndim;
    int size;
    int device_id;          // Which GPU
    bool on_device;         // Currently on GPU?
} CUDATensor;

// ============================================
// Memory Management
// ============================================

// Create tensor on GPU
CUDATensor* cuda_tensor_create(int* shape, int ndim, int device_id);

// Transfer CPU -> GPU
void cuda_tensor_to_device(CUDATensor* t);

// Transfer GPU -> CPU
void cuda_tensor_to_host(CUDATensor* t);

// Free GPU memory
void cuda_tensor_free(CUDATensor* t);

// ============================================
// GPU Operations (10-100x faster than CPU)
// ============================================

// Matrix multiplication using cuBLAS
// Up to 100x faster than CPU!
CUDATensor* cuda_matmul(CUDATensor* a, CUDATensor* b);

// Element-wise operations (fused kernels)
CUDATensor* cuda_add(CUDATensor* a, CUDATensor* b);
CUDATensor* cuda_mul(CUDATensor* a, CUDATensor* b);

// Activations (custom kernels)
CUDATensor* cuda_relu(CUDATensor* input);
CUDATensor* cuda_sigmoid(CUDATensor* input);
CUDATensor* cuda_tanh(CUDATensor* input);

// Softmax (optimized with shared memory)
CUDATensor* cuda_softmax(CUDATensor* input);

// ============================================
// Advanced GPU Features
// ============================================

// Tensor Core operations (FP16, 8x faster)
CUDATensor* cuda_matmul_fp16(CUDATensor* a, CUDATensor* b);

// Fused operations (single kernel, no intermediate memory)
CUDATensor* cuda_linear_relu(CUDATensor* input, CUDATensor* weight, CUDATensor* bias);

// Multi-GPU support
void cuda_set_device(int device_id);
int cuda_get_device_count();

// Asynchronous operations (overlap compute + transfer)
void cuda_async_matmul(CUDATensor* a, CUDATensor* b, CUDATensor* c, void* stream);

#ifdef __cplusplus
}
#endif

#endif // NUX_CUDA_TENSOR_H
