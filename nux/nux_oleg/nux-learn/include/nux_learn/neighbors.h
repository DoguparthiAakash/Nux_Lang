#ifndef NUX_LEARN_NEIGHBORS_H
#define NUX_LEARN_NEIGHBORS_H

#include <vector>

namespace NuxLearn {

class KNeighborsClassifier {
public:
    KNeighborsClassifier(int nNeighbors = 5);
    ~KNeighborsClassifier();
    
    // Training
    void Fit(const std::vector<std::vector<double>>& X, const std::vector<int>& y);
    
    // Prediction
    std::vector<int> Predict(const std::vector<std::vector<double>>& X) const;
    int Predict(const std::vector<double>& x) const;
    
    // Accuracy
    double Score(const std::vector<std::vector<double>>& X, const std::vector<int>& y) const;
    
private:
    double Distance(const std::vector<double>& a, const std::vector<double>& b) const;
    
    int m_NNeighbors;
    std::vector<std::vector<double>> m_XTrain;
    std::vector<int> m_YTrain;
};

class KNeighborsRegressor {
public:
    KNeighborsRegressor(int nNeighbors = 5);
    ~KNeighborsRegressor();
    
    // Training
    void Fit(const std::vector<std::vector<double>>& X, const std::vector<double>& y);
    
    // Prediction
    std::vector<double> Predict(const std::vector<std::vector<double>>& X) const;
    double Predict(const std::vector<double>& x) const;
    
private:
    double Distance(const std::vector<double>& a, const std::vector<double>& b) const;
    
    int m_NNeighbors;
    std::vector<std::vector<double>> m_XTrain;
    std::vector<double> m_YTrain;
};

} // namespace NuxLearn

#endif // NUX_LEARN_NEIGHBORS_H
