#include <stdint.h>
#include <stdbool.h>
#include <cuda_runtime.h>
fn main() {
    uint32_t* data;
    cudaMallocManaged((void**)&(data), 1024);
    print("GPU Map Test!");
}

