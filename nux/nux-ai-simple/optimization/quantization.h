// C - INT8 Quantization for 4x speedup and 4x memory reduction
// Quantization-Aware Training for minimal accuracy loss

#ifndef NUX_QUANTIZATION_H
#define NUX_QUANTIZATION_H

#include "../core/tensor.h"
#include <stdint.h>

// Quantized tensor (INT8)
typedef struct {
    int8_t* data;       // Quantized values
    float scale;        // Scaling factor
    int8_t zero_point;  // Zero point
    int* shape;
    int ndim;
    int size;
} QuantizedTensor;

// ============================================
// Quantization Methods
// ============================================

// Symmetric quantization: [-127, 127]
QuantizedTensor* quantize_symmetric(Tensor* t);

// Asymmetric quantization: [0, 255]
QuantizedTensor* quantize_asymmetric(Tensor* t);

// Per-channel quantization (better accuracy for weights)
QuantizedTensor* quantize_per_channel(Tensor* t, int channel_axis);

// Dequantize back to FP32
Tensor* dequantize(QuantizedTensor* qt);

// ============================================
// Quantized Operations (4x faster!)
// ============================================

// INT8 matrix multiplication
QuantizedTensor* quantized_matmul(QuantizedTensor* a, QuantizedTensor* b);

// Fused quantized operations
QuantizedTensor* quantized_linear_relu(
    QuantizedTensor* input,
    QuantizedTensor* weight,
    QuantizedTensor* bias
);

// ============================================
// Calibration for Post-Training Quantization
// ============================================

typedef struct {
    float* min_vals;
    float* max_vals;
    int num_batches;
} CalibrationData;

// Collect statistics for calibration
CalibrationData* calibration_create();
void calibration_update(CalibrationData* cal, Tensor* t);
QuantizedTensor* calibration_quantize(CalibrationData* cal, Tensor* t);
void calibration_free(CalibrationData* cal);

// ============================================
// Dynamic Quantization (quantize on-the-fly)
// ============================================

// Quantize activations dynamically during inference
void dynamic_quantize_forward(
    Tensor* input,
    QuantizedTensor* weight,
    Tensor* output
);

#endif // NUX_QUANTIZATION_H
