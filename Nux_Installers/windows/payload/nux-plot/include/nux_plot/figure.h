#ifndef NUX_PLOT_FIGURE_H
#define NUX_PLOT_FIGURE_H

#include <string>
#include <vector>
#include <memory>

namespace NuxPlot {

struct PlotData {
    std::vector<double> x;
    std::vector<double> y;
    std::string label;
    std::string color;
    std::string marker;
    float lineWidth;
};

class Figure {
public:
    Figure(int width = 800, int height = 600);
    ~Figure();
    
    // Plotting functions
    void Plot(const std::vector<double>& x, const std::vector<double>& y,
              const std::string& color = "blue", const std::string& label = "",
              float lineWidth = 2.0f);
    
    void Scatter(const std::vector<double>& x, const std::vector<double>& y,
                 const std::string& color = "blue", const std::string& marker = "o",
                 const std::string& label = "");
    
    void Bar(const std::vector<double>& x, const std::vector<double>& heights,
             const std::string& color = "blue", const std::string& label = "");
    
    void Histogram(const std::vector<double>& data, int bins = 10,
                   const std::string& color = "blue", const std::string& label = "");
    
    // Labels and title
    void SetTitle(const std::string& title);
    void SetXLabel(const std::string& label);
    void SetYLabel(const std::string& label);
    
    // Legend
    void Legend();
    
    // Grid
    void Grid(bool enable = true);
    
    // Limits
    void SetXLim(double min, double max);
    void SetYLim(double min, double max);
    
    // Save and show
    void Save(const std::string& filename);
    void Show();
    
    // Clear
    void Clear();
    
private:
    void GenerateSVG(const std::string& filename);
    void ComputeAutoLimits();
    
    int m_Width;
    int m_Height;
    std::string m_Title;
    std::string m_XLabel;
    std::string m_YLabel;
    bool m_ShowGrid;
    bool m_ShowLegend;
    
    double m_XMin, m_XMax;
    double m_YMin, m_YMax;
    bool m_AutoLimits;
    
    std::vector<PlotData> m_Plots;
};

// Convenience functions
Figure* figure(int width = 800, int height = 600);
void plot(const std::vector<double>& x, const std::vector<double>& y,
          const std::string& color = "blue");
void scatter(const std::vector<double>& x, const std::vector<double>& y,
             const std::string& color = "blue");
void show();
void savefig(const std::string& filename);

} // namespace NuxPlot

#endif // NUX_PLOT_FIGURE_H
