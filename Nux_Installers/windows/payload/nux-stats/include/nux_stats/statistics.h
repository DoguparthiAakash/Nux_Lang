#ifndef NUX_STATS_STATISTICS_H
#define NUX_STATS_STATISTICS_H

#include <vector>
#include <string>

namespace NuxStats {

// Probability distributions
class Distribution {
public:
    virtual ~Distribution() = default;
    virtual double PDF(double x) const = 0;  // Probability density function
    virtual double CDF(double x) const = 0;  // Cumulative distribution
    virtual double Sample() const = 0;
};

class NormalDistribution : public Distribution {
public:
    NormalDistribution(double mean = 0.0, double stddev = 1.0);
    double PDF(double x) const override;
    double CDF(double x) const override;
    double Sample() const override;
    
private:
    double m_Mean;
    double m_Stddev;
};

class UniformDistribution : public Distribution {
public:
    UniformDistribution(double min = 0.0, double max = 1.0);
    double PDF(double x) const override;
    double CDF(double x) const override;
    double Sample() const override;
    
private:
    double m_Min;
    double m_Max;
};

// Hypothesis testing
struct TTestResult {
    double statistic;
    double pvalue;
    double df;  // degrees of freedom
};

struct ChiSquareResult {
    double statistic;
    double pvalue;
    double df;
};

class HypothesisTesting {
public:
    // T-tests
    static TTestResult OneSampleTTest(const std::vector<double>& data, double mu);
    static TTestResult TwoSampleTTest(const std::vector<double>& data1, 
                                      const std::vector<double>& data2);
    static TTestResult PairedTTest(const std::vector<double>& data1,
                                   const std::vector<double>& data2);
    
    // Chi-square test
    static ChiSquareResult ChiSquareTest(const std::vector<std::vector<int>>& observed);
    
    // ANOVA
    static double ANOVA(const std::vector<std::vector<double>>& groups);
};

// Correlation and regression
class Correlation {
public:
    static double Pearson(const std::vector<double>& x, const std::vector<double>& y);
    static double Spearman(const std::vector<double>& x, const std::vector<double>& y);
    static double Kendall(const std::vector<double>& x, const std::vector<double>& y);
};

struct RegressionResult {
    std::vector<double> coefficients;
    double intercept;
    double r_squared;
    double adjusted_r_squared;
    std::vector<double> pvalues;
};

class Regression {
public:
    static RegressionResult OLS(const std::vector<std::vector<double>>& X,
                                const std::vector<double>& y);
    static RegressionResult Ridge(const std::vector<std::vector<double>>& X,
                                  const std::vector<double>& y, double alpha = 1.0);
    static RegressionResult Lasso(const std::vector<std::vector<double>>& X,
                                  const std::vector<double>& y, double alpha = 1.0);
};

// Time series analysis
class TimeSeries {
public:
    TimeSeries(const std::vector<double>& data);
    
    // Decomposition
    std::vector<double> Trend() const;
    std::vector<double> Seasonal(int period) const;
    std::vector<double> Residual() const;
    
    // Autocorrelation
    std::vector<double> ACF(int maxLag = 20) const;
    std::vector<double> PACF(int maxLag = 20) const;
    
    // Forecasting
    std::vector<double> MovingAverage(int window) const;
    std::vector<double> ExponentialSmoothing(double alpha) const;
    std::vector<double> ARIMA(int p, int d, int q, int steps) const;
    
private:
    std::vector<double> m_Data;
};

// Bayesian statistics
class BayesianInference {
public:
    // Prior, likelihood, posterior
    static std::vector<double> UpdatePosterior(
        const std::vector<double>& prior,
        const std::vector<double>& likelihood);
    
    // MCMC sampling
    static std::vector<double> MetropolisHastings(
        std::function<double(double)> targetDist,
        int numSamples, double proposalStd = 1.0);
};

} // namespace NuxStats

#endif // NUX_STATS_STATISTICS_H
