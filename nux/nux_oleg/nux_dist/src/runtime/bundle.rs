// Self-Contained Runtime Bundle - Single binary with embedded resources
// Enables zero-dependency standalone execution

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use flate2::read::GzDecoder;
use tar::Archive;

/// Self-contained runtime bundle
pub struct RuntimeBundle {
    bundle_dir: PathBuf,
    extracted: bool,
    config: BundleConfig,
}

/// Bundle configuration
#[derive(Debug, Clone)]
pub struct BundleConfig {
    pub bundle_name: String,
    pub version: String,
    pub minimal_mode: bool,
    pub include_jit: bool,
    pub include_polyglot: bool,
}

impl Default for BundleConfig {
    fn default() -> Self {
        BundleConfig {
            bundle_name: "nux".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            minimal_mode: false,
            include_jit: true,
            include_polyglot: true,
        }
    }
}

/// Embedded resources (compiled into binary)
pub struct EmbeddedResources {
    pub stdlib_bytecode: &'static [u8],
    pub python_runtime: &'static [u8],
    pub nodejs_runtime: &'static [u8],
    pub rust_runtime: &'static [u8],
}

impl RuntimeBundle {
    /// Create new runtime bundle
    pub fn new(config: BundleConfig) -> Self {
        let bundle_dir = Self::get_bundle_directory();
        
        RuntimeBundle {
            bundle_dir,
            extracted: false,
            config,
        }
    }

    /// Get bundle directory (platform-specific)
    fn get_bundle_directory() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("nux")
        }
        
        #[cfg(target_os = "macos")]
        {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("nux")
        }
        
        #[cfg(target_os = "windows")]
        {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("C:\\Temp"))
                .join("nux")
        }
    }

    /// Initialize runtime (extract embedded resources on first run)
    pub fn initialize(&mut self) -> Result<(), BundleError> {
        println!("[RUNTIME] Initializing Nux runtime bundle v{}", self.config.version);

        // Check if already extracted
        if self.is_extracted() {
            println!("[RUNTIME] Using existing runtime at {}", self.bundle_dir.display());
            self.extracted = true;
            return Ok(());
        }

        // Create bundle directory
        fs::create_dir_all(&self.bundle_dir)
            .map_err(|e| BundleError::IoError(e.to_string()))?;

        // Extract embedded resources
        self.extract_stdlib()?;
        
        if self.config.include_polyglot {
            self.extract_runtimes()?;
        }

        // Mark as extracted
        self.mark_extracted()?;
        self.extracted = true;

        println!("[RUNTIME] Runtime initialized successfully");
        Ok(())
    }

    /// Check if runtime is already extracted
    fn is_extracted(&self) -> bool {
        self.bundle_dir.join(".nux_runtime").exists()
    }

    /// Mark runtime as extracted
    fn mark_extracted(&self) -> Result<(), BundleError> {
        let marker_file = self.bundle_dir.join(".nux_runtime");
        let mut file = fs::File::create(marker_file)
            .map_err(|e| BundleError::IoError(e.to_string()))?;
        
        writeln!(file, "version={}", self.config.version)
            .map_err(|e| BundleError::IoError(e.to_string()))?;
        
        Ok(())
    }

    /// Extract standard library bytecode
    fn extract_stdlib(&self) -> Result<(), BundleError> {
        println!("[RUNTIME] Extracting standard library...");
        
        let stdlib_dir = self.bundle_dir.join("lib").join("std");
        fs::create_dir_all(&stdlib_dir)
            .map_err(|e| BundleError::IoError(e.to_string()))?;

        // In production, this would extract embedded bytecode
        // For now, we'll create placeholder files
        let stdlib_modules = vec![
            "io.nuxc", "fs.nuxc", "net.nuxc", "math.nuxc",
            "collections.nuxc", "async.nuxc", "json.nuxc",
            "http.nuxc", "crypto.nuxc", "testing.nuxc",
        ];

        for module in stdlib_modules {
            let module_path = stdlib_dir.join(module);
            fs::write(&module_path, b"NUX_BYTECODE_PLACEHOLDER")
                .map_err(|e| BundleError::IoError(e.to_string()))?;
        }

        println!("[RUNTIME] Standard library extracted");
        Ok(())
    }

    /// Extract language runtimes
    fn extract_runtimes(&self) -> Result<(), BundleError> {
        println!("[RUNTIME] Extracting language runtimes...");
        
        let runtimes_dir = self.bundle_dir.join("runtimes");
        fs::create_dir_all(&runtimes_dir)
            .map_err(|e| BundleError::IoError(e.to_string()))?;

        // Extract Python runtime
        self.extract_runtime("python", &runtimes_dir.join("python"))?;
        
        // Extract Node.js runtime
        self.extract_runtime("nodejs", &runtimes_dir.join("nodejs"))?;
        
        // Extract Rust runtime
        self.extract_runtime("rust", &runtimes_dir.join("rust"))?;

        println!("[RUNTIME] Language runtimes extracted");
        Ok(())
    }

    /// Extract a single runtime
    fn extract_runtime(&self, name: &str, target_dir: &Path) -> Result<(), BundleError> {
        fs::create_dir_all(target_dir)
            .map_err(|e| BundleError::IoError(e.to_string()))?;

        // In production, this would decompress embedded runtime
        // For now, create placeholder
        let placeholder = target_dir.join(format!("{}_runtime", name));
        fs::write(&placeholder, format!("EMBEDDED_{}_RUNTIME", name.to_uppercase()))
            .map_err(|e| BundleError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Get path to standard library
    pub fn get_stdlib_path(&self) -> PathBuf {
        self.bundle_dir.join("lib").join("std")
    }

    /// Get path to runtime
    pub fn get_runtime_path(&self, runtime: &str) -> PathBuf {
        self.bundle_dir.join("runtimes").join(runtime)
    }

    /// Get runtime information
    pub fn get_info(&self) -> RuntimeInfo {
        RuntimeInfo {
            version: self.config.version.clone(),
            bundle_dir: self.bundle_dir.clone(),
            extracted: self.extracted,
            minimal_mode: self.config.minimal_mode,
            jit_enabled: self.config.include_jit,
            polyglot_enabled: self.config.include_polyglot,
            stdlib_path: self.get_stdlib_path(),
        }
    }

    /// Clean runtime (remove extracted files)
    pub fn clean(&self) -> Result<(), BundleError> {
        println!("[RUNTIME] Cleaning runtime bundle...");
        
        if self.bundle_dir.exists() {
            fs::remove_dir_all(&self.bundle_dir)
                .map_err(|e| BundleError::IoError(e.to_string()))?;
        }

        println!("[RUNTIME] Runtime cleaned");
        Ok(())
    }

    /// Update runtime to new version
    pub fn update(&mut self, new_version: &str) -> Result<(), BundleError> {
        println!("[RUNTIME] Updating runtime to version {}...", new_version);
        
        // Clean old version
        self.clean()?;
        
        // Update config
        self.config.version = new_version.to_string();
        
        // Re-initialize
        self.initialize()?;

        println!("[RUNTIME] Runtime updated successfully");
        Ok(())
    }
}

/// Runtime information
#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub version: String,
    pub bundle_dir: PathBuf,
    pub extracted: bool,
    pub minimal_mode: bool,
    pub jit_enabled: bool,
    pub polyglot_enabled: bool,
    pub stdlib_path: PathBuf,
}

