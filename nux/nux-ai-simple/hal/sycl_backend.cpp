// Intel oneAPI/SYCL Backend
// Unified programming for Intel CPUs, GPUs, FPGAs

#include "../hal/hal.h"

#ifdef NUX_SYCL_ENABLED

#include <sycl/sycl.hpp>

using namespace sycl;

// ============================================
// SYCL Device Management
// ============================================

static queue* sycl_queues[16];

void hal_sycl_init() {
    // Enumerate Intel GPUs
    auto devices = device::get_devices(info::device_type::gpu);
    
    for (size_t i = 0; i < devices.size() && i < 16; i++) {
        sycl_queues[i] = new queue(devices[i]);
    }
}

Device* hal_sycl_get_device(int device_id) {
    Device* dev = (Device*)malloc(sizeof(Device));
    dev->type = DEVICE_SYCL;
    dev->device_id = device_id;
    
    auto& q = *sycl_queues[device_id];
    auto device = q.get_device();
    
    std::string name = device.get_info<info::device::name>();
    std::string vendor = device.get_info<info::device::vendor>();
    
    strncpy(dev->name, name.c_str(), 255);
    strncpy(dev->vendor, vendor.c_str(), 127);
    
    dev->total_memory = device.get_info<info::device::global_mem_size>();
    dev->max_threads = device.get_info<info::device::max_work_group_size>();
    
    return dev;
}

// ============================================
// Memory Operations
// ============================================

void* hal_sycl_malloc(int device_id, size_t size) {
    auto& q = *sycl_queues[device_id];
    return malloc_device(size, q);
}

void hal_sycl_free(int device_id, void* ptr) {
    auto& q = *sycl_queues[device_id];
    free(ptr, q);
}

void hal_sycl_memcpy_h2d(int device_id, void* dst, const void* src, size_t size) {
    auto& q = *sycl_queues[device_id];
    q.memcpy(dst, src, size).wait();
}

// ============================================
// Compute Kernels
// ============================================

void hal_sycl_matmul(int device_id,
                     const float* a, const float* b, float* c,
                     int m, int k, int n) {
    auto& q = *sycl_queues[device_id];
    
    // Use oneMKL for optimized BLAS
    // Or custom kernel:
    q.parallel_for(range<2>(m, n), [=](id<2> idx) {
        int row = idx[0];
        int col = idx[1];
        
        float sum = 0.0f;
        for (int i = 0; i < k; i++) {
            sum += a[row * k + i] * b[i * n + col];
        }
        c[row * n + col] = sum;
    }).wait();
}

void hal_sycl_relu(int device_id, const float* input, float* output, int size) {
    auto& q = *sycl_queues[device_id];
    
    q.parallel_for(range<1>(size), [=](id<1> idx) {
        output[idx] = sycl::fmax(0.0f, input[idx]);
    }).wait();
}

// ============================================
// XMX (Tensor Core equivalent) Support
// ============================================

void hal_sycl_matmul_xmx(int device_id,
                         const float* a, const float* b, float* c,
                         int m, int k, int n) {
    // Use Intel XMX engines for 8x speedup
    // Requires oneMKL or custom implementation
    auto& q = *sycl_queues[device_id];
    
    // Use sub-groups for XMX
    q.submit([&](handler& h) {
        h.parallel_for(nd_range<2>({m, n}, {16, 16}), [=](nd_item<2> item) {
            // XMX matrix operations
            // 16x16 tiles
        });
    }).wait();
}

#endif // NUX_SYCL_ENABLED
