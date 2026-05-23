#ifndef NUX_SAFE_PARALLEL_H
#define NUX_SAFE_PARALLEL_H

#include <vector>
#include <thread>
#include <future>
#include <functional>
#include <queue>
#include <mutex>
#include <condition_variable>

namespace NuxSafe {

// Thread pool for parallel processing
class ThreadPool {
public:
    ThreadPool(size_t numThreads = std::thread::hardware_concurrency()) {
        for (size_t i = 0; i < numThreads; i++) {
            m_Workers.emplace_back([this] {
                while (true) {
                    std::function<void()> task;
                    {
                        std::unique_lock<std::mutex> lock(m_QueueMutex);
                        m_Condition.wait(lock, [this] {
                            return m_Stop || !m_Tasks.empty();
                        });
                        
                        if (m_Stop && m_Tasks.empty()) return;
                        
                        task = std::move(m_Tasks.front());
                        m_Tasks.pop();
                    }
                    task();
                }
            });
        }
    }
    
    ~ThreadPool() {
        {
            std::unique_lock<std::mutex> lock(m_QueueMutex);
            m_Stop = true;
        }
        m_Condition.notify_all();
        for (auto& worker : m_Workers) {
            worker.join();
        }
    }
    
    template<typename F, typename... Args>
    auto Enqueue(F&& f, Args&&... args) -> std::future<typename std::result_of<F(Args...)>::type> {
        using ReturnType = typename std::result_of<F(Args...)>::type;
        
        auto task = std::make_shared<std::packaged_task<ReturnType()>>(
            std::bind(std::forward<F>(f), std::forward<Args>(args)...)
        );
        
        std::future<ReturnType> result = task->get_future();
        {
            std::unique_lock<std::mutex> lock(m_QueueMutex);
            if (m_Stop) {
                throw std::runtime_error("Enqueue on stopped ThreadPool");
            }
            m_Tasks.emplace([task]() { (*task)(); });
        }
        m_Condition.notify_one();
        return result;
    }
    
private:
    std::vector<std::thread> m_Workers;
    std::queue<std::function<void()>> m_Tasks;
    std::mutex m_QueueMutex;
    std::condition_variable m_Condition;
    bool m_Stop = false;
};

// Parallel map operation
template<typename T, typename F>
std::vector<typename std::result_of<F(T)>::type> ParallelMap(
    const std::vector<T>& data, F func, size_t numThreads = 0) {
    
    if (numThreads == 0) {
        numThreads = std::thread::hardware_concurrency();
    }
    
    using ResultType = typename std::result_of<F(T)>::type;
    std::vector<ResultType> results(data.size());
    
    ThreadPool pool(numThreads);
    std::vector<std::future<void>> futures;
    
    for (size_t i = 0; i < data.size(); i++) {
        futures.push_back(pool.Enqueue([&data, &results, &func, i]() {
            results[i] = func(data[i]);
        }));
    }
    
    for (auto& future : futures) {
        future.get();
    }
    
    return results;
}

// Parallel reduce operation
template<typename T, typename F>
T ParallelReduce(const std::vector<T>& data, T init, F func, size_t numThreads = 0) {
    if (data.empty()) return init;
    if (numThreads == 0) {
        numThreads = std::thread::hardware_concurrency();
    }
    
    size_t chunkSize = (data.size() + numThreads - 1) / numThreads;
    std::vector<std::future<T>> futures;
    ThreadPool pool(numThreads);
    
    for (size_t i = 0; i < numThreads; i++) {
        size_t start = i * chunkSize;
        if (start >= data.size()) break;
        size_t end = std::min(start + chunkSize, data.size());
        
        futures.push_back(pool.Enqueue([&data, &func, start, end, init]() {
            T result = init;
            for (size_t j = start; j < end; j++) {
                result = func(result, data[j]);
            }
            return result;
        }));
    }
    
    T result = init;
    for (auto& future : futures) {
        result = func(result, future.get());
    }
    
    return result;
}

} // namespace NuxSafe

#endif // NUX_SAFE_PARALLEL_H
