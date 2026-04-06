#include "nux_learn/neighbors.h"
#include <cmath>
#include <algorithm>
#include <map>

namespace NuxLearn {

// KNeighborsClassifier
KNeighborsClassifier::KNeighborsClassifier(int nNeighbors)
    : m_NNeighbors(nNeighbors)
{
}

KNeighborsClassifier::~KNeighborsClassifier() {
}

double KNeighborsClassifier::Distance(const std::vector<double>& a, const std::vector<double>& b) const {
    double sum = 0.0;
    for (size_t i = 0; i < a.size(); i++) {
        double diff = a[i] - b[i];
        sum += diff * diff;
    }
    return std::sqrt(sum);
}

void KNeighborsClassifier::Fit(const std::vector<std::vector<double>>& X, const std::vector<int>& y) {
    m_XTrain = X;
    m_YTrain = y;
}

int KNeighborsClassifier::Predict(const std::vector<double>& x) const {
    // Compute distances to all training points
    std::vector<std::pair<double, int>> distances;
    for (size_t i = 0; i < m_XTrain.size(); i++) {
        double dist = Distance(x, m_XTrain[i]);
        distances.push_back({dist, m_YTrain[i]});
    }
    
    // Sort by distance
    std::sort(distances.begin(), distances.end());
    
    // Count votes from k nearest neighbors
    std::map<int, int> votes;
    for (int i = 0; i < m_NNeighbors && i < static_cast<int>(distances.size()); i++) {
        votes[distances[i].second]++;
    }
    
    // Return most common class
    int maxVotes = 0;
    int prediction = 0;
    for (const auto& pair : votes) {
        if (pair.second > maxVotes) {
            maxVotes = pair.second;
            prediction = pair.first;
        }
    }
    
    return prediction;
}

std::vector<int> KNeighborsClassifier::Predict(const std::vector<std::vector<double>>& X) const {
    std::vector<int> predictions;
    for (const auto& x : X) {
        predictions.push_back(Predict(x));
    }
    return predictions;
}

double KNeighborsClassifier::Score(const std::vector<std::vector<double>>& X, const std::vector<int>& y) const {
    auto predictions = Predict(X);
    int correct = 0;
    for (size_t i = 0; i < y.size(); i++) {
        if (predictions[i] == y[i]) correct++;
    }
    return static_cast<double>(correct) / y.size();
}

// KNeighborsRegressor
KNeighborsRegressor::KNeighborsRegressor(int nNeighbors)
    : m_NNeighbors(nNeighbors)
{
}

KNeighborsRegressor::~KNeighborsRegressor() {
}

double KNeighborsRegressor::Distance(const std::vector<double>& a, const std::vector<double>& b) const {
    double sum = 0.0;
    for (size_t i = 0; i < a.size(); i++) {
        double diff = a[i] - b[i];
        sum += diff * diff;
    }
    return std::sqrt(sum);
}

void KNeighborsRegressor::Fit(const std::vector<std::vector<double>>& X, const std::vector<double>& y) {
    m_XTrain = X;
    m_YTrain = y;
}

double KNeighborsRegressor::Predict(const std::vector<double>& x) const {
    // Compute distances to all training points
    std::vector<std::pair<double, double>> distances;
    for (size_t i = 0; i < m_XTrain.size(); i++) {
        double dist = Distance(x, m_XTrain[i]);
        distances.push_back({dist, m_YTrain[i]});
    }
    
    // Sort by distance
    std::sort(distances.begin(), distances.end());
    
    // Average k nearest neighbors
    double sum = 0.0;
    for (int i = 0; i < m_NNeighbors && i < static_cast<int>(distances.size()); i++) {
        sum += distances[i].second;
    }
    
    return sum / std::min(m_NNeighbors, static_cast<int>(distances.size()));
}

std::vector<double> KNeighborsRegressor::Predict(const std::vector<std::vector<double>>& X) const {
    std::vector<double> predictions;
    for (const auto& x : X) {
        predictions.push_back(Predict(x));
    }
    return predictions;
}

} // namespace NuxLearn
