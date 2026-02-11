#ifndef NUX_AI_TENSOR_H
#define NUX_AI_TENSOR_H

#include <vector>
#include <memory>
#include <initializer_list>
#include <functional>

namespace NuxAI {

class Tensor {
public:
    // Constructors
    Tensor();
    Tensor(const std::vector<int>& shape);
    Tensor(const std::vector<int>& shape, float value);
    Tensor(const std::vector<int>& shape, const std::vector<float>& data);
    Tensor(std::initializer_list<std::initializer_list<float>> data);  // 2D init
    ~Tensor();
    
    // Shape and size
    const std::vector<int>& Shape() const { return m_Shape; }
    int Dim(int axis) const;
    int NumDims() const { return m_Shape.size(); }
    int Size() const { return m_Size; }
    
    // Data access
    float* Data() { return m_Data.get(); }
    const float* Data() const { return m_Data.get(); }
    float& At(const std::vector<int>& indices);
    float At(const std::vector<int>& indices) const;
    float Item() const;  // For scalar tensors
    
    // Reshape and view
    Tensor Reshape(const std::vector<int>& newShape) const;
    Tensor Transpose() const;  // For 2D tensors
    Tensor Squeeze() const;    // Remove dimensions of size 1
    
    // Basic operations
    Tensor operator+(const Tensor& other) const;
    Tensor operator-(const Tensor& other) const;
    Tensor operator*(const Tensor& other) const;  // Element-wise
    Tensor operator/(const Tensor& other) const;
    Tensor operator+(float scalar) const;
    Tensor operator-(float scalar) const;
    Tensor operator*(float scalar) const;
    Tensor operator/(float scalar) const;
    
    // Matrix operations
    Tensor MatMul(const Tensor& other) const;
    Tensor Dot(const Tensor& other) const;
    
    // Reduction operations
    Tensor Sum(int axis = -1) const;
    Tensor Mean(int axis = -1) const;
    Tensor Max(int axis = -1) const;
    Tensor Min(int axis = -1) const;
    
    // Element-wise functions
    Tensor Sqrt() const;
    Tensor Exp() const;
    Tensor Log() const;
    Tensor Pow(float exponent) const;
    
    // Autograd support
    void SetRequiresGrad(bool requiresGrad) { m_RequiresGrad = requiresGrad; }
    bool RequiresGrad() const { return m_RequiresGrad; }
    void SetGrad(const Tensor& grad) { m_Grad = std::make_shared<Tensor>(grad); }
    Tensor* Grad() { return m_Grad.get(); }
    void ZeroGrad();
    void Backward();
    
    // Utility
    void Fill(float value);
    void Print() const;
    
    // Static factory methods
    static Tensor Zeros(const std::vector<int>& shape);
    static Tensor Ones(const std::vector<int>& shape);
    static Tensor Random(const std::vector<int>& shape, float min = 0.0f, float max = 1.0f);
    static Tensor Randn(const std::vector<int>& shape, float mean = 0.0f, float stddev = 1.0f);
    static Tensor Arange(float start, float end, float step = 1.0f);
    
private:
    std::vector<int> m_Shape;
    int m_Size;
    std::shared_ptr<float[]> m_Data;
    
    // Autograd
    bool m_RequiresGrad;
    std::shared_ptr<Tensor> m_Grad;
    std::function<void()> m_BackwardFn;
    
    // Helper methods
    int ComputeSize(const std::vector<int>& shape) const;
    int ComputeIndex(const std::vector<int>& indices) const;
    bool CanBroadcast(const Tensor& other) const;
    std::vector<int> BroadcastShape(const Tensor& other) const;
};

} // namespace NuxAI

#endif // NUX_AI_TENSOR_H
