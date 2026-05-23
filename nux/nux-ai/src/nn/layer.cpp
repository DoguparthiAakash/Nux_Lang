#include "nux_ai/nn/layer.h"

namespace NuxAI {
namespace NN {

Layer::Layer(const std::string& name)
    : m_Name(name)
    , m_Training(true)
{
}

Layer::~Layer() {
}

std::vector<Tensor*> Layer::Parameters() {
    return std::vector<Tensor*>();
}

void Layer::ZeroGrad() {
    for (auto* param : Parameters()) {
        param->ZeroGrad();
    }
}

} // namespace NN
} // namespace NuxAI
