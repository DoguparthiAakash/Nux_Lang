#ifndef NUX_SAFE_MEMORY_H
#define NUX_SAFE_MEMORY_H

#include <memory>
#include <vector>
#include <stdexcept>
#include <atomic>
#include <mutex>

namespace NuxSafe {

// Safe pointer wrapper with bounds checking
template<typename T>
class SafePtr {
public:
    SafePtr() : m_Data(nullptr), m_Size(0) {}
    
    SafePtr(T* data, size_t size) 
        : m_Data(data), m_Size(size) {}
    
    T& operator[](size_t index) {
        if (index >= m_Size) {
            throw std::out_of_range("Index out of bounds");
        }
        return m_Data[index];
    }
    
    const T& operator[](size_t index) const {
        if (index >= m_Size) {
            throw std::out_of_range("Index out of bounds");
        }
        return m_Data[index];
    }
    
    T& At(size_t index) {
        return (*this)[index];
    }
    
    size_t Size() const { return m_Size; }
    T* Data() { return m_Data; }
    const T* Data() const { return m_Data; }
    
private:
    T* m_Data;
    size_t m_Size;
};

// Safe array with automatic bounds checking
template<typename T>
class SafeArray {
public:
    SafeArray(size_t size) 
        : m_Data(new T[size]), m_Size(size), m_Capacity(size) {}
    
    ~SafeArray() {
        delete[] m_Data;
    }
    
    // Disable copy (use move semantics)
    SafeArray(const SafeArray&) = delete;
    SafeArray& operator=(const SafeArray&) = delete;
    
    // Move semantics
    SafeArray(SafeArray&& other) noexcept
        : m_Data(other.m_Data), m_Size(other.m_Size), m_Capacity(other.m_Capacity) {
        other.m_Data = nullptr;
        other.m_Size = 0;
        other.m_Capacity = 0;
    }
    
    T& operator[](size_t index) {
        if (index >= m_Size) {
            throw std::out_of_range("Array index out of bounds");
        }
        return m_Data[index];
    }
    
    const T& operator[](size_t index) const {
        if (index >= m_Size) {
            throw std::out_of_range("Array index out of bounds");
        }
        return m_Data[index];
    }
    
    void Resize(size_t newSize) {
        if (newSize > m_Capacity) {
            T* newData = new T[newSize];
            for (size_t i = 0; i < m_Size; i++) {
                newData[i] = std::move(m_Data[i]);
            }
            delete[] m_Data;
            m_Data = newData;
            m_Capacity = newSize;
        }
        m_Size = newSize;
    }
    
    size_t Size() const { return m_Size; }
    size_t Capacity() const { return m_Capacity; }
    
private:
    T* m_Data;
    size_t m_Size;
    size_t m_Capacity;
};

// Reference counting for safe memory management
template<typename T>
class RefCounted {
public:
    RefCounted(T* ptr) : m_Ptr(ptr), m_RefCount(new std::atomic<int>(1)) {}
    
    RefCounted(const RefCounted& other) 
        : m_Ptr(other.m_Ptr), m_RefCount(other.m_RefCount) {
        (*m_RefCount)++;
    }
    
    ~RefCounted() {
        if (--(*m_RefCount) == 0) {
            delete m_Ptr;
            delete m_RefCount;
        }
    }
    
    T* Get() { return m_Ptr; }
    const T* Get() const { return m_Ptr; }
    
    T& operator*() { return *m_Ptr; }
    const T& operator*() const { return *m_Ptr; }
    
    T* operator->() { return m_Ptr; }
    const T* operator->() const { return m_Ptr; }
    
    int RefCount() const { return m_RefCount->load(); }
    
private:
    T* m_Ptr;
    std::atomic<int>* m_RefCount;
};

// Thread-safe container
template<typename T>
class ThreadSafeVector {
public:
    void Push(const T& value) {
        std::lock_guard<std::mutex> lock(m_Mutex);
        m_Data.push_back(value);
    }
    
    bool Pop(T& value) {
        std::lock_guard<std::mutex> lock(m_Mutex);
        if (m_Data.empty()) return false;
        value = m_Data.back();
        m_Data.pop_back();
        return true;
    }
    
    T At(size_t index) const {
        std::lock_guard<std::mutex> lock(m_Mutex);
        if (index >= m_Data.size()) {
            throw std::out_of_range("Index out of bounds");
        }
        return m_Data[index];
    }
    
    size_t Size() const {
        std::lock_guard<std::mutex> lock(m_Mutex);
        return m_Data.size();
    }
    
    void Clear() {
        std::lock_guard<std::mutex> lock(m_Mutex);
        m_Data.clear();
    }
    
private:
    mutable std::mutex m_Mutex;
    std::vector<T> m_Data;
};

} // namespace NuxSafe

#endif // NUX_SAFE_MEMORY_H
