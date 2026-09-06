// GC Module - Garbage Collection
pub mod generational;

pub use generational::{GenerationalGC, GCConfig, GCStatistics, MemoryUsage, GCError};
