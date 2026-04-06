// Advanced Profiler - Performance analysis and debugging tools
// Provides CPU profiling, memory profiling, and execution tracing

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Advanced profiler with multiple profiling modes
pub struct AdvancedProfiler {
    cpu_profiler: CpuProfiler,
    memory_profiler: MemoryProfiler,
    execution_tracer: ExecutionTracer,
    enabled: bool,
}

/// CPU profiler - tracks function execution time
struct CpuProfiler {
    samples: Vec<Sample>,
    call_stack: Vec<StackFrame>,
    function_stats: HashMap<String, FunctionStats>,
}

#[derive(Debug, Clone)]
struct Sample {
    timestamp: Instant,
    function: String,
    duration: Duration,
}

#[derive(Debug, Clone)]
struct StackFrame {
    function: String,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct FunctionStats {
    call_count: u64,
    total_time: Duration,
    self_time: Duration,
    avg_time: Duration,
}

/// Memory profiler - tracks allocations and memory usage
struct MemoryProfiler {
    allocations: Vec<Allocation>,
    total_allocated: usize,
    total_freed: usize,
    peak_memory: usize,
    current_memory: usize,
}

#[derive(Debug, Clone)]
struct Allocation {
    id: usize,
    size: usize,
    allocated_at: Instant,
    freed_at: Option<Instant>,
    stack_trace: Vec<String>,
}

/// Execution tracer - records execution flow
struct ExecutionTracer {
    events: Vec<TraceEvent>,
    max_events: usize,
}

#[derive(Debug, Clone)]
struct TraceEvent {
    timestamp: Instant,
    event_type: EventType,
    details: String,
}

#[derive(Debug, Clone)]
enum EventType {
    FunctionCall,
    FunctionReturn,
    Allocation,
    Deallocation,
    GarbageCollection,
    JitCompilation,
    Exception,
}

impl AdvancedProfiler {
    pub fn new() -> Self {
        AdvancedProfiler {
            cpu_profiler: CpuProfiler {
                samples: Vec::new(),
                call_stack: Vec::new(),
                function_stats: HashMap::new(),
            },
            memory_profiler: MemoryProfiler {
                allocations: Vec::new(),
                total_allocated: 0,
                total_freed: 0,
                peak_memory: 0,
                current_memory: 0,
            },
            execution_tracer: ExecutionTracer {
                events: Vec::new(),
                max_events: 100000,
            },
            enabled: false,
        }
    }

    /// Start profiling
    pub fn start(&mut self) {
        self.enabled = true;
        println!("[PROFILER] Started profiling");
    }

    /// Stop profiling
    pub fn stop(&mut self) {
        self.enabled = false;
        println!("[PROFILER] Stopped profiling");
    }

    /// Record function entry
    pub fn enter_function(&mut self, function: &str) {
        if !self.enabled {
            return;
        }

        let frame = StackFrame {
            function: function.to_string(),
            start_time: Instant::now(),
        };
        self.cpu_profiler.call_stack.push(frame);

        self.execution_tracer.record_event(EventType::FunctionCall, function);
    }

    /// Record function exit
    pub fn exit_function(&mut self, function: &str) {
        if !self.enabled {
            return;
        }

        if let Some(frame) = self.cpu_profiler.call_stack.pop() {
            let duration = frame.start_time.elapsed();

            // Record sample
            self.cpu_profiler.samples.push(Sample {
                timestamp: Instant::now(),
                function: function.to_string(),
                duration,
            });

            // Update function statistics
            let stats = self.cpu_profiler.function_stats
                .entry(function.to_string())
                .or_insert(FunctionStats {
                    call_count: 0,
                    total_time: Duration::ZERO,
                    self_time: Duration::ZERO,
                    avg_time: Duration::ZERO,
                });

            stats.call_count += 1;
            stats.total_time += duration;
            stats.avg_time = stats.total_time / stats.call_count as u32;
        }

        self.execution_tracer.record_event(EventType::FunctionReturn, function);
    }

    /// Record memory allocation
    pub fn record_allocation(&mut self, size: usize, stack_trace: Vec<String>) -> usize {
        if !self.enabled {
            return 0;
        }

        let id = self.memory_profiler.allocations.len();
        let allocation = Allocation {
            id,
            size,
            allocated_at: Instant::now(),
            freed_at: None,
            stack_trace,
        };

        self.memory_profiler.allocations.push(allocation);
        self.memory_profiler.total_allocated += size;
        self.memory_profiler.current_memory += size;

        if self.memory_profiler.current_memory > self.memory_profiler.peak_memory {
            self.memory_profiler.peak_memory = self.memory_profiler.current_memory;
        }

        self.execution_tracer.record_event(EventType::Allocation, &format!("{} bytes", size));

        id
    }

    /// Record memory deallocation
    pub fn record_deallocation(&mut self, id: usize) {
        if !self.enabled {
            return;
        }

        if let Some(allocation) = self.memory_profiler.allocations.get_mut(id) {
            allocation.freed_at = Some(Instant::now());
            self.memory_profiler.total_freed += allocation.size;
            self.memory_profiler.current_memory -= allocation.size;

            self.execution_tracer.record_event(EventType::Deallocation, &format!("{} bytes", allocation.size));
        }
    }

