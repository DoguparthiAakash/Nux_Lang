// Security Module - Security monitoring and threat detection
pub mod monitor;

pub use monitor::{SecurityMonitor, SecurityConfig, DetectedThreat, SecurityAction, SecurityReport};
