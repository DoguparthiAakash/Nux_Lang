// Advanced Security Monitor - Real-time threat detection and prevention
// Monitors syscalls, behavior patterns, and resource usage

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Security monitor with real-time threat detection
pub struct SecurityMonitor {
    syscall_tracker: SyscallTracker,
    behavior_analyzer: BehaviorAnalyzer,
    resource_monitor: ResourceMonitor,
    threat_detector: ThreatDetector,
    audit_log: AuditLog,
    config: SecurityConfig,
}

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_syscall_monitoring: bool,
    pub enable_behavior_analysis: bool,
    pub enable_resource_monitoring: bool,
    pub auto_quarantine: bool,
    pub log_all_events: bool,
    pub threat_threshold: f64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            enable_syscall_monitoring: true,
            enable_behavior_analysis: true,
            enable_resource_monitoring: true,
            auto_quarantine: true,
            log_all_events: false,
            threat_threshold: 0.7,
        }
    }
}

/// Syscall tracker
struct SyscallTracker {
    syscalls: HashMap<String, SyscallStats>,
    recent_calls: VecDeque<SyscallEvent>,
    max_history: usize,
}

#[derive(Debug, Clone)]
struct SyscallStats {
    count: u64,
    denied_count: u64,
    last_called: Instant,
}

#[derive(Debug, Clone)]
struct SyscallEvent {
    syscall: String,
    timestamp: Instant,
    allowed: bool,
    args: Vec<String>,
}

/// Behavior analyzer using ML-based anomaly detection
struct BehaviorAnalyzer {
    patterns: Vec<BehaviorPattern>,
    anomaly_scores: VecDeque<f64>,
    baseline_established: bool,
}

#[derive(Debug, Clone)]
struct BehaviorPattern {
    pattern_type: PatternType,
    frequency: f64,
    severity: Severity,
}

#[derive(Debug, Clone, PartialEq)]
enum PatternType {
    FileSystemAccess,
    NetworkActivity,
    ProcessCreation,
    MemoryAllocation,
    CryptoOperations,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Resource monitor
struct ResourceMonitor {
    cpu_usage: Vec<f64>,
    memory_usage: Vec<usize>,
    disk_io: Vec<usize>,
    network_io: Vec<usize>,
    start_time: Instant,
}

/// Threat detector with pattern matching
struct ThreatDetector {
    known_threats: HashMap<String, ThreatSignature>,
    detected_threats: Vec<DetectedThreat>,
}

#[derive(Debug, Clone)]
struct ThreatSignature {
    name: String,
    pattern: String,
    severity: Severity,
    auto_block: bool,
}

#[derive(Debug, Clone)]
pub struct DetectedThreat {
    pub threat_type: String,
    pub severity: Severity,
    pub timestamp: Instant,
    pub details: String,
    pub action_taken: SecurityAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityAction {
    Allow,
    Warn,
    Deny,
    Quarantine,
    Terminate,
}

/// Audit log
struct AuditLog {
    events: VecDeque<AuditEvent>,
    max_events: usize,
}

#[derive(Debug, Clone)]
struct AuditEvent {
    timestamp: Instant,
    event_type: String,
    details: String,
    action: SecurityAction,
}

impl SecurityMonitor {
    pub fn new(config: SecurityConfig) -> Self {
        SecurityMonitor {
            syscall_tracker: SyscallTracker {
                syscalls: HashMap::new(),
                recent_calls: VecDeque::with_capacity(1000),
                max_history: 1000,
            },
            behavior_analyzer: BehaviorAnalyzer {
                patterns: Vec::new(),
                anomaly_scores: VecDeque::with_capacity(100),
                baseline_established: false,
            },
            resource_monitor: ResourceMonitor {
                cpu_usage: Vec::new(),
                memory_usage: Vec::new(),
                disk_io: Vec::new(),
                network_io: Vec::new(),
                start_time: Instant::now(),
            },
            threat_detector: ThreatDetector {
                known_threats: Self::initialize_threat_signatures(),
                detected_threats: Vec::new(),
            },
            audit_log: AuditLog {
                events: VecDeque::with_capacity(10000),
                max_events: 10000,
            },
            config,
        }
    }

    /// Initialize known threat signatures
    fn initialize_threat_signatures() -> HashMap<String, ThreatSignature> {
        let mut threats = HashMap::new();

        threats.insert("file_mass_deletion".to_string(), ThreatSignature {
            name: "Mass File Deletion".to_string(),
            pattern: "rm -rf|del /s".to_string(),
            severity: Severity::Critical,
            auto_block: true,
        });

        threats.insert("privilege_escalation".to_string(), ThreatSignature {
            name: "Privilege Escalation Attempt".to_string(),
            pattern: "sudo|su |chmod 777".to_string(),
            severity: Severity::Critical,
            auto_block: true,
        });

        threats.insert("network_scan".to_string(), ThreatSignature {
            name: "Network Port Scanning".to_string(),
            pattern: "nmap|masscan".to_string(),
            severity: Severity::High,
            auto_block: false,
        });

        threats.insert("crypto_mining".to_string(), ThreatSignature {
            name: "Cryptocurrency Mining".to_string(),
            pattern: "xmrig|ethminer".to_string(),
            severity: Severity::High,
            auto_block: true,
        });

        threats.insert("data_exfiltration".to_string(), ThreatSignature {
            name: "Data Exfiltration".to_string(),
            pattern: "curl|wget.*upload".to_string(),
            severity: Severity::Critical,
            auto_block: true,
        });

        threats
    }

