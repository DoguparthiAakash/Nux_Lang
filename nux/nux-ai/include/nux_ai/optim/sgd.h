#ifndef NUX_AI_OPTIM_SGD_H
#define NUX_AI_OPTIM_SGD_H

#include "optimizer.h"

namespace NuxAI {
namespace Optim {

class SGD : public Optimizer {
public:
    SGD(const std::vector<Tensor*>& parameters, float learningRate, float momentum = 0.0f);
    virtual ~SGD();
    
    virtual void Step() override;
    
private:
    float m_Momentum;
    std::vector<Tensor> m_Velocities;
};

} // namespace Optim
} // namespace NuxAI

#endif // NUX_AI_OPTIM_SGD_H
