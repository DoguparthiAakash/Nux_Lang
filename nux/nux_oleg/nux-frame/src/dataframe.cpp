#include "nux_frame/dataframe.h"
#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <cmath>
#include <iomanip>

namespace NuxFrame {

// Series implementation
Series::Series() : m_Name("") {
}

Series::Series(const std::vector<double>& data)
    : m_Name("")
    , m_Data(data)
{
}

Series::Series(const std::string& name, const std::vector<double>& data)
    : m_Name(name)
    , m_Data(data)
{
}

Series::~Series() {
}

double& Series::operator[](int index) {
    return m_Data[index];
}

double Series::operator[](int index) const {
    return m_Data[index];
}

double Series::Sum() const {
    double sum = 0.0;
    for (double val : m_Data) {
        sum += val;
    }
    return sum;
}

double Series::Mean() const {
    if (m_Data.empty()) return 0.0;
    return Sum() / m_Data.size();
}

double Series::Min() const {
    if (m_Data.empty()) return 0.0;
    return *std::min_element(m_Data.begin(), m_Data.end());
}

double Series::Max() const {
    if (m_Data.empty()) return 0.0;
    return *std::max_element(m_Data.begin(), m_Data.end());
}

double Series::Std() const {
    if (m_Data.empty()) return 0.0;
    double mean = Mean();
    double variance = 0.0;
    for (double val : m_Data) {
        variance += (val - mean) * (val - mean);
    }
    return std::sqrt(variance / m_Data.size());
}

Series Series::operator+(double scalar) const {
    Series result(*this);
    for (auto& val : result.m_Data) {
        val += scalar;
    }
    return result;
}

Series Series::operator*(double scalar) const {
    Series result(*this);
    for (auto& val : result.m_Data) {
        val *= scalar;
    }
    return result;
}

std::vector<bool> Series::operator>(double value) const {
    std::vector<bool> result(m_Data.size());
    for (size_t i = 0; i < m_Data.size(); i++) {
        result[i] = m_Data[i] > value;
    }
    return result;
}

// DataFrame implementation
DataFrame::DataFrame() : m_Rows(0) {
}

DataFrame::DataFrame(const std::map<std::string, std::vector<double>>& data)
    : m_Rows(0)
{
    for (const auto& pair : data) {
        AddColumn(pair.first, pair.second);
    }
}

DataFrame::~DataFrame() {
}

void DataFrame::AddColumn(const std::string& name, const std::vector<double>& data) {
    if (m_Rows == 0) {
        m_Rows = data.size();
    } else if (data.size() != static_cast<size_t>(m_Rows)) {
        throw std::invalid_argument("Column size doesn't match DataFrame rows");
    }
    
    m_Columns[name] = std::make_shared<Series>(name, data);
    m_ColumnOrder.push_back(name);
}

Series& DataFrame::operator[](const std::string& column) {
    return *m_Columns.at(column);
}

const Series& DataFrame::operator[](const std::string& column) const {
    return *m_Columns.at(column);
}

double DataFrame::At(int row, const std::string& column) const {
    return (*m_Columns.at(column))[row];
}

std::vector<std::string> DataFrame::Columns() const {
    return m_ColumnOrder;
}

DataFrame DataFrame::Head(int n) const {
    return Slice(0, std::min(n, m_Rows));
}

DataFrame DataFrame::Tail(int n) const {
    return Slice(std::max(0, m_Rows - n), m_Rows);
}

DataFrame DataFrame::Slice(int start, int end) const {
    DataFrame result;
    for (const auto& colName : m_ColumnOrder) {
        const auto& series = *m_Columns.at(colName);
        std::vector<double> slicedData(series.Data().begin() + start, 
                                       series.Data().begin() + end);
        result.AddColumn(colName, slicedData);
    }
    return result;
}

double DataFrame::Sum(const std::string& column) const {
    return (*m_Columns.at(column)).Sum();
}

double DataFrame::Mean(const std::string& column) const {
    return (*m_Columns.at(column)).Mean();
}

double DataFrame::Min(const std::string& column) const {
    return (*m_Columns.at(column)).Min();
}

double DataFrame::Max(const std::string& column) const {
    return (*m_Columns.at(column)).Max();
}

DataFrame DataFrame::ReadCSV(const std::string& filename, bool hasHeader) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open file: " + filename);
    }
    
    DataFrame df;
    std::string line;
    std::vector<std::string> headers;
    std::map<std::string, std::vector<double>> data;
    
    // Read header
    if (hasHeader && std::getline(file, line)) {
        std::stringstream ss(line);
        std::string cell;
        while (std::getline(ss, cell, ',')) {
            // Trim whitespace
            cell.erase(0, cell.find_first_not_of(" \t\r\n"));
            cell.erase(cell.find_last_not_of(" \t\r\n") + 1);
            headers.push_back(cell);
            data[cell] = std::vector<double>();
        }
    }
    
    // Read data
    while (std::getline(file, line)) {
        std::stringstream ss(line);
        std::string cell;
        size_t col = 0;
        
        while (std::getline(ss, cell, ',') && col < headers.size()) {
            try {
                double value = std::stod(cell);
                data[headers[col]].push_back(value);
            } catch (...) {
                data[headers[col]].push_back(0.0);  // Default for non-numeric
            }
            col++;
        }
    }
    
    file.close();
    
    // Create DataFrame
    for (const auto& header : headers) {
        df.AddColumn(header, data[header]);
    }
    
    return df;
}

