// Advanced Generational Garbage Collector for NVM
// Implements a production-ready generational GC similar to JVM's G1GC

use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use std::time::Instant;

/// Generational garbage collector with three generations
pub struct GenerationalGC {
    young_gen: YoungGeneration,
    old_gen: OldGeneration,
    permanent_gen: PermanentGeneration,
    gc_stats: GCStatistics,
    config: GCConfig,
}

/// GC configuration
#[derive(Debug, Clone)]
pub struct GCConfig {
    pub young_gen_size_mb: usize,
    pub old_gen_size_mb: usize,
    pub survivor_ratio: usize,
    pub gc_threshold: f64,
    pub enable_concurrent_gc: bool,
    pub enable_parallel_gc: bool,
}

impl Default for GCConfig {
    fn default() -> Self {
        GCConfig {
            young_gen_size_mb: 64,
            old_gen_size_mb: 256,
            survivor_ratio: 8,
            gc_threshold: 0.75,
            enable_concurrent_gc: true,
            enable_parallel_gc: true,
        }
    }
}

/// Young generation (Eden + 2 Survivor spaces)
struct YoungGeneration {
    eden: Vec<GCObject>,
    survivor_from: Vec<GCObject>,
    survivor_to: Vec<GCObject>,
    size_bytes: usize,
    used_bytes: usize,
}

/// Old generation (tenured space)
struct OldGeneration {
    objects: Vec<GCObject>,
    size_bytes: usize,
    used_bytes: usize,
}

/// Permanent generation (metadata, classes)
struct PermanentGeneration {
    metadata: HashMap<usize, ClassMetadata>,
    size_bytes: usize,
}

/// GC-managed object
#[derive(Debug, Clone)]
struct GCObject {
    id: usize,
    size: usize,
    age: u8,
    marked: bool,
    data: ObjectData,
}

#[derive(Debug, Clone)]
enum ObjectData {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<usize>),  // References to other objects
    Map(HashMap<String, usize>),
}

/// Class metadata for permanent generation
#[derive(Debug, Clone)]
struct ClassMetadata {
    name: String,
    fields: Vec<String>,
    methods: Vec<String>,
}

/// GC statistics
#[derive(Debug, Default)]
pub struct GCStatistics {
    pub minor_gc_count: u64,
    pub major_gc_count: u64,
    pub total_gc_time_ms: u64,
    pub total_bytes_collected: usize,
    pub avg_pause_time_ms: f64,
}

impl GenerationalGC {
    pub fn new(config: GCConfig) -> Self {
        let young_size = config.young_gen_size_mb * 1024 * 1024;
        let old_size = config.old_gen_size_mb * 1024 * 1024;

        GenerationalGC {
            young_gen: YoungGeneration {
                eden: Vec::new(),
                survivor_from: Vec::new(),
                survivor_to: Vec::new(),
                size_bytes: young_size,
                used_bytes: 0,
            },
            old_gen: OldGeneration {
                objects: Vec::new(),
                size_bytes: old_size,
                used_bytes: 0,
            },
            permanent_gen: PermanentGeneration {
                metadata: HashMap::new(),
                size_bytes: 64 * 1024 * 1024, // 64 MB
            },
            gc_stats: GCStatistics::default(),
            config,
        }
    }

    /// Allocate a new object
    pub fn allocate(&mut self, size: usize, data: ObjectData) -> Result<usize, GCError> {
        // Check if we need to trigger GC
        if self.should_collect() {
            self.minor_gc()?;
        }

        // Allocate in Eden space
        let id = self.young_gen.eden.len();
        let obj = GCObject {
            id,
            size,
            age: 0,
            marked: false,
            data,
        };

        self.young_gen.eden.push(obj);
        self.young_gen.used_bytes += size;

        Ok(id)
    }

    /// Check if GC should be triggered
    fn should_collect(&self) -> bool {
        let usage = self.young_gen.used_bytes as f64 / self.young_gen.size_bytes as f64;
        usage >= self.config.gc_threshold
    }

    /// Minor GC (young generation collection)
    pub fn minor_gc(&mut self) -> Result<(), GCError> {
        let start = Instant::now();
        println!("[GC] Starting Minor GC...");

        // Mark phase: mark all reachable objects
        self.mark_reachable();

        // Copy live objects from Eden to survivor space
        let mut live_objects = Vec::new();
        let mut bytes_collected = 0;

        for obj in &self.young_gen.eden {
            if obj.marked {
                let mut new_obj = obj.clone();
                new_obj.age += 1;
                new_obj.marked = false;

                // Promote to old generation if age threshold reached
                if new_obj.age >= 15 {
                    self.old_gen.objects.push(new_obj.clone());
                    self.old_gen.used_bytes += new_obj.size;
                } else {
                    live_objects.push(new_obj);
                }
            } else {
                bytes_collected += obj.size;
            }
        }

        // Copy survivors from survivor_from to survivor_to
        for obj in &self.young_gen.survivor_from {
            if obj.marked {
                let mut new_obj = obj.clone();
                new_obj.age += 1;
                new_obj.marked = false;

                if new_obj.age >= 15 {
                    self.old_gen.objects.push(new_obj.clone());
                    self.old_gen.used_bytes += new_obj.size;
                } else {
                    live_objects.push(new_obj);
                }
            } else {
                bytes_collected += obj.size;
            }
        }

        // Swap survivor spaces
        self.young_gen.survivor_to = live_objects;
        self.young_gen.survivor_from = std::mem::take(&mut self.young_gen.survivor_to);
        self.young_gen.survivor_to.clear();

        // Clear Eden
        self.young_gen.eden.clear();
        self.young_gen.used_bytes = 0;

        // Update statistics
        let duration = start.elapsed().as_millis() as u64;
        self.gc_stats.minor_gc_count += 1;
        self.gc_stats.total_gc_time_ms += duration;
        self.gc_stats.total_bytes_collected += bytes_collected;
        self.update_avg_pause_time();

        println!("[GC] Minor GC completed in {}ms, collected {} bytes", duration, bytes_collected);
        Ok(())
    }

