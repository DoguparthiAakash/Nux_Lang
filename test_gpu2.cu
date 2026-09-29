#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <cuda_runtime.h>
#include <stdio.h>
int main() {
    uint8_t* ptr;
    cudaMallocManaged((void**)&(ptr), 1024);
    printf("GPU unified memory mapped successfully!\n");
}

