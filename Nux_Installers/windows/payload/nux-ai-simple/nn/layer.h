// C++ - Neural Network Layer Base Class
// Clean, simple, easy to understand

#ifndef NUX_LAYER_H
#define NUX_LAYER_H

#include "../core/tensor.h"
#include <string>
#include <vector>

namespace NuxAI {

// Base class for all neural network layers
class Layer {
protected:
    std::string name;
    bool trainable;
    
public:
    Layer(const std::string& layer_name) 
        : name(layer_name), trainable(true) {}
    
    virtual ~Layer() {}
    
    // Forward pass: compute output from input
    virtual Tensor* forward(Tensor* input) = 0;
    
    // Backward pass: compute gradients
    virtual Tensor* backward(Tensor* grad_output) = 0;
    
    // Update parameters (called by optimizer)
    virtual void update(float learning_rate) = 0;
    
    // Get layer name
    std::string get_name() const { return name; }
    
    // Set trainable flag
    void set_trainable(bool train) { trainable = train; }
    bool is_trainable() const { return trainable; }
};

// Linear (Dense/Fully-Connected) Layer
class Linear : public Layer {
private:
    int in_features;
    int out_features;
    
    Tensor* weights;        // Shape: [in_features, out_features]
    Tensor* bias;           // Shape: [out_features]
    
    Tensor* grad_weights;   // Gradient for weights
    Tensor* grad_bias;      // Gradient for bias
    
    Tensor* input_cache;    // Cached for backward pass
    
public:
    Linear(int in_feat, int out_feat);
    ~Linear();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
    
    // Xavier initialization
    void init_weights();
};

// ReLU Activation Layer
class ReLU : public Layer {
private:
    Tensor* input_cache;    // Cached for backward pass
    
public:
    ReLU() : Layer("ReLU") {}
    ~ReLU();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override {} // No parameters
};

// Sigmoid Activation Layer
class Sigmoid : public Layer {
private:
    Tensor* output_cache;   // Cached for backward pass
    
public:
    Sigmoid() : Layer("Sigmoid") {}
    ~Sigmoid();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override {} // No parameters
};

// Softmax Activation Layer
class Softmax : public Layer {
private:
    Tensor* output_cache;   // Cached for backward pass
    
public:
    Softmax() : Layer("Softmax") {}
    ~Softmax();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override {} // No parameters
};

} // namespace NuxAI

#endif // NUX_LAYER_H
