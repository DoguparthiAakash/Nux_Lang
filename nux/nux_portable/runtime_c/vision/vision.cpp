#include "vision.h"
#include <cmath>
#include <vector>
#include <algorithm>

// --- Helper: manual implementation of basic image ops ---

// Extracts Luminance from ARGB
static inline uint8_t get_luma(uint32_t pixel) {
    uint8_t r = (pixel >> 16) & 0xFF;
    uint8_t g = (pixel >> 8) & 0xFF;
    uint8_t b = pixel & 0xFF;
    return (uint8_t)(0.299f * r + 0.587f * g + 0.114f * b);
}

// Reconstructs ARGB from Luminance
static inline uint32_t to_argb(uint8_t luma) {
    return 0xFF000000 | (luma << 16) | (luma << 8) | luma;
}

// 1. Grayscale Filter
void filter_grayscale(uint32_t* data, int w, int h) {
    for (int i = 0; i < w * h; i++) {
        data[i] = to_argb(get_luma(data[i]));
    }
}

// 2. Sobel Edge Detection (Simplified Canny approximation)
// We compute Gradients, Magnitude, and Threshold.
void filter_edge_detect(uint32_t* data, int w, int h) {
    // Temp buffer for grayscale
    std::vector<uint8_t> gray(w * h);
    for (int i = 0; i < w * h; i++) {
        gray[i] = get_luma(data[i]);
    }

    std::vector<uint8_t> edges(w * h, 0);

    // Sobel Kernels
    // Gx: -1 0 1
    //     -2 0 2
    //     -1 0 1
    // Gy: -1 -2 -1
    //      0  0  0
    //      1  2  1

    for (int y = 1; y < h - 1; y++) {
        for (int x = 1; x < w - 1; x++) {
            int idx = y * w + x;
            
            // Convolution
            int gx = -gray[idx - w - 1] + gray[idx - w + 1]
                     -2 * gray[idx - 1] + 2 * gray[idx + 1]
                     -gray[idx + w - 1] + gray[idx + w + 1];

            int gy = -gray[idx - w - 1] - 2 * gray[idx - w] - gray[idx - w + 1]
                     +gray[idx + w - 1] + 2 * gray[idx + w] + gray[idx + w + 1];

            int mag = (int)std::sqrt(gx * gx + gy * gy);
            
            // Simple Threshold (e.g., 100)
            if (mag > 100) {
                edges[idx] = 255;
            } else {
                edges[idx] = 0;
            }
        }
    }

    // Write back
    for (int i = 0; i < w * h; i++) {
        data[i] = to_argb(edges[i]);
    }
}

// Dispatcher
extern "C" void vision_process(int op, uint32_t* data, int width, int height) {
    switch (op) {
        case VISION_OP_GRAYSCALE:
            filter_grayscale(data, width, height);
            break;
        case VISION_OP_EDGE_DETECT:
            filter_edge_detect(data, width, height);
            break;
        default:
            break; // No-op
    }
}
