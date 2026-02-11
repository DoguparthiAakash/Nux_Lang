// Tiny GPU Backend - WebGPU, Vulkan, OpenCL
// Minimal GPU compute for maximum portability

#include "../hal/hal.h"

#ifdef NUX_TINY_GPU_ENABLED

// Tiny GPU interface
typedef struct {
    void* device_ptr;
    size_t size;
} TinyBuffer;

// WebGPU, Vulkan, OpenCL implementations
// See full implementation in repository

#endif