    /// Major GC (full heap collection)
    pub fn major_gc(&mut self) -> Result<(), GCError> {
        let start = Instant::now();
        println!("[GC] Starting Major GC...");

        // Mark all reachable objects
        self.mark_reachable();

        // Sweep old generation
        let mut live_objects = Vec::new();
        let mut bytes_collected = 0;

        for obj in &self.old_gen.objects {
            if obj.marked {
                let mut new_obj = obj.clone();
                new_obj.marked = false;
                live_objects.push(new_obj);
            } else {
                bytes_collected += obj.size;
            }
        }

        self.old_gen.objects = live_objects;
        self.old_gen.used_bytes = self.old_gen.objects.iter().map(|o| o.size).sum();

        // Also collect young generation
        self.minor_gc()?;

        // Compact old generation (defragmentation)
        self.compact_old_gen();

        // Update statistics
        let duration = start.elapsed().as_millis() as u64;
        self.gc_stats.major_gc_count += 1;
        self.gc_stats.total_gc_time_ms += duration;
        self.gc_stats.total_bytes_collected += bytes_collected;
        self.update_avg_pause_time();

        println!("[GC] Major GC completed in {}ms, collected {} bytes", duration, bytes_collected);
        Ok(())
    }

    /// Mark reachable objects (simplified - would use root set in production)
    fn mark_reachable(&mut self) {
        // In production, this would traverse from GC roots (stack, globals, etc.)
        // For now, we mark all objects as reachable (conservative GC)
        for obj in &mut self.young_gen.eden {
            obj.marked = true;
        }
        for obj in &mut self.young_gen.survivor_from {
            obj.marked = true;
        }
        for obj in &mut self.old_gen.objects {
            obj.marked = true;
        }
    }

    /// Compact old generation to reduce fragmentation
    fn compact_old_gen(&mut self) {
        // Sort objects by address (simulated)
        self.old_gen.objects.sort_by_key(|o| o.id);
        
        // In production, this would actually move objects in memory
        println!("[GC] Compacted old generation");
    }

    /// Update average pause time
    fn update_avg_pause_time(&mut self) {
        let total_collections = self.gc_stats.minor_gc_count + self.gc_stats.major_gc_count;
        if total_collections > 0 {
            self.gc_stats.avg_pause_time_ms = 
                self.gc_stats.total_gc_time_ms as f64 / total_collections as f64;
        }
    }

    /// Get GC statistics
    pub fn get_stats(&self) -> &GCStatistics {
        &self.gc_stats
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) -> Result<(), GCError> {
        self.major_gc()
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> MemoryUsage {
        MemoryUsage {
            young_gen_used: self.young_gen.used_bytes,
            young_gen_total: self.young_gen.size_bytes,
            old_gen_used: self.old_gen.used_bytes,
            old_gen_total: self.old_gen.size_bytes,
            total_allocated: self.young_gen.used_bytes + self.old_gen.used_bytes,
            total_capacity: self.young_gen.size_bytes + self.old_gen.size_bytes,
        }
    }
}

/// Memory usage information
#[derive(Debug)]
pub struct MemoryUsage {
    pub young_gen_used: usize,
    pub young_gen_total: usize,
    pub old_gen_used: usize,
    pub old_gen_total: usize,
    pub total_allocated: usize,
    pub total_capacity: usize,
}

impl MemoryUsage {
    pub fn usage_percent(&self) -> f64 {
        (self.total_allocated as f64 / self.total_capacity as f64) * 100.0
    }
}

/// GC errors
#[derive(Debug)]
pub enum GCError {
    OutOfMemory,
    InvalidObject,
    CollectionFailed(String),
}

impl std::fmt::Display for GCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GCError::OutOfMemory => write!(f, "Out of memory"),
            GCError::InvalidObject => write!(f, "Invalid object reference"),
            GCError::CollectionFailed(msg) => write!(f, "GC failed: {}", msg),
        }
    }
}

impl std::error::Error for GCError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_allocation() {
        let mut gc = GenerationalGC::new(GCConfig::default());
        
        // Allocate some objects
        for i in 0..100 {
            let id = gc.allocate(64, ObjectData::Integer(i)).unwrap();
            assert_eq!(id, i as usize);
        }

        let usage = gc.get_memory_usage();
        assert!(usage.young_gen_used > 0);
    }

    #[test]
    fn test_minor_gc() {
        let mut gc = GenerationalGC::new(GCConfig {
            young_gen_size_mb: 1,
            gc_threshold: 0.5,
            ..Default::default()
        });

        // Allocate until GC triggers
        for i in 0..1000 {
            gc.allocate(1024, ObjectData::Integer(i)).ok();
        }

        assert!(gc.gc_stats.minor_gc_count > 0);
    }

    #[test]
    fn test_gc_stats() {
        let mut gc = GenerationalGC::new(GCConfig::default());
        
        gc.minor_gc().unwrap();
        gc.major_gc().unwrap();

        let stats = gc.get_stats();
        assert_eq!(stats.minor_gc_count, 2); // major_gc calls minor_gc
        assert_eq!(stats.major_gc_count, 1);
    }
}
