#ifndef NUX_AI_OPTIM_OPTIMIZER_H
#define NUX_AI_OPTIM_OPTIMIZER_H

#include "../tensor.h"
#include <vector>

namespace NuxAI {
namespace Optim {

class Optimizer {
public:
    Optimizer(const std::vector<Tensor*>& parameters, float learningRate);
    virtual ~Optimizer();
    
    virtual void Step() = 0;
    virtual void ZeroGrad();
    
    void SetLearningRate(float lr) { m_LearningRate = lr; }
    float GetLearningRate() const { return m_LearningRate; }
    
protected:
    std::vector<Tensor*> m_Parameters;
    float m_LearningRate;
};

} // namespace Optim
} // namespace NuxAI

#endif // NUX_AI_OPTIM_OPTIMIZER_H
