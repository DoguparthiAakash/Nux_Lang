#include "nux_ai/optim/sgd.h"

namespace NuxAI {
namespace Optim {

SGD::SGD(const std::vector<Tensor*>& parameters, float learningRate, float momentum)
    : Optimizer(parameters, learningRate)
    , m_Momentum(momentum)
{
    // Initialize velocities if using momentum
    if (m_Momentum > 0.0f) {
        for (auto* param : m_Parameters) {
            m_Velocities.push_back(Tensor::Zeros(param->Shape()));
        }
    }
}

SGD::~SGD() {
}

void SGD::Step() {
    for (size_t i = 0; i < m_Parameters.size(); i++) {
        Tensor* param = m_Parameters[i];
        Tensor* grad = param->Grad();
        
        if (!grad) continue;
        
        if (m_Momentum > 0.0f) {
            // Update velocity: v = momentum * v - lr * grad
            m_Velocities[i] = m_Velocities[i] * m_Momentum - (*grad) * m_LearningRate;
            // Update parameter: param = param + v
            *param = *param + m_Velocities[i];
        } else {
            // Simple SGD: param = param - lr * grad
            *param = *param - (*grad) * m_LearningRate;
        }
    }
}

} // namespace Optim
} // namespace NuxAI
