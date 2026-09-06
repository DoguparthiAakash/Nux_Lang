#ifndef NUX_LEARN_LINEAR_MODEL_H
#define NUX_LEARN_LINEAR_MODEL_H

#include <vector>

namespace NuxLearn {

class LinearRegression {
public:
    LinearRegression();
    ~LinearRegression();
    
    // Training
    void Fit(const std::vector<std::vector<double>>& X, const std::vector<double>& y);
    
    // Prediction
    std::vector<double> Predict(const std::vector<std::vector<double>>& X) const;
    double Predict(const std::vector<double>& x) const;
    
    // Model parameters
    const std::vector<double>& Coefficients() const { return m_Coefficients; }
    double Intercept() const { return m_Intercept; }
    
    // Model evaluation
    double Score(const std::vector<std::vector<double>>& X, const std::vector<double>& y) const;
    
private:
    std::vector<double> m_Coefficients;
    double m_Intercept;
};

class LogisticRegression {
public:
    LogisticRegression(double learningRate = 0.01, int maxIter = 1000);
    ~LogisticRegression();
    
    // Training
    void Fit(const std::vector<std::vector<double>>& X, const std::vector<int>& y);
    
    // Prediction
    std::vector<int> Predict(const std::vector<std::vector<double>>& X) const;
    std::vector<double> PredictProba(const std::vector<std::vector<double>>& X) const;
    
    // Model parameters
    const std::vector<double>& Coefficients() const { return m_Coefficients; }
    double Intercept() const { return m_Intercept; }
    
private:
    double Sigmoid(double z) const;
    
    std::vector<double> m_Coefficients;
    double m_Intercept;
    double m_LearningRate;
    int m_MaxIter;
};

} // namespace NuxLearn

#endif // NUX_LEARN_LINEAR_MODEL_H
