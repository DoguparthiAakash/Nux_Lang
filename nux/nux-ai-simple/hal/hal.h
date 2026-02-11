// Universal Hardware Abstraction Layer (HAL)
// Run Nux on ANY device: GPUs, TPUs, NPUs, CPUs, MCUs

#ifndef NUX_HAL_H
#define NUX_HAL_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================
// Device Types
// ============================================

typedef enum {
    DEVICE_CPU,           // x86-64, ARM, RISC-V
    DEVICE_CUDA,          // NVIDIA GPU
    DEVICE_ROCM,          // AMD GPU
    DEVICE_SYCL,          // Intel GPU (oneAPI)
    DEVICE_METAL,         // Apple GPU
    DEVICE_OPENCL,        // Generic OpenCL
    DEVICE_VULKAN,        // Vulkan Compute
    DEVICE_TPU,           // Google TPU
    DEVICE_NPU,           // Intel NPU
    DEVICE_NEURAL_ENGINE, // Apple Neural Engine
    DEVICE_EMBEDDED,      // Microcontrollers
    DEVICE_TINY_GPU       // Tiny GPU project
} DeviceType;

typedef enum {
    ARCH_X86_64,
    ARCH_ARM64,
    ARCH_RISCV,
    ARCH_XTENSA,          // ESP32
    ARCH_CORTEX_M,        // STM32, etc.
    ARCH_AVR              // Arduino
} CPUArchitecture;

// ============================================
// Device Information
// ============================================

typedef struct {
    DeviceType type;
    int device_id;
    char name[256];
    char vendor[128];
    
    // Capabilities
    size_t total_memory;
    size_t free_memory;
    int compute_capability;
    int max_threads;
    
    // CPU-specific
    CPUArchitecture arch;
    bool has_simd;
    bool has_fpu;
    int num_cores;
    int clock_mhz;
    
    // Features
    bool supports_fp16;
    bool supports_int8;
    bool supports_tensor_cores;
} Device;

// ============================================
// Device Management
// ============================================

// Enumerate all available devices
int hal_get_device_count();
Device* hal_get_device(int index);
Device* hal_get_best_device();  // Auto-select fastest

// Set active device
void hal_set_device(Device* dev);
Device* hal_get_current_device();

// Device properties
const char* hal_device_type_name(DeviceType type);
bool hal_device_available(DeviceType type);

// ============================================
// Memory Management
// ============================================

// Allocate device memory
void* hal_malloc(Device* dev, size_t size);
void hal_free(Device* dev, void* ptr);

// Memory transfer
void hal_memcpy_host_to_device(Device* dev, void* dst, const void* src, size_t size);
void hal_memcpy_device_to_host(Device* dev, void* dst, const void* src, size_t size);
void hal_memcpy_device_to_device(Device* dev, void* dst, const void* src, size_t size);

// Unified memory (if supported)
void* hal_malloc_unified(Device* dev, size_t size);

// ============================================
// Compute Operations
// ============================================

// Matrix multiplication
void hal_matmul(Device* dev,
                const float* a, const float* b, float* c,
                int m, int k, int n);

// Element-wise operations
void hal_add(Device* dev, const float* a, const float* b, float* c, int size);
void hal_mul(Device* dev, const float* a, const float* b, float* c, int size);

// Activations
void hal_relu(Device* dev, const float* input, float* output, int size);
void hal_sigmoid(Device* dev, const float* input, float* output, int size);
void hal_softmax(Device* dev, const float* input, float* output, int batch, int classes);

// Convolution
void hal_conv2d(Device* dev,
                const float* input, const float* kernel, float* output,
                int batch, int in_ch, int out_ch,
                int h, int w, int kh, int kw);

// ============================================
// Quantized Operations (INT8)
// ============================================

void hal_matmul_int8(Device* dev,
                     const int8_t* a, const int8_t* b, float* c,
                     float scale_a, float scale_b,
                     int m, int k, int n);

void hal_conv2d_int8(Device* dev,
                     const int8_t* input, const int8_t* kernel, int8_t* output,
                     float scale, ...);

// ============================================
// Synchronization
// ============================================

void hal_synchronize(Device* dev);
void hal_device_reset(Device* dev);

// ============================================
// Backend-Specific Functions
// ============================================

// CUDA
#ifdef NUX_CUDA_ENABLED
void* hal_cuda_get_stream(Device* dev);
void hal_cuda_set_stream(Device* dev, void* stream);
#endif

// ROCm
#ifdef NUX_ROCM_ENABLED
void* hal_rocm_get_stream(Device* dev);
#endif

// Metal
#ifdef NUX_METAL_ENABLED
void* hal_metal_get_command_queue(Device* dev);
#endif

// TPU
#ifdef NUX_TPU_ENABLED
void* hal_tpu_get_context(Device* dev);
#endif

// Embedded
#ifdef NUX_EMBEDDED_ENABLED
void hal_embedded_set_clock(int mhz);
void hal_embedded_sleep_ms(int ms);
#endif

#ifdef __cplusplus
}
#endif

#endif // NUX_HAL_H
