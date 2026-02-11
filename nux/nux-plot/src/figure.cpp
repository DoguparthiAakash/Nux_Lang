#include "nux_plot/figure.h"
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cmath>
#include <iostream>

namespace NuxPlot {

static Figure* g_CurrentFigure = nullptr;

Figure::Figure(int width, int height)
    : m_Width(width)
    , m_Height(height)
    , m_ShowGrid(false)
    , m_ShowLegend(false)
    , m_XMin(0), m_XMax(1)
    , m_YMin(0), m_YMax(1)
    , m_AutoLimits(true)
{
}

Figure::~Figure() {
}

void Figure::Plot(const std::vector<double>& x, const std::vector<double>& y,
                  const std::string& color, const std::string& label, float lineWidth) {
    PlotData data;
    data.x = x;
    data.y = y;
    data.color = color;
    data.label = label;
    data.lineWidth = lineWidth;
    data.marker = "";
    m_Plots.push_back(data);
    
    if (!label.empty()) {
        m_ShowLegend = true;
    }
}

void Figure::Scatter(const std::vector<double>& x, const std::vector<double>& y,
                     const std::string& color, const std::string& marker,
                     const std::string& label) {
    PlotData data;
    data.x = x;
    data.y = y;
    data.color = color;
    data.marker = marker;
    data.label = label;
    data.lineWidth = 0;
    m_Plots.push_back(data);
    
    if (!label.empty()) {
        m_ShowLegend = true;
    }
}

void Figure::SetTitle(const std::string& title) {
    m_Title = title;
}

void Figure::SetXLabel(const std::string& label) {
    m_XLabel = label;
}

void Figure::SetYLabel(const std::string& label) {
    m_YLabel = label;
}

void Figure::Legend() {
    m_ShowLegend = true;
}

void Figure::Grid(bool enable) {
    m_ShowGrid = enable;
}

void Figure::SetXLim(double min, double max) {
    m_XMin = min;
    m_XMax = max;
    m_AutoLimits = false;
}

void Figure::SetYLim(double min, double max) {
    m_YMin = min;
    m_YMax = max;
    m_AutoLimits = false;
}

void Figure::ComputeAutoLimits() {
    if (!m_AutoLimits || m_Plots.empty()) return;
    
    m_XMin = m_XMax = m_Plots[0].x[0];
    m_YMin = m_YMax = m_Plots[0].y[0];
    
    for (const auto& plot : m_Plots) {
        for (double val : plot.x) {
            m_XMin = std::min(m_XMin, val);
            m_XMax = std::max(m_XMax, val);
        }
        for (double val : plot.y) {
            m_YMin = std::min(m_YMin, val);
            m_YMax = std::max(m_YMax, val);
        }
    }
    
    // Add 5% padding
    double xRange = m_XMax - m_XMin;
    double yRange = m_YMax - m_YMin;
    m_XMin -= xRange * 0.05;
    m_XMax += xRange * 0.05;
    m_YMin -= yRange * 0.05;
    m_YMax += yRange * 0.05;
}

void Figure::GenerateSVG(const std::string& filename) {
    ComputeAutoLimits();
    
    std::ofstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open file: " + filename);
    }
    
    // SVG header
    file << "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    file << "<svg width=\"" << m_Width << "\" height=\"" << m_Height 
         << "\" xmlns=\"http://www.w3.org/2000/svg\">\n";
    
    // White background
    file << "<rect width=\"" << m_Width << "\" height=\"" << m_Height 
         << "\" fill=\"white\"/>\n";
    
    // Plot area
    int marginLeft = 60, marginRight = 20, marginTop = 40, marginBottom = 50;
    int plotWidth = m_Width - marginLeft - marginRight;
    int plotHeight = m_Height - marginTop - marginBottom;
    
    // Grid
    if (m_ShowGrid) {
        file << "<g stroke=\"#e0e0e0\" stroke-width=\"1\">\n";
        for (int i = 0; i <= 10; i++) {
            int y = marginTop + i * plotHeight / 10;
            file << "<line x1=\"" << marginLeft << "\" y1=\"" << y 
                 << "\" x2=\"" << (marginLeft + plotWidth) << "\" y2=\"" << y << "\"/>\n";
        }
        for (int i = 0; i <= 10; i++) {
            int x = marginLeft + i * plotWidth / 10;
            file << "<line x1=\"" << x << "\" y1=\"" << marginTop 
                 << "\" x2=\"" << x << "\" y2=\"" << (marginTop + plotHeight) << "\"/>\n";
        }
        file << "</g>\n";
    }
    