    /// Monitor a syscall
    pub fn monitor_syscall(&mut self, syscall: &str, args: Vec<String>) -> SecurityAction {
        if !self.config.enable_syscall_monitoring {
            return SecurityAction::Allow;
        }

        // Update syscall statistics
        let stats = self.syscall_tracker.syscalls
            .entry(syscall.to_string())
            .or_insert(SyscallStats {
                count: 0,
                denied_count: 0,
                last_called: Instant::now(),
            });
        stats.count += 1;
        stats.last_called = Instant::now();

        // Check for threats
        let action = self.check_syscall_threats(syscall, &args);

        // Record event
        let event = SyscallEvent {
            syscall: syscall.to_string(),
            timestamp: Instant::now(),
            allowed: action == SecurityAction::Allow,
            args: args.clone(),
        };

        if self.syscall_tracker.recent_calls.len() >= self.syscall_tracker.max_history {
            self.syscall_tracker.recent_calls.pop_front();
        }
        self.syscall_tracker.recent_calls.push_back(event);

        // Update denied count if blocked
        if action != SecurityAction::Allow {
            stats.denied_count += 1;
        }

        // Log to audit
        self.log_event("syscall", &format!("{} {:?}", syscall, args), action.clone());

        action
    }

    /// Check syscall for threats
    fn check_syscall_threats(&mut self, syscall: &str, args: &[String]) -> SecurityAction {
        // Check for dangerous syscalls
        match syscall {
            "unlink" | "rmdir" | "remove" => {
                // Check if trying to delete system files
                for arg in args {
                    if arg.starts_with("/") || arg.starts_with("/usr") || arg.starts_with("/etc") {
                        self.detect_threat("file_system_attack", Severity::Critical, 
                            &format!("Attempted to delete system file: {}", arg));
                        return SecurityAction::Deny;
                    }
                }
            }
            "execve" | "system" => {
                // Check for dangerous commands
                for arg in args {
                    for (threat_id, signature) in &self.threat_detector.known_threats {
                        if arg.contains(&signature.pattern) {
                            self.detect_threat(threat_id, signature.severity.clone(), 
                                &format!("Dangerous command detected: {}", arg));
                            if signature.auto_block {
                                return SecurityAction::Quarantine;
                            }
                        }
                    }
                }
            }
            "socket" | "connect" | "bind" => {
                // Monitor network activity
                self.behavior_analyzer.patterns.push(BehaviorPattern {
                    pattern_type: PatternType::NetworkActivity,
                    frequency: 1.0,
                    severity: Severity::Medium,
                });
            }
            _ => {}
        }

        SecurityAction::Allow
    }

    /// Analyze behavior patterns
    pub fn analyze_behavior(&mut self) -> f64 {
        if !self.config.enable_behavior_analysis {
            return 0.0;
        }

        // Calculate anomaly score based on recent activity
        let mut anomaly_score = 0.0;

        // Check for unusual syscall patterns
        let recent_syscalls: Vec<_> = self.syscall_tracker.recent_calls
            .iter()
            .rev()
            .take(100)
            .collect();

        // Detect rapid file operations
        let file_ops = recent_syscalls.iter()
            .filter(|e| e.syscall.contains("open") || e.syscall.contains("write"))
            .count();
        if file_ops > 50 {
            anomaly_score += 0.3;
        }

        // Detect network scanning
        let network_ops = recent_syscalls.iter()
            .filter(|e| e.syscall.contains("socket") || e.syscall.contains("connect"))
            .count();
        if network_ops > 20 {
            anomaly_score += 0.4;
        }

        // Detect process spawning
        let process_ops = recent_syscalls.iter()
            .filter(|e| e.syscall.contains("fork") || e.syscall.contains("exec"))
            .count();
        if process_ops > 10 {
            anomaly_score += 0.3;
        }

        // Store anomaly score
        if self.behavior_analyzer.anomaly_scores.len() >= 100 {
            self.behavior_analyzer.anomaly_scores.pop_front();
        }
        self.behavior_analyzer.anomaly_scores.push_back(anomaly_score);

        // Trigger quarantine if threshold exceeded
        if anomaly_score >= self.config.threat_threshold {
            self.detect_threat("behavioral_anomaly", Severity::High, 
                &format!("Anomaly score: {:.2}", anomaly_score));
        }

        anomaly_score
    }

