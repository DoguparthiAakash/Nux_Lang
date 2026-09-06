#include "nux_learn/clustering.h"
#include <cmath>
#include <random>
#include <limits>
#include <algorithm>

namespace NuxLearn {

KMeans::KMeans(int nClusters, int maxIter, int randomState)
    : m_NClusters(nClusters)
    , m_MaxIter(maxIter)
    , m_RandomState(randomState)
    , m_Inertia(0.0)
{
}

KMeans::~KMeans() {
}

double KMeans::Distance(const std::vector<double>& a, const std::vector<double>& b) const {
    double sum = 0.0;
    for (size_t i = 0; i < a.size(); i++) {
        double diff = a[i] - b[i];
        sum += diff * diff;
    }
    return std::sqrt(sum);
}

int KMeans::FindNearestCenter(const std::vector<double>& point) const {
    int nearest = 0;
    double minDist = std::numeric_limits<double>::max();
    
    for (size_t i = 0; i < m_Centers.size(); i++) {
        double dist = Distance(point, m_Centers[i]);
        if (dist < minDist) {
            minDist = dist;
            nearest = i;
        }
    }
    
    return nearest;
}

void KMeans::Fit(const std::vector<std::vector<double>>& X) {
    if (X.empty()) return;
    
    int n = X.size();
    int m = X[0].size();
    
    // Initialize centers randomly
    std::mt19937 gen(m_RandomState);
    std::uniform_int_distribution<> dis(0, n - 1);
    
    m_Centers.clear();
    for (int i = 0; i < m_NClusters; i++) {
        m_Centers.push_back(X[dis(gen)]);
    }
    
    // K-Means iterations
    for (int iter = 0; iter < m_MaxIter; iter++) {
        // Assign points to clusters
        std::vector<int> labels(n);
        for (int i = 0; i < n; i++) {
            labels[i] = FindNearestCenter(X[i]);
        }
        
        // Update centers
        std::vector<std::vector<double>> newCenters(m_NClusters, std::vector<double>(m, 0.0));
        std::vector<int> counts(m_NClusters, 0);
        
        for (int i = 0; i < n; i++) {
            int cluster = labels[i];
            counts[cluster]++;
            for (int j = 0; j < m; j++) {
                newCenters[cluster][j] += X[i][j];
            }
        }
        
        for (int i = 0; i < m_NClusters; i++) {
            if (counts[i] > 0) {
                for (int j = 0; j < m; j++) {
                    newCenters[i][j] /= counts[i];
                }
            }
        }
        
        // Check convergence
        bool converged = true;
        for (int i = 0; i < m_NClusters; i++) {
            if (Distance(m_Centers[i], newCenters[i]) > 1e-6) {
                converged = false;
                break;
            }
        }
        
        m_Centers = newCenters;
        
        if (converged) break;
    }
    
    // Compute inertia
    m_Inertia = 0.0;
    for (const auto& point : X) {
        int cluster = FindNearestCenter(point);
        double dist = Distance(point, m_Centers[cluster]);
        m_Inertia += dist * dist;
    }
}

std::vector<int> KMeans::Predict(const std::vector<std::vector<double>>& X) const {
    std::vector<int> labels;
    for (const auto& point : X) {
        labels.push_back(FindNearestCenter(point));
    }
    return labels;
}

int KMeans::Predict(const std::vector<double>& x) const {
    return FindNearestCenter(x);
}

} // namespace NuxLearn
