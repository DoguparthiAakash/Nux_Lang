#include "nux_ai/tensor.h"
#include <iostream>
#include <cmath>
#include <random>
#include <stdexcept>
#include <numeric>
#include <algorithm>

namespace NuxAI {

// Constructors
Tensor::Tensor()
    : m_Shape({0})
    , m_Size(0)
    , m_Data(nullptr)
    , m_RequiresGrad(false)
{
}

Tensor::Tensor(const std::vector<int>& shape)
    : m_Shape(shape)
    , m_Size(ComputeSize(shape))
    , m_Data(new float[m_Size]())
    , m_RequiresGrad(false)
{
}

Tensor::Tensor(const std::vector<int>& shape, float value)
    : m_Shape(shape)
    , m_Size(ComputeSize(shape))
    , m_Data(new float[m_Size])
    , m_RequiresGrad(false)
{
    Fill(value);
}

Tensor::Tensor(const std::vector<int>& shape, const std::vector<float>& data)
    : m_Shape(shape)
    , m_Size(ComputeSize(shape))
    , m_Data(new float[m_Size])
    , m_RequiresGrad(false)
{
    if (data.size() != static_cast<size_t>(m_Size)) {
        throw std::invalid_argument("Data size doesn't match tensor shape");
    }
    std::copy(data.begin(), data.end(), m_Data.get());
}

Tensor::Tensor(std::initializer_list<std::initializer_list<float>> data)
    : m_RequiresGrad(false)
{
    int rows = data.size();
    int cols = data.begin()->size();
    m_Shape = {rows, cols};
    m_Size = rows * cols;
    m_Data.reset(new float[m_Size]);
    
    int idx = 0;
    for (const auto& row : data) {
        for (float val : row) {
            m_Data[idx++] = val;
        }
    }
}

Tensor::~Tensor() {
}

// Shape and size
int Tensor::Dim(int axis) const {
    if (axis < 0) axis += m_Shape.size();
    return m_Shape[axis];
}

// Data access
float& Tensor::At(const std::vector<int>& indices) {
    return m_Data[ComputeIndex(indices)];
}

float Tensor::At(const std::vector<int>& indices) const {
    return m_Data[ComputeIndex(indices)];
}

float Tensor::Item() const {
    if (m_Size != 1) {
        throw std::runtime_error("Item() only works for scalar tensors");
    }
    return m_Data[0];
}

// Basic operations
Tensor Tensor::operator+(const Tensor& other) const {
    if (m_Shape != other.m_Shape) {
        throw std::invalid_argument("Shape mismatch for addition");
    }
    
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = m_Data[i] + other.m_Data[i];
    }
    
    return result;
}

Tensor Tensor::operator-(const Tensor& other) const {
    if (m_Shape != other.m_Shape) {
        throw std::invalid_argument("Shape mismatch for subtraction");
    }
    
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = m_Data[i] - other.m_Data[i];
    }
    
    return result;
}

Tensor Tensor::operator*(const Tensor& other) const {
    if (m_Shape != other.m_Shape) {
        throw std::invalid_argument("Shape mismatch for multiplication");
    }
    
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = m_Data[i] * other.m_Data[i];
    }
    
    return result;
}

Tensor Tensor::operator*(float scalar) const {
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = m_Data[i] * scalar;
    }
    return result;
}

// Matrix multiplication
Tensor Tensor::MatMul(const Tensor& other) const {
    if (m_Shape.size() != 2 || other.m_Shape.size() != 2) {
        throw std::invalid_argument("MatMul requires 2D tensors");
    }
    if (m_Shape[1] != other.m_Shape[0]) {
        throw std::invalid_argument("Inner dimensions must match for MatMul");
    }
    
    int m = m_Shape[0];
    int n = other.m_Shape[1];
    int k = m_Shape[1];
    
    Tensor result({m, n});
    
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            float sum = 0.0f;
            for (int p = 0; p < k; p++) {
                sum += m_Data[i * k + p] * other.m_Data[p * n + j];
            }
            result.m_Data[i * n + j] = sum;
        }
    }
    
    return result;
}

