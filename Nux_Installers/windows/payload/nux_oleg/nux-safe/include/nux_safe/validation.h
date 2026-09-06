#ifndef NUX_SAFE_VALIDATION_H
#define NUX_SAFE_VALIDATION_H

#include <string>
#include <vector>
#include <stdexcept>
#include <cmath>
#include <limits>

namespace NuxSafe {

class ValidationError : public std::runtime_error {
public:
    ValidationError(const std::string& msg) : std::runtime_error(msg) {}
};

class Validator {
public:
    // Numeric validation
    static void CheckPositive(double value, const std::string& name = "value") {
        if (value <= 0) {
            throw ValidationError(name + " must be positive");
        }
    }
    
    static void CheckNonNegative(double value, const std::string& name = "value") {
        if (value < 0) {
            throw ValidationError(name + " must be non-negative");
        }
    }
    
    static void CheckRange(double value, double min, double max, const std::string& name = "value") {
        if (value < min || value > max) {
            throw ValidationError(name + " must be in range [" + 
                                std::to_string(min) + ", " + std::to_string(max) + "]");
        }
    }
    
    static void CheckFinite(double value, const std::string& name = "value") {
        if (!std::isfinite(value)) {
            throw ValidationError(name + " must be finite (not NaN or Inf)");
        }
    }
    
    // Array validation
    static void CheckNotEmpty(const std::vector<double>& data, const std::string& name = "array") {
        if (data.empty()) {
            throw ValidationError(name + " cannot be empty");
        }
    }
    
    static void CheckSameSize(const std::vector<double>& a, const std::vector<double>& b,
                             const std::string& nameA = "array1", const std::string& nameB = "array2") {
        if (a.size() != b.size()) {
            throw ValidationError(nameA + " and " + nameB + " must have the same size");
        }
    }
    
    static void CheckMinSize(const std::vector<double>& data, size_t minSize, 
                            const std::string& name = "array") {
        if (data.size() < minSize) {
            throw ValidationError(name + " must have at least " + std::to_string(minSize) + " elements");
        }
    }
    
    // Matrix validation
    template<typename T>
    static void CheckRectangular(const std::vector<std::vector<T>>& matrix, 
                                const std::string& name = "matrix") {
        if (matrix.empty()) return;
        size_t cols = matrix[0].size();
        for (size_t i = 1; i < matrix.size(); i++) {
            if (matrix[i].size() != cols) {
                throw ValidationError(name + " must be rectangular (all rows same length)");
            }
        }
    }
    
    static void CheckSquare(const std::vector<std::vector<double>>& matrix,
                           const std::string& name = "matrix") {
        CheckRectangular(matrix, name);
        if (!matrix.empty() && matrix.size() != matrix[0].size()) {
            throw ValidationError(name + " must be square");
        }
    }
    
    // String validation
    static void CheckNotEmpty(const std::string& str, const std::string& name = "string") {
        if (str.empty()) {
            throw ValidationError(name + " cannot be empty");
        }
    }
    
    // Null pointer validation
    template<typename T>
    static void CheckNotNull(T* ptr, const std::string& name = "pointer") {
        if (ptr == nullptr) {
            throw ValidationError(name + " cannot be null");
        }
    }
};

// Result type for safe error handling (like Rust's Result)
template<typename T, typename E = std::string>
class Result {
public:
    static Result Ok(const T& value) {
        Result r;
        r.m_IsOk = true;
        r.m_Value = value;
        return r;
    }
    
    static Result Err(const E& error) {
        Result r;
        r.m_IsOk = false;
        r.m_Error = error;
        return r;
    }
    
    bool IsOk() const { return m_IsOk; }
    bool IsErr() const { return !m_IsOk; }
    
    T Unwrap() const {
        if (!m_IsOk) {
            throw std::runtime_error("Called Unwrap on error Result");
        }
        return m_Value;
    }
    
    T UnwrapOr(const T& defaultValue) const {
        return m_IsOk ? m_Value : defaultValue;
    }
    
    E Error() const {
        if (m_IsOk) {
            throw std::runtime_error("Called Error on Ok Result");
        }
        return m_Error;
    }
    
private:
    Result() : m_IsOk(false) {}
    bool m_IsOk;
    T m_Value;
    E m_Error;
};

// Option type for safe null handling (like Rust's Option)
template<typename T>
class Option {
public:
    static Option Some(const T& value) {
        Option opt;
        opt.m_HasValue = true;
        opt.m_Value = value;
        return opt;
    }
    
    static Option None() {
        return Option();
    }
    
    bool IsSome() const { return m_HasValue; }
    bool IsNone() const { return !m_HasValue; }
    
    T Unwrap() const {
        if (!m_HasValue) {
            throw std::runtime_error("Called Unwrap on None Option");
        }
        return m_Value;
    }
    
    T UnwrapOr(const T& defaultValue) const {
        return m_HasValue ? m_Value : defaultValue;
    }
    
private:
    Option() : m_HasValue(false) {}
    bool m_HasValue;
    T m_Value;
};

} // namespace NuxSafe

#endif // NUX_SAFE_VALIDATION_H