void DataFrame::ToCSV(const std::string& filename) const {
    std::ofstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open file for writing: " + filename);
    }
    
    // Write header
    for (size_t i = 0; i < m_ColumnOrder.size(); i++) {
        file << m_ColumnOrder[i];
        if (i < m_ColumnOrder.size() - 1) file << ",";
    }
    file << "\n";
    
    // Write data
    for (int row = 0; row < m_Rows; row++) {
        for (size_t col = 0; col < m_ColumnOrder.size(); col++) {
            file << (*m_Columns.at(m_ColumnOrder[col]))[row];
            if (col < m_ColumnOrder.size() - 1) file << ",";
        }
        file << "\n";
    }
    
    file.close();
}

void DataFrame::Print(int maxRows) const {
    std::cout << "DataFrame [" << m_Rows << " rows x " << m_ColumnOrder.size() << " columns]" << std::endl;
    
    // Print header
    for (const auto& col : m_ColumnOrder) {
        std::cout << std::setw(12) << col << " ";
    }
    std::cout << std::endl;
    
    // Print separator
    for (size_t i = 0; i < m_ColumnOrder.size(); i++) {
        std::cout << "------------ ";
    }
    std::cout << std::endl;
    
    // Print rows
    int rowsToPrint = std::min(maxRows, m_Rows);
    for (int row = 0; row < rowsToPrint; row++) {
        for (const auto& colName : m_ColumnOrder) {
            std::cout << std::setw(12) << std::fixed << std::setprecision(2) 
                     << (*m_Columns.at(colName))[row] << " ";
        }
        std::cout << std::endl;
    }
    
    if (m_Rows > maxRows) {
        std::cout << "... (" << (m_Rows - maxRows) << " more rows)" << std::endl;
    }
}

void DataFrame::Info() const {
    std::cout << "DataFrame Info:" << std::endl;
    std::cout << "Rows: " << m_Rows << std::endl;
    std::cout << "Columns: " << m_ColumnOrder.size() << std::endl;
    std::cout << "\nColumn Details:" << std::endl;
    for (const auto& colName : m_ColumnOrder) {
        const auto& series = *m_Columns.at(colName);
        std::cout << "  " << colName << ": " << series.Size() << " values" << std::endl;
    }
}

} // namespace NuxFrame
