// C++ - Model Implementation

#include "model.h"
#include <iostream>
#include <cmath>

namespace NuxAI {

Model::~Model() {
    for (Layer* layer : layers) {
        delete layer;
    }
}

void Model::add(Layer* layer) {
    layers.push_back(layer);
}

Tensor* Model::forward(Tensor* input) {
    Tensor* output = input;
    
    for (Layer* layer : layers) {
        Tensor* next_output = layer->forward(output);
        if (output != input) {
            tensor_free(output);
        }
        output = next_output;
    }
    
    return output;
}

void Model::backward(Tensor* grad_output) {
    Tensor* grad = grad_output;
    
    // Backward through layers in reverse order
    for (int i = layers.size() - 1; i >= 0; i--) {
        Tensor* next_grad = layers[i]->backward(grad);
        if (grad != grad_output) {
            tensor_free(grad);
        }
        grad = next_grad;
    }
    
    tensor_free(grad);
}

void Model::update(float learning_rate) {
    for (Layer* layer : layers) {
        layer->update(learning_rate);
    }
}

void Model::summary() const {
    std::cout << "Model Summary:" << std::endl;
    std::cout << "============================================" << std::endl;
    
    for (size_t i = 0; i < layers.size(); i++) {
        std::cout << "Layer " << i << ": " << layers[i]->get_name() << std::endl;
    }
    
    std::cout << "============================================" << std::endl;
    std::cout << "Total layers: " << layers.size() << std::endl;
}

// ============================================
// MSE Loss
// ============================================

float MSELoss::compute(Tensor* predictions, Tensor* targets) {
    float loss = 0.0f;
    for (int i = 0; i < predictions->size; i++) {
        float diff = predictions->data[i] - targets->data[i];
        loss += diff * diff;
    }
    return loss / predictions->size;
}

Tensor* MSELoss::gradient(Tensor* predictions, Tensor* targets) {
    Tensor* grad = tensor_create(predictions->shape, predictions->ndim);
    for (int i = 0; i < predictions->size; i++) {
        grad->data[i] = 2.0f * (predictions->data[i] - targets->data[i]) / predictions->size;
    }
    return grad;
}

// ============================================
// Cross Entropy Loss
// ============================================

float CrossEntropyLoss::compute(Tensor* predictions, Tensor* targets) {
    float loss = 0.0f;
    
    if (predictions->ndim == 2) {
        // Batch processing
        for (int i = 0; i < predictions->shape[0]; i++) {
            for (int j = 0; j < predictions->shape[1]; j++) {
                int idx = i * predictions->shape[1] + j;
                if (targets->data[idx] > 0) {
                    loss -= targets->data[idx] * std::log(predictions->data[idx] + 1e-10f);
                }
            }
        }
        loss /= predictions->shape[0];
    }
    
    return loss;
}

Tensor* CrossEntropyLoss::gradient(Tensor* predictions, Tensor* targets) {
    // Simplified gradient (assumes softmax + cross-entropy)
    Tensor* grad = tensor_create(predictions->shape, predictions->ndim);
    
    for (int i = 0; i < predictions->size; i++) {
        grad->data[i] = predictions->data[i] - targets->data[i];
    }
    
    if (predictions->ndim == 2) {
        // Normalize by batch size
        for (int i = 0; i < grad->size; i++) {
            grad->data[i] /= predictions->shape[0];
        }
    }
    
    return grad;
}

} // namespace NuxAI