    /// Get CPU profile report
    pub fn get_cpu_report(&self) -> CpuReport {
        let mut functions: Vec<_> = self.cpu_profiler.function_stats.iter()
            .map(|(name, stats)| FunctionProfile {
                name: name.clone(),
                call_count: stats.call_count,
                total_time_ms: stats.total_time.as_millis() as u64,
                avg_time_ms: stats.avg_time.as_millis() as u64,
                percent_time: 0.0, // Will be calculated
            })
            .collect();

        // Calculate total time
        let total_time: u64 = functions.iter().map(|f| f.total_time_ms).sum();

        // Calculate percentages
        for func in &mut functions {
            func.percent_time = (func.total_time_ms as f64 / total_time as f64) * 100.0;
        }

        // Sort by total time descending
        functions.sort_by(|a, b| b.total_time_ms.cmp(&a.total_time_ms));

        CpuReport {
            total_samples: self.cpu_profiler.samples.len(),
            total_time_ms: total_time,
            functions,
        }
    }

    /// Get memory profile report
    pub fn get_memory_report(&self) -> MemoryReport {
        let live_allocations = self.memory_profiler.allocations.iter()
            .filter(|a| a.freed_at.is_none())
            .count();

        MemoryReport {
            total_allocated: self.memory_profiler.total_allocated,
            total_freed: self.memory_profiler.total_freed,
            current_usage: self.memory_profiler.current_memory,
            peak_usage: self.memory_profiler.peak_memory,
            live_allocations,
            total_allocations: self.memory_profiler.allocations.len(),
        }
    }

    /// Get execution trace
    pub fn get_trace(&self) -> Vec<String> {
        self.execution_tracer.events.iter()
            .map(|e| format!("[{:?}] {:?}: {}", e.timestamp.elapsed(), e.event_type, e.details))
            .collect()
    }

    /// Generate flamegraph data
    pub fn generate_flamegraph(&self) -> String {
        let mut flamegraph = String::new();
        
        for (function, stats) in &self.cpu_profiler.function_stats {
            flamegraph.push_str(&format!("{} {}\n", function, stats.total_time.as_micros()));
        }
        
        flamegraph
    }

    /// Find memory leaks
    pub fn find_memory_leaks(&self) -> Vec<MemoryLeak> {
        let mut leaks = Vec::new();
        let now = Instant::now();

        for allocation in &self.memory_profiler.allocations {
            if allocation.freed_at.is_none() {
                let age = now.duration_since(allocation.allocated_at);
                
                // Consider allocations older than 60 seconds as potential leaks
                if age.as_secs() > 60 {
                    leaks.push(MemoryLeak {
                        allocation_id: allocation.id,
                        size: allocation.size,
                        age_seconds: age.as_secs(),
                        stack_trace: allocation.stack_trace.clone(),
                    });
                }
            }
        }

        leaks.sort_by(|a, b| b.size.cmp(&a.size));
        leaks
    }

    /// Get hotspots (most time-consuming functions)
    pub fn get_hotspots(&self, limit: usize) -> Vec<Hotspot> {
        let mut hotspots: Vec<_> = self.cpu_profiler.function_stats.iter()
            .map(|(name, stats)| Hotspot {
                function: name.clone(),
                total_time_ms: stats.total_time.as_millis() as u64,
                call_count: stats.call_count,
                avg_time_ms: stats.avg_time.as_millis() as u64,
            })
            .collect();

        hotspots.sort_by(|a, b| b.total_time_ms.cmp(&a.total_time_ms));
        hotspots.truncate(limit);
        hotspots
    }
}

impl ExecutionTracer {
    fn record_event(&mut self, event_type: EventType, details: &str) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }

        self.events.push(TraceEvent {
            timestamp: Instant::now(),
            event_type,
            details: details.to_string(),
        });
    }
}

/// CPU profiling report
#[derive(Debug)]
pub struct CpuReport {
    pub total_samples: usize,
    pub total_time_ms: u64,
    pub functions: Vec<FunctionProfile>,
}

#[derive(Debug)]
pub struct FunctionProfile {
    pub name: String,
    pub call_count: u64,
    pub total_time_ms: u64,
    pub avg_time_ms: u64,
    pub percent_time: f64,
}

/// Memory profiling report
#[derive(Debug)]
pub struct MemoryReport {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub live_allocations: usize,
    pub total_allocations: usize,
}

/// Memory leak information
#[derive(Debug)]
pub struct MemoryLeak {
    pub allocation_id: usize,
    pub size: usize,
    pub age_seconds: u64,
    pub stack_trace: Vec<String>,
}

/// Performance hotspot
#[derive(Debug)]
pub struct Hotspot {
    pub function: String,
    pub total_time_ms: u64,
    pub call_count: u64,
    pub avg_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_profiling() {
        let mut profiler = AdvancedProfiler::new();
        profiler.start();

        profiler.enter_function("test_func");
        std::thread::sleep(Duration::from_millis(10));
        profiler.exit_function("test_func");

        let report = profiler.get_cpu_report();
        assert_eq!(report.functions.len(), 1);
        assert!(report.total_time_ms >= 10);
    }

    #[test]
    fn test_memory_profiling() {
        let mut profiler = AdvancedProfiler::new();
        profiler.start();

        let id = profiler.record_allocation(1024, vec!["test".to_string()]);
        let report = profiler.get_memory_report();
        assert_eq!(report.current_usage, 1024);

        profiler.record_deallocation(id);
        let report = profiler.get_memory_report();
        assert_eq!(report.current_usage, 0);
    }

    #[test]
    fn test_hotspots() {
        let mut profiler = AdvancedProfiler::new();
        profiler.start();

        for i in 0..10 {
            profiler.enter_function(&format!("func_{}", i));
            std::thread::sleep(Duration::from_micros(i * 100));
            profiler.exit_function(&format!("func_{}", i));
        }

        let hotspots = profiler.get_hotspots(3);
        assert_eq!(hotspots.len(), 3);
    }
}
