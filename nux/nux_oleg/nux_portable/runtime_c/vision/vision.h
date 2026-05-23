#ifndef NUX_VISION_H
#define NUX_VISION_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Vision Operations
#define VISION_OP_GRAYSCALE 1
#define VISION_OP_EDGE_DETECT 2
#define VISION_OP_BLUR 3

// Applies a vision filter to an ARGB buffer in-place or to a destination
// w, h: Logical dimensions
void vision_process(int op, uint32_t* data, int width, int height);

#ifdef __cplusplus
}
#endif

#endif
