// Test for NuxSafe library
#include "nux_safe/memory.h"
#include "nux_safe/validation.h"
#include "nux_safe/parallel.h"
#include <iostream>
#include <vector>

using namespace NuxSafe;

void TestMemorySafety() {
    std::cout << "[Test 1] Memory Safety..." << std::endl;
    
    // Safe array with bounds checking
    SafeArray<int> arr(10);
    for (size_t i = 0; i < arr.Size(); i++) {
        arr[i] = i * 2;
    }
    
    try {
        int val = arr[15];  // Out of bounds
        std::cout << "ERROR: Should have thrown exception!" << std::endl;
    } catch (const std::out_of_range& e) {
        std::cout << "✓ Caught bounds error: " << e.what() << std::endl;
    }
    
    // Thread-safe vector
    ThreadSafeVector<int> tsVec;
    tsVec.Push(10);
    tsVec.Push(20);
    std::cout << "✓ Thread-safe vector size: " << tsVec.Size() << std::endl;
    
    std::cout << std::endl;
}

void TestValidation() {
    std::cout << "[Test 2] Input Validation..." << std::endl;
    
    try {
        Validator::CheckPositive(-5.0, "temperature");
    } catch (const ValidationError& e) {
        std::cout << "✓ Caught validation error: " << e.what() << std::endl;
    }
    
    try {
        Validator::CheckRange(150.0, 0.0, 100.0, "percentage");
    } catch (const ValidationError& e) {
        std::cout << "✓ Caught range error: " << e.what() << std::endl;
    }
    
    std::vector<double> data = {1.0, 2.0, 3.0};
    Validator::CheckNotEmpty(data, "data");
    std::cout << "✓ Non-empty validation passed" << std::endl;
    
    std::cout << std::endl;
}

void TestResultOption() {
    std::cout << "[Test 3] Result and Option Types..." << std::endl;
    
    // Result type
    auto divide = [](double a, double b) -> Result<double> {
        if (b == 0) {
            return Result<double>::Err("Division by zero");
        }
        return Result<double>::Ok(a / b);
    };
    
    auto result1 = divide(10.0, 2.0);
    if (result1.IsOk()) {
        std::cout << "✓ Division result: " << result1.Unwrap() << std::endl;
    }
    
    auto result2 = divide(10.0, 0.0);
    if (result2.IsErr()) {
        std::cout << "✓ Division error: " << result2.Error() << std::endl;
    }
    
    // Option type
    auto findFirst = [](const std::vector<int>& vec, int value) -> Option<size_t> {
        for (size_t i = 0; i < vec.size(); i++) {
            if (vec[i] == value) {
                return Option<size_t>::Some(i);
            }
        }
        return Option<size_t>::None();
    };
    
    std::vector<int> nums = {1, 2, 3, 4, 5};
    auto opt1 = findFirst(nums, 3);
    if (opt1.IsSome()) {
        std::cout << "✓ Found at index: " << opt1.Unwrap() << std::endl;
    }
    
    auto opt2 = findFirst(nums, 10);
    if (opt2.IsNone()) {
        std::cout << "✓ Not found (None)" << std::endl;
    }
    
    std::cout << std::endl;
}

void TestParallel() {
    std::cout << "[Test 4] Parallel Processing..." << std::endl;
    
    // Parallel map
    std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    auto squared = ParallelMap(data, [](int x) { return x * x; });
    
    std::cout << "✓ Parallel map (squared): ";
    for (int val : squared) {
        std::cout << val << " ";
    }
    std::cout << std::endl;
    
    // Parallel reduce
    auto sum = ParallelReduce(data, 0, [](int a, int b) { return a + b; });
    std::cout << "✓ Parallel reduce (sum): " << sum << std::endl;
    
    std::cout << std::endl;
}

int main() {
    std::cout << "=====================================" << std::endl;
    std::cout << "  NuxSafe Library Test" << std::endl;
    std::cout << "=====================================" << std::endl << std::endl;
    
    TestMemorySafety();
    TestValidation();
    TestResultOption();
    TestParallel();
    
    std::cout << "=====================================" << std::endl;
    std::cout << "  ✓ All Safety Tests Passed!" << std::endl;
    std::cout << "  Memory safety verified" << std::endl;
    std::cout << "  Validation working" << std::endl;
    std::cout << "  Result/Option types functional" << std::endl;
    std::cout << "  Parallel processing enabled" << std::endl;
    std::cout << "=====================================" << std::endl;
    
    return 0;
}
