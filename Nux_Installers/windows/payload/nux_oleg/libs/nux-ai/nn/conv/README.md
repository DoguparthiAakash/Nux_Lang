# NuxAI - Convolutional Layers

Convolutional neural network layers for image processing.

## Features
- Conv1D, Conv2D, Conv3D
- Pooling layers (MaxPool, AvgPool)
- Transposed convolution
- Depthwise separable convolution

## API
```cpp
#include <nux_ai/nn/conv/conv2d.h>

Conv2D conv(3, 64, 3, 1, 1);  // in_channels, out_channels, kernel, stride, padding
auto output = conv.Forward(input);
```

## Dependencies
- NuxAI Core (tensor with autograd)
- NuxArray (tensor operations)

## Build
```bash
cd nn/conv
mkdir build && cd build
cmake ..
make
```
