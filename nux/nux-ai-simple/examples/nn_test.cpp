// C++ - Neural Network Test

#include "nn/model.h"
#include <iostream>

using namespace NuxAI;

int main() {
    std::cout << "=== Nux AI Neural Network Test ===" << std::endl << std::endl;
    
    // Create a simple model
    std::cout << "Building model..." << std::endl;
    Model model;
    model.add(new Linear(4, 8));
    model.add(new ReLU());
    model.add(new Linear(8, 3));
    model.add(new Softmax());
    
    model.summary();
    std::cout << std::endl;
    
    // Create input data
    std::cout << "Creating input data..." << std::endl;
    int input_shape[] = {1, 4};  // Batch size 1, 4 features
    Tensor* input = tensor_random(input_shape, 2);
    std::cout << "Input:" << std::endl;
    tensor_print(input);
    
    // Forward pass
    std::cout << "\nForward pass..." << std::endl;
    Tensor* output = model.forward(input);
    std::cout << "Output (probabilities):" << std::endl;
    tensor_print(output);
    
    // Create target
    int target_shape[] = {1, 3};
    Tensor* target = tensor_zeros(target_shape, 2);
    target->data[1] = 1.0f;  // One-hot encoding for class 1
    std::cout << "\nTarget:" << std::endl;
    tensor_print(target);
    
    // Compute loss
    CrossEntropyLoss loss_fn;
    float loss = loss_fn.compute(output, target);
    std::cout << "\nLoss: " << loss << std::endl;
    
    // Backward pass
    std::cout << "\nBackward pass..." << std::endl;
    Tensor* grad = loss_fn.gradient(output, target);
    model.backward(grad);
    
    // Update weights
    std::cout << "Updating weights (lr=0.01)..." << std::endl;
    model.update(0.01f);
    
    // Forward pass again
    std::cout << "\nForward pass after update..." << std::endl;
    Tensor* output2 = model.forward(input);
    std::cout << "New output:" << std::endl;
    tensor_print(output2);
    
    float loss2 = loss_fn.compute(output2, target);
    std::cout << "\nNew loss: " << loss2 << std::endl;
    std::cout << "Loss decreased: " << (loss2 < loss ? "YES" : "NO") << std::endl;
    
    // Cleanup
    tensor_free(input);
    tensor_free(output);
    tensor_free(output2);
    tensor_free(target);
    tensor_free(grad);
    
    std::cout << "\n=== Test complete! ===" << std::endl;
    return 0;
}
