#include <stdint.h>
#include <stdbool.h>
#include <cuda_runtime.h>
fn main() {
    let mut data_ptr: int = 0;
    cudaMallocManaged((void**)&(data_ptr), 1024);
    print("Mapped data_ptr to CUDA managed memory!\n");
}

