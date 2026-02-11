#include "nux_ai/optim/optimizer.h"

namespace NuxAI {
namespace Optim {

Optimizer::Optimizer(const std::vector<Tensor*>& parameters, float learningRate)
    : m_Parameters(parameters)
    , m_LearningRate(learningRate)
{
}

Optimizer::~Optimizer() {
}

void Optimizer::ZeroGrad() {
    for (auto* param : m_Parameters) {
        param->ZeroGrad();
    }
}

} // namespace Optim
} // namespace NuxAI
