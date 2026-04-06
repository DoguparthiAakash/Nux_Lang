#include "nux_learn/linear_model.h"
#include <cmath>
#include <stdexcept>

namespace NuxLearn {

// Linear Regression
LinearRegression::LinearRegression() : m_Intercept(0.0) {
}

LinearRegression::~LinearRegression() {
}

void LinearRegression::Fit(const std::vector<std::vector<double>>& X, const std::vector<double>& y) {
    if (X.empty() || y.empty() || X.size() != y.size()) {
        throw std::invalid_argument("Invalid input dimensions");
    }
    
    int n = X.size();
    int m = X[0].size();
    
    // Initialize coefficients
    m_Coefficients.resize(m, 0.0);
    m_Intercept = 0.0;
    
    // Compute means
    std::vector<double> X_mean(m, 0.0);
    double y_mean = 0.0;
    
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            X_mean[j] += X[i][j];
        }
        y_mean += y[i];
    }
    
    for (int j = 0; j < m; j++) {
        X_mean[j] /= n;
    }
    y_mean /= n;
    
    // Compute covariance matrix (simplified for single feature)
    if (m == 1) {
        double cov_xy = 0.0;
        double var_x = 0.0;
        
        for (int i = 0; i < n; i++) {
            cov_xy += (X[i][0] - X_mean[0]) * (y[i] - y_mean);
            var_x += (X[i][0] - X_mean[0]) * (X[i][0] - X_mean[0]);
        }
        
        m_Coefficients[0] = cov_xy / var_x;
        m_Intercept = y_mean - m_Coefficients[0] * X_mean[0];
    } else {
        // For multiple features, use gradient descent (simplified)
        double learningRate = 0.01;
        int iterations = 1000;
        
        for (int iter = 0; iter < iterations; iter++) {
            std::vector<double> grad_coef(m, 0.0);
            double grad_intercept = 0.0;
            
            for (int i = 0; i < n; i++) {
                double pred = m_Intercept;
                for (int j = 0; j < m; j++) {
                    pred += m_Coefficients[j] * X[i][j];
                }
                
                double error = pred - y[i];
                grad_intercept += error;
                for (int j = 0; j < m; j++) {
                    grad_coef[j] += error * X[i][j];
                }
            }
            
            m_Intercept -= learningRate * grad_intercept / n;
            for (int j = 0; j < m; j++) {
                m_Coefficients[j] -= learningRate * grad_coef[j] / n;
            }
        }
    }
}

std::vector<double> LinearRegression::Predict(const std::vector<std::vector<double>>& X) const {
    std::vector<double> predictions;
    for (const auto& x : X) {
        predictions.push_back(Predict(x));
    }
    return predictions;
}

double LinearRegression::Predict(const std::vector<double>& x) const {
    double pred = m_Intercept;
    for (size_t i = 0; i < x.size(); i++) {
        pred += m_Coefficients[i] * x[i];
    }
    return pred;
}

double LinearRegression::Score(const std::vector<std::vector<double>>& X, const std::vector<double>& y) const {
    auto predictions = Predict(X);
    
    // Compute R² score
    double y_mean = 0.0;
    for (double val : y) {
        y_mean += val;
    }
    y_mean /= y.size();
    
    double ss_tot = 0.0;
    double ss_res = 0.0;
    
    for (size_t i = 0; i < y.size(); i++) {
        ss_tot += (y[i] - y_mean) * (y[i] - y_mean);
        ss_res += (y[i] - predictions[i]) * (y[i] - predictions[i]);
    }
    
    return 1.0 - (ss_res / ss_tot);
}

// Logistic Regression
LogisticRegression::LogisticRegression(double learningRate, int maxIter)
    : m_Intercept(0.0)
    , m_LearningRate(learningRate)
    , m_MaxIter(maxIter)
{
}

LogisticRegression::~LogisticRegression() {
}

double LogisticRegression::Sigmoid(double z) const {
    return 1.0 / (1.0 + std::exp(-z));
}

void LogisticRegression::Fit(const std::vector<std::vector<double>>& X, const std::vector<int>& y) {
    if (X.empty() || y.empty() || X.size() != y.size()) {
        throw std::invalid_argument("Invalid input dimensions");
    }
    
    int n = X.size();
    int m = X[0].size();
    
    m_Coefficients.resize(m, 0.0);
    m_Intercept = 0.0;
    
    // Gradient descent
    for (int iter = 0; iter < m_MaxIter; iter++) {
        std::vector<double> grad_coef(m, 0.0);
        double grad_intercept = 0.0;
        
        for (int i = 0; i < n; i++) {
            double z = m_Intercept;
            for (int j = 0; j < m; j++) {
                z += m_Coefficients[j] * X[i][j];
            }
            
            double pred = Sigmoid(z);
            double error = pred - y[i];
            
            grad_intercept += error;
            for (int j = 0; j < m; j++) {
                grad_coef[j] += error * X[i][j];
            }
        }
        
        m_Intercept -= m_LearningRate * grad_intercept / n;
        for (int j = 0; j < m; j++) {
            m_Coefficients[j] -= m_LearningRate * grad_coef[j] / n;
        }
    }
}

std::vector<int> LogisticRegression::Predict(const std::vector<std::vector<double>>& X) const {
    std::vector<int> predictions;
    for (const auto& x : X) {
        double z = m_Intercept;
        for (size_t j = 0; j < x.size(); j++) {
            z += m_Coefficients[j] * x[j];
        }
        predictions.push_back(Sigmoid(z) >= 0.5 ? 1 : 0);
    }
    return predictions;
}

std::vector<double> LogisticRegression::PredictProba(const std::vector<std::vector<double>>& X) const {
    std::vector<double> probabilities;
    for (const auto& x : X) {
        double z = m_Intercept;
        for (size_t j = 0; j < x.size(); j++) {
            z += m_Coefficients[j] * x[j];
        }
        probabilities.push_back(Sigmoid(z));
    }
    return probabilities;
}

} // namespace NuxLearn
