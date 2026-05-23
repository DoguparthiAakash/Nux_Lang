// AMD ROCm/HIP Backend
// HIP is CUDA-compatible, so we can reuse CUDA kernels!

#include "../hal/hal.h"

#ifdef NUX_ROCM_ENABLED

#include <hip/hip_runtime.h>
#include <rocblas/rocblas.h>

// ============================================
// ROCm Device Management
// ============================================

static rocblas_handle rocblas_handles[16];

void hal_rocm_init() {
    int device_count;
    hipGetDeviceCount(&device_count);
    
    for (int i = 0; i < device_count; i++) {
        hipSetDevice(i);
        rocblas_create_handle(&rocblas_handles[i]);
    }
}

Device* hal_rocm_get_device(int device_id) {
    Device* dev = (Device*)malloc(sizeof(Device));
    dev->type = DEVICE_ROCM;
    dev->device_id = device_id;
    
    hipDeviceProp_t prop;
    hipGetDeviceProperties(&prop, device_id);
    
    strcpy(dev->name, prop.name);
    strcpy(dev->vendor, "AMD");
    dev->total_memory = prop.totalGlobalMem;
    dev->compute_capability = prop.major * 10 + prop.minor;
    dev->max_threads = prop.maxThreadsPerBlock;
    
    return dev;
}

// ============================================
// Memory Operations
// ============================================

void* hal_rocm_malloc(size_t size) {
    void* ptr;
    hipMalloc(&ptr, size);
    return ptr;
}

void hal_rocm_free(void* ptr) {
    hipFree(ptr);
}

void hal_rocm_memcpy_h2d(void* dst, const void* src, size_t size) {
    hipMemcpy(dst, src, size, hipMemcpyHostToDevice);
}

void hal_rocm_memcpy_d2h(void* dst, const void* src, size_t size) {
    hipMemcpy(dst, src, size, hipMemcpyDeviceToHost);
}

// ============================================
// Compute Kernels (HIP = CUDA syntax!)
// ============================================

__global__ void rocm_matmul_kernel(
    const float* a, const float* b, float* c,
    int m, int k, int n
) {
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    
    if (row < m && col < n) {
        float sum = 0.0f;
        for (int i = 0; i < k; i++) {
            sum += a[row * k + i] * b[i * n + col];
        }
        c[row * n + col] = sum;
    }
}

void hal_rocm_matmul(const float* a, const float* b, float* c,
                     int m, int k, int n) {
    // Use rocBLAS for maximum performance
    rocblas_handle handle = rocblas_handles[0];
    
    float alpha = 1.0f;
    float beta = 0.0f;
    
    rocblas_sgemm(handle,
                  rocblas_operation_none,
                  rocblas_operation_none,
                  n, m, k,
                  &alpha,
                  b, n,
                  a, k,
                  &beta,
                  c, n);
}

__global__ void rocm_relu_kernel(const float* input, float* output, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < size) {
        output[idx] = fmaxf(0.0f, input[idx]);
    }
}

void hal_rocm_relu(const float* input, float* output, int size) {
    int threads = 256;
    int blocks = (size + threads - 1) / threads;
    hipLaunchKernelGGL(rocm_relu_kernel, dim3(blocks), dim3(threads), 0, 0,
                       input, output, size);
}

#endif // NUX_ROCM_ENABLED
