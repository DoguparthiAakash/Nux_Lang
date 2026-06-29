#include <stdint.h>
#include <stddef.h>

int64_t my_gpu_add_impl(int64_t x, int64_t y) {
    // A simple C implementation since we might not have CUDA nvcc installed
    return x + y + 100;
}

#ifdef _WIN32
__declspec(dllexport)
#endif
int64_t my_gpu_add(const int64_t* args, size_t num_args, const void* state) {
    if (num_args < 2) return 0;
    int64_t x = args[0];
    int64_t y = args[1];
    return my_gpu_add_impl(x, y);
}

