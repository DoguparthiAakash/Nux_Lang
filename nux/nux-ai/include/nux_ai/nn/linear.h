#ifndef NUX_AI_NN_LINEAR_H
#define NUX_AI_NN_LINEAR_H

#include "layer.h"

namespace NuxAI {
namespace NN {

class Linear : public Layer {
public:
    Linear(int inFeatures, int outFeatures, bool bias = true);
    virtual ~Linear();
    
    virtual Tensor Forward(const Tensor& input) override;
    virtual std::vector<Tensor*> Parameters() override;
    
    Tensor& Weight() { return m_Weight; }
    Tensor& Bias() { return m_Bias; }
    
private:
    int m_InFeatures;
    int m_OutFeatures;
    bool m_UseBias;
    Tensor m_Weight;  // Shape: [out_features, in_features]
    Tensor m_Bias;    // Shape: [out_features]
};

} // namespace NN
} // namespace NuxAI

#endif // NUX_AI_NN_LINEAR_H