    /// Monitor resource usage
    pub fn monitor_resources(&mut self, cpu: f64, memory: usize, disk_io: usize, network_io: usize) {
        if !self.config.enable_resource_monitoring {
            return;
        }

        self.resource_monitor.cpu_usage.push(cpu);
        self.resource_monitor.memory_usage.push(memory);
        self.resource_monitor.disk_io.push(disk_io);
        self.resource_monitor.network_io.push(network_io);

        // Check for resource exhaustion attacks
        if cpu > 95.0 {
            self.detect_threat("cpu_exhaustion", Severity::High, 
                &format!("CPU usage: {:.1}%", cpu));
        }

        if memory > 1024 * 1024 * 1024 { // 1 GB
            self.detect_threat("memory_exhaustion", Severity::High, 
                &format!("Memory usage: {} bytes", memory));
        }
    }

    /// Detect a threat
    fn detect_threat(&mut self, threat_type: &str, severity: Severity, details: &str) {
        let action = if self.config.auto_quarantine && severity >= Severity::High {
            SecurityAction::Quarantine
        } else {
            SecurityAction::Warn
        };

        let threat = DetectedThreat {
            threat_type: threat_type.to_string(),
            severity,
            timestamp: Instant::now(),
            details: details.to_string(),
            action_taken: action.clone(),
        };

        self.threat_detector.detected_threats.push(threat.clone());
        self.log_event("threat_detected", &format!("{}: {}", threat_type, details), action);

        println!("[SECURITY] Threat detected: {} - {}", threat_type, details);
    }

    /// Log an event to audit log
    fn log_event(&mut self, event_type: &str, details: &str, action: SecurityAction) {
        if !self.config.log_all_events && action == SecurityAction::Allow {
            return;
        }

        let event = AuditEvent {
            timestamp: Instant::now(),
            event_type: event_type.to_string(),
            details: details.to_string(),
            action,
        };

        if self.audit_log.events.len() >= self.audit_log.max_events {
            self.audit_log.events.pop_front();
        }
        self.audit_log.events.push_back(event);
    }

    /// Get detected threats
    pub fn get_threats(&self) -> &[DetectedThreat] {
        &self.threat_detector.detected_threats
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> Vec<String> {
        self.audit_log.events.iter()
            .map(|e| format!("[{:?}] {}: {} ({:?})", 
                e.timestamp.elapsed(), e.event_type, e.details, e.action))
            .collect()
    }

    /// Get security report
    pub fn get_security_report(&self) -> SecurityReport {
        SecurityReport {
            total_syscalls: self.syscall_tracker.syscalls.values().map(|s| s.count).sum(),
            denied_syscalls: self.syscall_tracker.syscalls.values().map(|s| s.denied_count).sum(),
            threats_detected: self.threat_detector.detected_threats.len(),
            critical_threats: self.threat_detector.detected_threats.iter()
                .filter(|t| t.severity == Severity::Critical)
                .count(),
            avg_anomaly_score: if !self.behavior_analyzer.anomaly_scores.is_empty() {
                self.behavior_analyzer.anomaly_scores.iter().sum::<f64>() / 
                    self.behavior_analyzer.anomaly_scores.len() as f64
            } else {
                0.0
            },
            uptime_seconds: self.resource_monitor.start_time.elapsed().as_secs(),
        }
    }
}

/// Security report
#[derive(Debug)]
pub struct SecurityReport {
    pub total_syscalls: u64,
    pub denied_syscalls: u64,
    pub threats_detected: usize,
    pub critical_threats: usize,
    pub avg_anomaly_score: f64,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_monitor() {
        let mut monitor = SecurityMonitor::new(SecurityConfig::default());
        
        // Test safe syscall
        let action = monitor.monitor_syscall("open", vec!["test.txt".to_string()]);
        assert_eq!(action, SecurityAction::Allow);
        
        // Test dangerous syscall
        let action = monitor.monitor_syscall("unlink", vec!["/etc/passwd".to_string()]);
        assert_eq!(action, SecurityAction::Deny);
    }

    #[test]
    fn test_threat_detection() {
        let mut monitor = SecurityMonitor::new(SecurityConfig::default());
        
        // Simulate malicious command
        monitor.monitor_syscall("system", vec!["rm -rf /".to_string()]);
        
        let threats = monitor.get_threats();
        assert!(threats.len() > 0);
    }

    #[test]
    fn test_behavior_analysis() {
        let mut monitor = SecurityMonitor::new(SecurityConfig::default());
        
        // Simulate suspicious activity
        for _ in 0..100 {
            monitor.monitor_syscall("open", vec!["file.txt".to_string()]);
        }
        
        let anomaly_score = monitor.analyze_behavior();
        assert!(anomaly_score > 0.0);
    }
}