    // Plot data
    for (const auto& plot : m_Plots) {
        if (plot.marker.empty() && plot.lineWidth > 0) {
            // Line plot
            file << "<polyline points=\"";
            for (size_t i = 0; i < plot.x.size(); i++) {
                double normX = (plot.x[i] - m_XMin) / (m_XMax - m_XMin);
                double normY = (plot.y[i] - m_YMin) / (m_YMax - m_YMin);
                int px = marginLeft + normX * plotWidth;
                int py = marginTop + plotHeight - normY * plotHeight;
                file << px << "," << py << " ";
            }
            file << "\" fill=\"none\" stroke=\"" << plot.color 
                 << "\" stroke-width=\"" << plot.lineWidth << "\"/>\n";
        } else {
            // Scatter plot
            for (size_t i = 0; i < plot.x.size(); i++) {
                double normX = (plot.x[i] - m_XMin) / (m_XMax - m_XMin);
                double normY = (plot.y[i] - m_YMin) / (m_YMax - m_YMin);
                int px = marginLeft + normX * plotWidth;
                int py = marginTop + plotHeight - normY * plotHeight;
                file << "<circle cx=\"" << px << "\" cy=\"" << py 
                     << "\" r=\"4\" fill=\"" << plot.color << "\"/>\n";
            }
        }
    }
    
    // Axes
    file << "<line x1=\"" << marginLeft << "\" y1=\"" << (marginTop + plotHeight)
         << "\" x2=\"" << (marginLeft + plotWidth) << "\" y2=\"" << (marginTop + plotHeight)
         << "\" stroke=\"black\" stroke-width=\"2\"/>\n";
    file << "<line x1=\"" << marginLeft << "\" y1=\"" << marginTop
         << "\" x2=\"" << marginLeft << "\" y2=\"" << (marginTop + plotHeight)
         << "\" stroke=\"black\" stroke-width=\"2\"/>\n";
    
    // Title
    if (!m_Title.empty()) {
        file << "<text x=\"" << (m_Width / 2) << "\" y=\"25\" "
             << "text-anchor=\"middle\" font-size=\"18\" font-weight=\"bold\">"
             << m_Title << "</text>\n";
    }
    
    // Labels
    if (!m_XLabel.empty()) {
        file << "<text x=\"" << (marginLeft + plotWidth / 2) << "\" y=\"" 
             << (m_Height - 10) << "\" text-anchor=\"middle\" font-size=\"14\">"
             << m_XLabel << "</text>\n";
    }
    if (!m_YLabel.empty()) {
        file << "<text x=\"15\" y=\"" << (marginTop + plotHeight / 2) 
             << "\" text-anchor=\"middle\" font-size=\"14\" "
             << "transform=\"rotate(-90 15 " << (marginTop + plotHeight / 2) << ")\">"
             << m_YLabel << "</text>\n";
    }
    
    file << "</svg>\n";
    file.close();
}

void Figure::Save(const std::string& filename) {
    GenerateSVG(filename);
}

void Figure::Show() {
    std::cout << "Figure displayed (SVG generation only - no GUI)" << std::endl;
}

void Figure::Clear() {
    m_Plots.clear();
    m_Title.clear();
    m_XLabel.clear();
    m_YLabel.clear();
    m_ShowGrid = false;
    m_ShowLegend = false;
    m_AutoLimits = true;
}

// Convenience functions
Figure* figure(int width, int height) {
    if (g_CurrentFigure) delete g_CurrentFigure;
    g_CurrentFigure = new Figure(width, height);
    return g_CurrentFigure;
}

void plot(const std::vector<double>& x, const std::vector<double>& y, const std::string& color) {
    if (!g_CurrentFigure) g_CurrentFigure = new Figure();
    g_CurrentFigure->Plot(x, y, color);
}

void scatter(const std::vector<double>& x, const std::vector<double>& y, const std::string& color) {
    if (!g_CurrentFigure) g_CurrentFigure = new Figure();
    g_CurrentFigure->Scatter(x, y, color);
}

void show() {
    if (g_CurrentFigure) g_CurrentFigure->Show();
}

void savefig(const std::string& filename) {
    if (g_CurrentFigure) g_CurrentFigure->Save(filename);
}

} // namespace NuxPlot
