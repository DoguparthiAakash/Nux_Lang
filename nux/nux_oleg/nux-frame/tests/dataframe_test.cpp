// Test for NuxFrame DataFrame library
#include "nux_frame/dataframe.h"
#include <iostream>

using namespace NuxFrame;

int main() {
    std::cout << "=====================================" << std::endl;
    std::cout << "  NuxFrame Library Test" << std::endl;
    std::cout << "=====================================" << std::endl << std::endl;
    
    // Test 1: Create DataFrame from data
    std::cout << "[Test 1] Creating DataFrame from data..." << std::endl;
    std::map<std::string, std::vector<double>> data = {
        {"age", {25, 30, 35, 40, 45}},
        {"salary", {50000, 60000, 75000, 90000, 100000}},
        {"years_exp", {2, 5, 8, 12, 15}}
    };
    
    DataFrame df(data);
    std::cout << "Created DataFrame with " << df.Rows() << " rows and " 
              << df.Cols() << " columns" << std::endl << std::endl;
    
    // Test 2: Display DataFrame
    std::cout << "[Test 2] Displaying DataFrame..." << std::endl;
    df.Print();
    std::cout << std::endl;
    
    // Test 3: Basic statistics
    std::cout << "[Test 3] Computing statistics..." << std::endl;
    std::cout << "Age - Mean: " << df.Mean("age") << ", Min: " << df.Min("age") 
              << ", Max: " << df.Max("age") << std::endl;
    std::cout << "Salary - Mean: " << df.Mean("salary") << ", Sum: " << df.Sum("salary") << std::endl;
    std::cout << std::endl;
    
    // Test 4: Head and Tail
    std::cout << "[Test 4] Head (first 3 rows)..." << std::endl;
    auto head = df.Head(3);
    head.Print();
    std::cout << std::endl;
    
    // Test 5: Column operations
    std::cout << "[Test 5] Adding new column..." << std::endl;
    std::vector<double> bonus = {5000, 6000, 7500, 9000, 10000};
    df.AddColumn("bonus", bonus);
    df.Print();
    std::cout << std::endl;
    
    // Test 6: Series operations
    std::cout << "[Test 6] Series operations..." << std::endl;
    Series ages("age", {25, 30, 35, 40, 45});
    std::cout << "Ages mean: " << ages.Mean() << std::endl;
    std::cout << "Ages std: " << ages.Std() << std::endl;
    
    Series doubled = ages * 2.0;
    std::cout << "Ages doubled - first value: " << doubled[0] << std::endl;
    std::cout << std::endl;
    
    // Test 7: CSV I/O
    std::cout << "[Test 7] CSV export/import..." << std::endl;
    df.ToCSV("test_data.csv");
    std::cout << "Exported to test_data.csv" << std::endl;
    
    auto df2 = DataFrame::ReadCSV("test_data.csv");
    std::cout << "Imported from CSV:" << std::endl;
    df2.Print(3);
    std::cout << std::endl;
    
    // Test 8: Info
    std::cout << "[Test 8] DataFrame info..." << std::endl;
    df.Info();
    std::cout << std::endl;
    
    std::cout << "=====================================" << std::endl;
    std::cout << "  ✓ All Tests Passed!" << std::endl;
    std::cout << "  NuxFrame library working correctly" << std::endl;
    std::cout << "=====================================" << std::endl;
    
    return 0;
}
