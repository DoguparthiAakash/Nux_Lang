#ifndef NUX_AI_NN_ACTIVATION_H
#define NUX_AI_NN_ACTIVATION_H

#include "../tensor.h"

namespace NuxAI {
namespace NN {

// Activation functions
Tensor ReLU(const Tensor& input);
Tensor LeakyReLU(const Tensor& input, float alpha = 0.01f);
Tensor Sigmoid(const Tensor& input);
Tensor Tanh(const Tensor& input);
Tensor Softmax(const Tensor& input, int axis = -1);

// Loss functions
Tensor MSELoss(const Tensor& pred, const Tensor& target);
Tensor CrossEntropyLoss(const Tensor& pred, const Tensor& target);
Tensor BinaryCrossEntropyLoss(const Tensor& pred, const Tensor& target);

} // namespace NN
} // namespace NuxAI

#endif // NUX_AI_NN_ACTIVATION_H
