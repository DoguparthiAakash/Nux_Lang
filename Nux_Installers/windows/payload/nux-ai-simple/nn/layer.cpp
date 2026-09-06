// C++ - Neural Network Layer Implementation

#include "layer.h"
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <algorithm>

namespace NuxAI {

// ============================================
// Linear Layer
// ============================================

Linear::Linear(int in_feat, int out_feat)
    : Layer("Linear")
    , in_features(in_feat)
    , out_features(out_feat)
    , input_cache(nullptr)
{
    // Create weight and bias tensors
    int weight_shape[] = {in_features, out_features};
    weights = tensor_create(weight_shape, 2);
    
    int bias_shape[] = {out_features};
    bias = tensor_create(bias_shape, 1);
    
    // Create gradient tensors
    grad_weights = tensor_create(weight_shape, 2);
    grad_bias = tensor_create(bias_shape, 1);
    
    // Initialize weights
    init_weights();
}

Linear::~Linear() {
    tensor_free(weights);
    tensor_free(bias);
    tensor_free(grad_weights);
    tensor_free(grad_bias);
    if (input_cache) tensor_free(input_cache);
}

void Linear::init_weights() {
    // Xavier initialization: scale = sqrt(2 / (in + out))
    float scale = std::sqrt(2.0f / (in_features + out_features));
    
    static bool seeded = false;
    if (!seeded) {
        srand(time(NULL));
        seeded = true;
    }
    
    // Initialize weights
    for (int i = 0; i < weights->size; i++) {
        weights->data[i] = ((float)rand() / RAND_MAX * 2.0f - 1.0f) * scale;
    }
    
    // Initialize bias to zero
    for (int i = 0; i < bias->size; i++) {
        bias->data[i] = 0.0f;
    }
}

Tensor* Linear::forward(Tensor* input) {
    // Cache input for backward pass
    if (input_cache) tensor_free(input_cache);
    input_cache = tensor_copy(input);
    
    // output = input @ weights + bias
    Tensor* output = tensor_matmul(input, weights);
    
    // Add bias (broadcasting)
    for (int i = 0; i < output->shape[0]; i++) {
        for (int j = 0; j < output->shape[1]; j++) {
            output->data[i * output->shape[1] + j] += bias->data[j];
        }
    }
    
    return output;
}

Tensor* Linear::backward(Tensor* grad_output) {
    // grad_weights = input^T @ grad_output
    Tensor* input_T = tensor_transpose(input_cache);
    tensor_free(grad_weights);
    grad_weights = tensor_matmul(input_T, grad_output);
    tensor_free(input_T);
    
    // grad_bias = sum(grad_output, axis=0)
    for (int j = 0; j < grad_output->shape[1]; j++) {
        float sum = 0.0f;
        for (int i = 0; i < grad_output->shape[0]; i++) {
            sum += grad_output->data[i * grad_output->shape[1] + j];
        }
        grad_bias->data[j] = sum;
    }
    
    // grad_input = grad_output @ weights^T
    Tensor* weights_T = tensor_transpose(weights);
    Tensor* grad_input = tensor_matmul(grad_output, weights_T);
    tensor_free(weights_T);
    
    return grad_input;
}

void Linear::update(float learning_rate) {
    if (!trainable) return;
    
    // weights -= learning_rate * grad_weights
    for (int i = 0; i < weights->size; i++) {
        weights->data[i] -= learning_rate * grad_weights->data[i];
    }
    
    // bias -= learning_rate * grad_bias
    for (int i = 0; i < bias->size; i++) {
        bias->data[i] -= learning_rate * grad_bias->data[i];
    }
}

// ============================================
// ReLU Layer
// ============================================

ReLU::~ReLU() {
    if (input_cache) tensor_free(input_cache);
}

Tensor* ReLU::forward(Tensor* input) {
    // Cache input for backward pass
    if (input_cache) tensor_free(input_cache);
    input_cache = tensor_copy(input);
    
    // output = max(0, input)
    Tensor* output = tensor_create(input->shape, input->ndim);
    for (int i = 0; i < input->size; i++) {
        output->data[i] = std::max(0.0f, input->data[i]);
    }
    
    return output;
}

Tensor* ReLU::backward(Tensor* grad_output) {
    // grad_input = grad_output * (input > 0)
    Tensor* grad_input = tensor_create(grad_output->shape, grad_output->ndim);
    for (int i = 0; i < grad_output->size; i++) {
        grad_input->data[i] = grad_output->data[i] * (input_cache->data[i] > 0 ? 1.0f : 0.0f);
    }
    
    return grad_input;
}

// ============================================
// Sigmoid Layer
// ============================================

Sigmoid::~Sigmoid() {
    if (output_cache) tensor_free(output_cache);
}

Tensor* Sigmoid::forward(Tensor* input) {
    // output = 1 / (1 + exp(-input))
    Tensor* output = tensor_create(input->shape, input->ndim);
    for (int i = 0; i < input->size; i++) {
        output->data[i] = 1.0f / (1.0f + std::exp(-input->data[i]));
    }
    
    // Cache output for backward pass
    if (output_cache) tensor_free(output_cache);
    output_cache = tensor_copy(output);
    
    return output;
}

Tensor* Sigmoid::backward(Tensor* grad_output) {
    // grad_input = grad_output * output * (1 - output)
    Tensor* grad_input = tensor_create(grad_output->shape, grad_output->ndim);
    for (int i = 0; i < grad_output->size; i++) {
        float sig = output_cache->data[i];
        grad_input->data[i] = grad_output->data[i] * sig * (1.0f - sig);
    }
    
    return grad_input;
}

// ============================================
// Softmax Layer
// ============================================

Softmax::~Softmax() {
    if (output_cache) tensor_free(output_cache);
}

Tensor* Softmax::forward(Tensor* input) {
    // output = exp(input) / sum(exp(input))
    Tensor* output = tensor_create(input->shape, input->ndim);
    
    if (input->ndim == 2) {
        // Batch processing
        for (int i = 0; i < input->shape[0]; i++) {
            // Find max for numerical stability
            float max_val = input->data[i * input->shape[1]];
            for (int j = 1; j < input->shape[1]; j++) {
                max_val = std::max(max_val, input->data[i * input->shape[1] + j]);
            }
            
            // Compute exp and sum
            float sum = 0.0f;
            for (int j = 0; j < input->shape[1]; j++) {
                float exp_val = std::exp(input->data[i * input->shape[1] + j] - max_val);
                output->data[i * input->shape[1] + j] = exp_val;
                sum += exp_val;
            }
            
            // Normalize
            for (int j = 0; j < input->shape[1]; j++) {
                output->data[i * input->shape[1] + j] /= sum;
            }
        }
    }
    
    // Cache output for backward pass
    if (output_cache) tensor_free(output_cache);
    output_cache = tensor_copy(output);
    
    return output;
}

Tensor* Softmax::backward(Tensor* grad_output) {
    // Simplified: grad_input = grad_output (usually combined with cross-entropy loss)
    return tensor_copy(grad_output);
}

} // namespace NuxAI
