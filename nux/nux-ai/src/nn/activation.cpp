#include "nux_ai/nn/activation.h"
#include <cmath>
#include <algorithm>
#include <stdexcept>

namespace NuxAI {
namespace NN {

Tensor ReLU(const Tensor& input) {
    Tensor output(input.Shape());
    const float* inData = input.Data();
    float* outData = output.Data();
    
    for (int i = 0; i < input.Size(); i++) {
        outData[i] = std::max(0.0f, inData[i]);
    }
    
    return output;
}

Tensor LeakyReLU(const Tensor& input, float alpha) {
    Tensor output(input.Shape());
    const float* inData = input.Data();
    float* outData = output.Data();
    
    for (int i = 0; i < input.Size(); i++) {
        outData[i] = inData[i] > 0 ? inData[i] : alpha * inData[i];
    }
    
    return output;
}

Tensor Sigmoid(const Tensor& input) {
    Tensor output(input.Shape());
    const float* inData = input.Data();
    float* outData = output.Data();
    
    for (int i = 0; i < input.Size(); i++) {
        outData[i] = 1.0f / (1.0f + std::exp(-inData[i]));
    }
    
    return output;
}

Tensor Tanh(const Tensor& input) {
    Tensor output(input.Shape());
    const float* inData = input.Data();
    float* outData = output.Data();
    
    for (int i = 0; i < input.Size(); i++) {
        outData[i] = std::tanh(inData[i]);
    }
    
    return output;
}

Tensor Softmax(const Tensor& input, int axis) {
    // For simplicity, implement for 2D tensors along last axis
    if (input.NumDims() != 2) {
        throw std::runtime_error("Softmax currently only supports 2D tensors");
    }
    
    Tensor output(input.Shape());
    int rows = input.Dim(0);
    int cols = input.Dim(1);
    
    const float* inData = input.Data();
    float* outData = output.Data();
    
    for (int i = 0; i < rows; i++) {
        // Find max for numerical stability
        float maxVal = inData[i * cols];
        for (int j = 1; j < cols; j++) {
            maxVal = std::max(maxVal, inData[i * cols + j]);
        }
        
        // Compute exp and sum
        float sum = 0.0f;
        for (int j = 0; j < cols; j++) {
            outData[i * cols + j] = std::exp(inData[i * cols + j] - maxVal);
            sum += outData[i * cols + j];
        }
        
        // Normalize
        for (int j = 0; j < cols; j++) {
            outData[i * cols + j] /= sum;
        }
    }
    
    return output;
}

Tensor MSELoss(const Tensor& pred, const Tensor& target) {
    if (pred.Shape() != target.Shape()) {
        throw std::invalid_argument("Prediction and target shapes must match");
    }
    
    Tensor diff = pred - target;
    Tensor squared = diff * diff;
    return squared.Mean();
}

Tensor CrossEntropyLoss(const Tensor& pred, const Tensor& target) {
    // pred: [batch_size, num_classes] (logits or probabilities)
    // target: [batch_size] (class indices) or [batch_size, num_classes] (one-hot)
    
    // Apply softmax to predictions
    Tensor probs = Softmax(pred);
    
    // Compute negative log likelihood
    // For simplicity, assume target is class indices
    Tensor loss = Tensor::Zeros({1});
    
    // TODO: Implement proper cross-entropy calculation
    // This is a placeholder
    
    return loss;
}

Tensor BinaryCrossEntropyLoss(const Tensor& pred, const Tensor& target) {
    if (pred.Shape() != target.Shape()) {
        throw std::invalid_argument("Prediction and target shapes must match");
    }
    
    const float* predData = pred.Data();
    const float* targetData = target.Data();
    float loss = 0.0f;
    
    for (int i = 0; i < pred.Size(); i++) {
        float p = std::max(1e-7f, std::min(1.0f - 1e-7f, predData[i]));  // Clip for stability
        loss += -(targetData[i] * std::log(p) + (1 - targetData[i]) * std::log(1 - p));
    }
    
    loss /= pred.Size();
    return Tensor({1}, loss);
}

} // namespace NN
} // namespace NuxAI
