#include "nux_ai/nn/linear.h"
#include <cmath>

namespace NuxAI {
namespace NN {

Linear::Linear(int inFeatures, int outFeatures, bool bias)
    : Layer("Linear")
    , m_InFeatures(inFeatures)
    , m_OutFeatures(outFeatures)
    , m_UseBias(bias)
{
    // Initialize weights with Xavier/Glorot initialization
    float stddev = std::sqrt(2.0f / (inFeatures + outFeatures));
    m_Weight = Tensor::Randn({outFeatures, inFeatures}, 0.0f, stddev);
    m_Weight.SetRequiresGrad(true);
    
    if (m_UseBias) {
        m_Bias = Tensor::Zeros({outFeatures});
        m_Bias.SetRequiresGrad(true);
    }
}

Linear::~Linear() {
}

Tensor Linear::Forward(const Tensor& input) {
    // input shape: [batch_size, in_features] or [in_features]
    // weight shape: [out_features, in_features]
    // output shape: [batch_size, out_features] or [out_features]
    
    Tensor output = input.MatMul(m_Weight.Transpose());
    
    if (m_UseBias) {
        // Broadcast bias addition
        output = output + m_Bias;
    }
    
    return output;
}

std::vector<Tensor*> Linear::Parameters() {
    std::vector<Tensor*> params;
    params.push_back(&m_Weight);
    if (m_UseBias) {
        params.push_back(&m_Bias);
    }
    return params;
}

} // namespace NN
} // namespace NuxAI
