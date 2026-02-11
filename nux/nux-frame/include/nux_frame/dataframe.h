#ifndef NUX_FRAME_DATAFRAME_H
#define NUX_FRAME_DATAFRAME_H

#include <string>
#include <vector>
#include <map>
#include <memory>
#include <functional>

namespace NuxFrame {

// Forward declarations
class Series;

class DataFrame {
public:
    DataFrame();
    DataFrame(const std::map<std::string, std::vector<double>>& data);
    ~DataFrame();
    
    // Data access
    Series& operator[](const std::string& column);
    const Series& operator[](const std::string& column) const;
    double At(int row, const std::string& column) const;
    void Set(int row, const std::string& column, double value);
    
    // Shape and info
    int Rows() const { return m_Rows; }
    int Cols() const { return m_Columns.size(); }
    std::vector<std::string> Columns() const;
    void Info() const;
    
    // Column operations
    void AddColumn(const std::string& name, const std::vector<double>& data);
    void RemoveColumn(const std::string& name);
    DataFrame Select(const std::vector<std::string>& columns) const;
    
    // Row operations
    DataFrame Head(int n = 5) const;
    DataFrame Tail(int n = 5) const;
    DataFrame Slice(int start, int end) const;
    
    // Filtering
    DataFrame Filter(const std::function<bool(int)>& predicate) const;
    DataFrame FilterByColumn(const std::string& column, double value) const;
    
    // Sorting
    void Sort(const std::string& column, bool ascending = true);
    
    // Aggregation
    DataFrame GroupBy(const std::string& column);
    double Sum(const std::string& column) const;
    double Mean(const std::string& column) const;
    double Min(const std::string& column) const;
    double Max(const std::string& column) const;
    double Std(const std::string& column) const;
    
    // Statistics
    DataFrame Describe() const;
    
    // I/O
    static DataFrame ReadCSV(const std::string& filename, bool hasHeader = true);
    void ToCSV(const std::string& filename) const;
    
    // Display
    void Print(int maxRows = 10) const;
    
private:
    int m_Rows;
    std::map<std::string, std::shared_ptr<Series>> m_Columns;
    std::vector<std::string> m_ColumnOrder;
};

class Series {
public:
    Series();
    Series(const std::vector<double>& data);
    Series(const std::string& name, const std::vector<double>& data);
    ~Series();
    
    // Data access
    double& operator[](int index);
    double operator[](int index) const;
    int Size() const { return m_Data.size(); }
    
    // Statistics
    double Sum() const;
    double Mean() const;
    double Min() const;
    double Max() const;
    double Std() const;
    
    // Operations
    Series operator+(const Series& other) const;
    Series operator-(const Series& other) const;
    Series operator*(const Series& other) const;
    Series operator/(const Series& other) const;
    Series operator+(double scalar) const;
    Series operator*(double scalar) const;
    
    // Comparison
    std::vector<bool> operator>(double value) const;
    std::vector<bool> operator<(double value) const;
    std::vector<bool> operator==(double value) const;
    
    // Name
    void SetName(const std::string& name) { m_Name = name; }
    const std::string& Name() const { return m_Name; }
    
    // Data
    const std::vector<double>& Data() const { return m_Data; }
    std::vector<double>& Data() { return m_Data; }
    
private:
    std::string m_Name;
    std::vector<double> m_Data;
};

} // namespace NuxFrame

#endif // NUX_FRAME_DATAFRAME_H