// Reduction operations
Tensor Tensor::Sum(int axis) const {
    if (axis == -1) {
        // Sum all elements
        float sum = 0.0f;
        for (int i = 0; i < m_Size; i++) {
            sum += m_Data[i];
        }
        return Tensor({1}, sum);
    }
    
    // TODO: Implement axis-specific sum
    throw std::runtime_error("Axis-specific sum not yet implemented");
}

Tensor Tensor::Mean(int axis) const {
    Tensor sum_tensor = Sum(axis);
    return sum_tensor * (1.0f / m_Size);
}

// Element-wise functions
Tensor Tensor::Sqrt() const {
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = std::sqrt(m_Data[i]);
    }
    return result;
}

Tensor Tensor::Exp() const {
    Tensor result(m_Shape);
    for (int i = 0; i < m_Size; i++) {
        result.m_Data[i] = std::exp(m_Data[i]);
    }
    return result;
}

Tensor Tensor::Transpose() const {
    if (m_Shape.size() != 2) {
        throw std::runtime_error("Transpose currently only supports 2D tensors");
    }
    
    int rows = m_Shape[0];
    int cols = m_Shape[1];
    Tensor result({cols, rows});
    
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            result.m_Data[j * rows + i] = m_Data[i * cols + j];
        }
    }
    
    return result;
}

// Autograd
void Tensor::ZeroGrad() {
    if (m_Grad) {
        m_Grad->Fill(0.0f);
    }
}

void Tensor::Backward() {
    if (!m_RequiresGrad) return;
    
    if (!m_Grad) {
        m_Grad = std::make_shared<Tensor>(m_Shape, 1.0f);
    }
    
    if (m_BackwardFn) {
        m_BackwardFn();
    }
}

// Utility
void Tensor::Fill(float value) {
    for (int i = 0; i < m_Size; i++) {
        m_Data[i] = value;
    }
}

void Tensor::Print() const {
    std::cout << "Tensor(shape=[";
    for (size_t i = 0; i < m_Shape.size(); i++) {
        std::cout << m_Shape[i];
        if (i < m_Shape.size() - 1) std::cout << ", ";
    }
    std::cout << "], data=[";
    
    int printLimit = std::min(m_Size, 10);
    for (int i = 0; i < printLimit; i++) {
        std::cout << m_Data[i];
        if (i < printLimit - 1) std::cout << ", ";
    }
    if (m_Size > printLimit) std::cout << ", ...";
    std::cout << "])" << std::endl;
}

// Static factory methods
Tensor Tensor::Zeros(const std::vector<int>& shape) {
    return Tensor(shape, 0.0f);
}

Tensor Tensor::Ones(const std::vector<int>& shape) {
    return Tensor(shape, 1.0f);
}

Tensor Tensor::Random(const std::vector<int>& shape, float min, float max) {
    Tensor result(shape);
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> dist(min, max);
    
    for (int i = 0; i < result.m_Size; i++) {
        result.m_Data[i] = dist(gen);
    }
    
    return result;
}

Tensor Tensor::Randn(const std::vector<int>& shape, float mean, float stddev) {
    Tensor result(shape);
    std::random_device rd;
    std::mt19937 gen(rd());
    std::normal_distribution<float> dist(mean, stddev);
    
    for (int i = 0; i < result.m_Size; i++) {
        result.m_Data[i] = dist(gen);
    }
    
    return result;
}

// Helper methods
int Tensor::ComputeSize(const std::vector<int>& shape) const {
    return std::accumulate(shape.begin(), shape.end(), 1, std::multiplies<int>());
}

int Tensor::ComputeIndex(const std::vector<int>& indices) const {
    int index = 0;
    int stride = 1;
    for (int i = m_Shape.size() - 1; i >= 0; i--) {
        index += indices[i] * stride;
        stride *= m_Shape[i];
    }
    return index;
}

} // namespace NuxAI
