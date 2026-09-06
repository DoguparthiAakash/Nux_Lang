// C++ - Neural Network Model Class

#ifndef NUX_MODEL_H
#define NUX_MODEL_H

#include "layer.h"
#include <vector>

namespace NuxAI {

class Model {
private:
    std::vector<Layer*> layers;
    
public:
    Model() {}
    ~Model();
    
    // Add a layer to the model
    void add(Layer* layer);
    
    // Forward pass through all layers
    Tensor* forward(Tensor* input);
    
    // Backward pass through all layers
    void backward(Tensor* grad_output);
    
    // Update all trainable layers
    void update(float learning_rate);
    
    // Get number of layers
    int num_layers() const { return layers.size(); }
    
    // Print model summary
    void summary() const;
};

// Loss functions
class Loss {
public:
    virtual ~Loss() {}
    
    // Compute loss
    virtual float compute(Tensor* predictions, Tensor* targets) = 0;
    
    // Compute gradient
    virtual Tensor* gradient(Tensor* predictions, Tensor* targets) = 0;
};

// Mean Squared Error Loss
class MSELoss : public Loss {
public:
    float compute(Tensor* predictions, Tensor* targets) override;
    Tensor* gradient(Tensor* predictions, Tensor* targets) override;
};

// Cross Entropy Loss
class CrossEntropyLoss : public Loss {
public:
    float compute(Tensor* predictions, Tensor* targets) override;
    Tensor* gradient(Tensor* predictions, Tensor* targets) override;
};

} // namespace NuxAI

#endif // NUX_MODEL_H
