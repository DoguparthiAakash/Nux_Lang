#ifndef NUX_AI_NN_LAYER_H
#define NUX_AI_NN_LAYER_H

#include "../tensor.h"
#include <vector>
#include <string>

namespace NuxAI {
namespace NN {

class Layer {
public:
    Layer(const std::string& name = "Layer");
    virtual ~Layer();
    
    // Forward pass
    virtual Tensor Forward(const Tensor& input) = 0;
    
    // Parameters
    virtual std::vector<Tensor*> Parameters();
    virtual void ZeroGrad();
    
    // Training mode
    void SetTraining(bool training) { m_Training = training; }
    bool IsTraining() const { return m_Training; }
    
    // Name
    const std::string& Name() const { return m_Name; }
    
protected:
    std::string m_Name;
    bool m_Training;
};

} // namespace NN
} // namespace NuxAI

#endif // NUX_AI_NN_LAYER_H
