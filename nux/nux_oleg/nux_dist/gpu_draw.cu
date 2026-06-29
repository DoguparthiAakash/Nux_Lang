#include <cuda_runtime.h>
#include <stdio.h>
#include <stdint.h>
#include <math.h>

__global__ void render_kernel(char* output, int width, int height, float time) {
    int x = blockIdx.x * blockDim.x + threadIdx.x;
    int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;

    // Normalize coordinates to [-1, 1]
    float u = (x * 2.0f) / width - 1.0f;
    float v = (y * 2.0f) / height - 1.0f;
    
    // Adjust aspect ratio (terminal characters are roughly 2x taller than wide)
    u *= ((float)width / height) * 0.5f;

    // Ray origin and direction
    float ro_x = 0.0f;
    float ro_y = 0.0f;
    float ro_z = -3.0f;
    
    float rd_x = u;
    float rd_y = v;
    float rd_z = 1.0f;

    // Normalize rd
    float len = sqrtf(rd_x*rd_x + rd_y*rd_y + rd_z*rd_z);
    rd_x /= len; rd_y /= len; rd_z /= len;

    // Sphere parameters
    float cx = sinf(time) * 1.5f;
    float cy = cosf(time * 0.5f) * 0.5f;
    float cz = 0.0f;
    float r = 1.0f;

    // Ray-sphere intersection (a*t^2 + b*t + c = 0)
    float oc_x = ro_x - cx;
    float oc_y = ro_y - cy;
    float oc_z = ro_z - cz;

    float b = 2.0f * (oc_x*rd_x + oc_y*rd_y + oc_z*rd_z);
    float c = (oc_x*oc_x + oc_y*oc_y + oc_z*oc_z) - r*r;
    float a = 1.0f;

    float discriminant = b*b - 4.0f*a*c;

    char pixel = ' ';
    if (discriminant > 0.0f) {
        float t = (-b - sqrtf(discriminant)) / (2.0f*a);
        if (t > 0.0f) {
            // Hit!
            float hit_x = ro_x + t*rd_x;
            float hit_y = ro_y + t*rd_y;
            float hit_z = ro_z + t*rd_z;

            float nx = (hit_x - cx) / r;
            float ny = (hit_y - cy) / r;
            float nz = (hit_z - cz) / r;

            // Light direction
            float lx = -1.0f;
            float ly = -1.0f;
            float lz = -1.0f;
            float llen = sqrtf(lx*lx + ly*ly + lz*lz);
            lx /= llen; ly /= llen; lz /= llen;

            // Diffuse lighting
            float diffuse = nx*lx + ny*ly + nz*lz;
            if (diffuse < 0.0f) diffuse = 0.0f;
            
            int intensity = (int)(diffuse * 11.0f);
            const char shades[] = ".,-~:;=!*#$@";
            if (intensity > 11) intensity = 11;
            pixel = shades[intensity];
        }
    }

    output[y * (width + 1) + x] = pixel;
    
    if (x == width - 1) {
        output[y * (width + 1) + width] = '\n';
    }
}

extern "C" __declspec(dllexport) int64_t draw_3d_object(const int64_t* args, size_t num_args, const void* state_ptr) {
    if (num_args < 3) return -1;
    int width = (int)args[0];
    int height = (int)args[1];
    float time = (float)args[2] / 10.0f;

    int total_chars = height * (width + 1) + 1;
    char* d_output;
    cudaMalloc((void**)&d_output, total_chars);
    
    char* h_output = (char*)malloc(total_chars);
    for(int i=0; i<total_chars; i++) h_output[i] = ' ';
    for(int y=0; y<height; y++) h_output[y*(width+1) + width] = '\n';
    h_output[total_chars-1] = '\0';
    
    cudaMemcpy(d_output, h_output, total_chars, cudaMemcpyHostToDevice);

    dim3 blockSize(16, 16);
    dim3 gridSize((width + blockSize.x - 1) / blockSize.x, (height + blockSize.y - 1) / blockSize.y);

    render_kernel<<<gridSize, blockSize>>>(d_output, width, height, time);
    cudaDeviceSynchronize();

    cudaMemcpy(h_output, d_output, total_chars, cudaMemcpyDeviceToHost);
    
    printf("\033[H\033[J"); // clear screen ANSI escape sequence
    printf("%s", h_output);

    cudaFree(d_output);
    free(h_output);

    return 0;
}
