#ifndef NUX_LEARN_CLUSTERING_H
#define NUX_LEARN_CLUSTERING_H

#include <vector>

namespace NuxLearn {

class KMeans {
public:
    KMeans(int nClusters = 3, int maxIter = 300, int randomState = 42);
    ~KMeans();
    
    // Training
    void Fit(const std::vector<std::vector<double>>& X);
    
    // Prediction
    std::vector<int> Predict(const std::vector<std::vector<double>>& X) const;
    int Predict(const std::vector<double>& x) const;
    
    // Cluster centers
    const std::vector<std::vector<double>>& ClusterCenters() const { return m_Centers; }
    
    // Inertia (sum of squared distances to nearest cluster)
    double Inertia() const { return m_Inertia; }
    
private:
    double Distance(const std::vector<double>& a, const std::vector<double>& b) const;
    int FindNearestCenter(const std::vector<double>& point) const;
    
    int m_NClusters;
    int m_MaxIter;
    int m_RandomState;
    std::vector<std::vector<double>> m_Centers;
    double m_Inertia;
};

} // namespace NuxLearn

#endif // NUX_LEARN_CLUSTERING_H