impl RuntimeInfo {
    pub fn print(&self) {
        println!("Nux Runtime Information:");
        println!("  Version: {}", self.version);
        println!("  Bundle Directory: {}", self.bundle_dir.display());
        println!("  Extracted: {}", self.extracted);
        println!("  Mode: {}", if self.minimal_mode { "Minimal" } else { "Full" });
        println!("  JIT Enabled: {}", self.jit_enabled);
        println!("  Polyglot Enabled: {}", self.polyglot_enabled);
        println!("  Standard Library: {}", self.stdlib_path.display());
    }
}

/// Bundle errors
#[derive(Debug)]
pub enum BundleError {
    IoError(String),
    ExtractionFailed(String),
    InvalidBundle(String),
}

impl std::fmt::Display for BundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleError::IoError(msg) => write!(f, "IO error: {}", msg),
            BundleError::ExtractionFailed(msg) => write!(f, "Extraction failed: {}", msg),
            BundleError::InvalidBundle(msg) => write!(f, "Invalid bundle: {}", msg),
        }
    }
}

impl std::error::Error for BundleError {}

// Mock dirs module for compilation
mod dirs {
    use std::path::PathBuf;
    
    pub fn data_local_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".local").join("share"))
    }
}

// Mock flate2 and tar for compilation
mod flate2 {
    pub mod read {
        pub struct GzDecoder<R>(std::marker::PhantomData<R>);
    }
}

mod tar {
    pub struct Archive<R>(std::marker::PhantomData<R>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_bundle_creation() {
        let config = BundleConfig::default();
        let bundle = RuntimeBundle::new(config);
        
        assert!(!bundle.extracted);
    }

    #[test]
    fn test_runtime_initialization() {
        let config = BundleConfig::default();
        let mut bundle = RuntimeBundle::new(config);
        
        let result = bundle.initialize();
        assert!(result.is_ok());
        assert!(bundle.extracted);
    }

    #[test]
    fn test_runtime_info() {
        let config = BundleConfig::default();
        let mut bundle = RuntimeBundle::new(config);
        bundle.initialize().unwrap();
        
        let info = bundle.get_info();
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.extracted);
    }
}
