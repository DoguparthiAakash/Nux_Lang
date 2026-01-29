// Virtual Environment Manager
// Manages isolated virtual environments for Nux projects

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Virtual environment for isolated execution
#[derive(Debug, Clone)]
pub struct VirtualEnvironment {
    pub id: String,
    pub name: String,
    pub root_path: PathBuf,
    pub bin_path: PathBuf,
    pub lib_path: PathBuf,
    pub python_path: Option<PathBuf>,
    pub nodejs_path: Option<PathBuf>,
    pub rust_path: Option<PathBuf>,
    pub c_compiler_path: Option<PathBuf>,
    pub resource_limits: ResourceLimits,
    pub security_level: SecurityLevel,
    pub created_at: u64,
    pub activated: bool,
}

/// Resource limits for the virtual environment
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_mb: usize,
    pub max_cpu_percent: u32,
    pub max_disk_mb: usize,
    pub max_processes: u32,
    pub max_open_files: u32,
    pub max_execution_time_ms: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        ResourceLimits {
            max_memory_mb: 2048,      // 2 GB
            max_cpu_percent: 80,       // 80% CPU
            max_disk_mb: 5120,         // 5 GB
            max_processes: 100,
            max_open_files: 1024,
            max_execution_time_ms: 300000, // 5 minutes
        }
    }
}

/// Security level for the virtual environment
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Trusted,      // Full access, no restrictions
    Restricted,   // Limited access, monitored
    Isolated,     // Completely isolated, separate process
}

/// Virtual environment manager
pub struct VenvManager {
    venvs: HashMap<String, VirtualEnvironment>,
    venv_root: PathBuf,
}

impl VenvManager {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let venv_root = PathBuf::from(home_dir).join(".nux").join("venvs");
        
        // Create venv root directory if it doesn't exist
        let _ = fs::create_dir_all(&venv_root);
        
        VenvManager {
            venvs: HashMap::new(),
            venv_root,
        }
    }

    /// Create a new virtual environment
    pub fn create(&mut self, name: &str, security_level: SecurityLevel) -> Result<String, VenvError> {
        // Generate unique ID
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let id = format!("{}_{}", name, timestamp);
        
        // Create venv directory structure
        let venv_path = self.venv_root.join(&id);
        let bin_path = venv_path.join("bin");
        let lib_path = venv_path.join("lib");
        let include_path = venv_path.join("include");
        let share_path = venv_path.join("share");
        
        fs::create_dir_all(&venv_path)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to create venv directory: {}", e)))?;
        fs::create_dir_all(&bin_path)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to create bin directory: {}", e)))?;
        fs::create_dir_all(&lib_path)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to create lib directory: {}", e)))?;
        fs::create_dir_all(&include_path)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to create include directory: {}", e)))?;
        fs::create_dir_all(&share_path)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to create share directory: {}", e)))?;
        
        // Create venv config file
        let config = format!(
            "VENV_ID={}\nVENV_NAME={}\nCREATED_AT={}\nSECURITY_LEVEL={:?}\n",
            id, name, timestamp, security_level
        );
        fs::write(venv_path.join(".nuxvenv"), config)
            .map_err(|e| VenvError::CreationFailed(format!("Failed to write config: {}", e)))?;
        
        let venv = VirtualEnvironment {
            id: id.clone(),
            name: name.to_string(),
            root_path: venv_path,
            bin_path,
            lib_path,
            python_path: None,
            nodejs_path: None,
            rust_path: None,
            c_compiler_path: None,
            resource_limits: ResourceLimits::default(),
            security_level,
            created_at: timestamp,
            activated: false,
        };
        
        self.venvs.insert(id.clone(), venv);
        
        println!("Created virtual environment: {} ({})", name, id);
        Ok(id)
    }

    /// Activate a virtual environment
    pub fn activate(&mut self, id: &str) -> Result<(), VenvError> {
        let venv = self.venvs.get_mut(id)
            .ok_or_else(|| VenvError::NotFound(id.to_string()))?;
        
        if venv.activated {
            return Ok(());
        }
        
        // Set environment variables
        std::env::set_var("NUX_VENV", &venv.id);
        std::env::set_var("NUX_VENV_ROOT", &venv.root_path);
        std::env::set_var("NUX_VENV_BIN", &venv.bin_path);
        
        // Prepend venv bin to PATH
        let current_path = std::env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", venv.bin_path.display(), current_path);
        std::env::set_var("PATH", new_path);
        
        venv.activated = true;
        println!("Activated virtual environment: {}", venv.name);
        Ok(())
    }

    /// Deactivate a virtual environment
    pub fn deactivate(&mut self, id: &str) -> Result<(), VenvError> {
        let venv = self.venvs.get_mut(id)
            .ok_or_else(|| VenvError::NotFound(id.to_string()))?;
        
        if !venv.activated {
            return Ok(());
        }
        
        // Remove environment variables
        std::env::remove_var("NUX_VENV");
        std::env::remove_var("NUX_VENV_ROOT");
        std::env::remove_var("NUX_VENV_BIN");
        
        // Restore original PATH (remove venv bin)
        let current_path = std::env::var("PATH").unwrap_or_default();
        let venv_bin_str = format!("{}:", venv.bin_path.display());
        let new_path = current_path.replace(&venv_bin_str, "");
        std::env::set_var("PATH", new_path);
        
        venv.activated = false;
        println!("Deactivated virtual environment: {}", venv.name);
        Ok(())
    }

    /// Install a language runtime in the virtual environment
    pub fn install_runtime(&mut self, id: &str, language: Language) -> Result<(), VenvError> {
        let venv = self.venvs.get_mut(id)
            .ok_or_else(|| VenvError::NotFound(id.to_string()))?;
        
        match language {
            Language::Python => self.install_python(venv)?,
            Language::JavaScript => self.install_nodejs(venv)?,
            Language::Rust => self.install_rust(venv)?,
            Language::C => self.install_c_compiler(venv)?,
        }
        
        Ok(())
    }

    fn install_python(&self, venv: &mut VirtualEnvironment) -> Result<(), VenvError> {
        println!("Installing Python runtime in venv: {}", venv.name);
        
        // Find system Python
        let python_path = which::which("python3")
            .map_err(|_| VenvError::RuntimeNotFound("Python not found on system".to_string()))?;
        
        // Copy Python to venv
        let venv_python = venv.bin_path.join("python3");
        fs::copy(&python_path, &venv_python)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to copy Python: {}", e)))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&venv_python).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&venv_python, perms).unwrap();
        }
        
        // Create Python lib directory
        let python_lib = venv.lib_path.join("python3.11");
        fs::create_dir_all(&python_lib)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to create Python lib dir: {}", e)))?;
        
        venv.python_path = Some(venv_python);
        println!("Python installed successfully");
        Ok(())
    }

    fn install_nodejs(&self, venv: &mut VirtualEnvironment) -> Result<(), VenvError> {
        println!("Installing Node.js runtime in venv: {}", venv.name);
        
        // Find system Node.js
        let node_path = which::which("node")
            .map_err(|_| VenvError::RuntimeNotFound("Node.js not found on system".to_string()))?;
        
        // Copy Node.js to venv
        let venv_node = venv.bin_path.join("node");
        fs::copy(&node_path, &venv_node)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to copy Node.js: {}", e)))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&venv_node).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&venv_node, perms).unwrap();
        }
        
        // Create node_modules directory
        let node_modules = venv.lib_path.join("node_modules");
        fs::create_dir_all(&node_modules)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to create node_modules: {}", e)))?;
        
        venv.nodejs_path = Some(venv_node);
        println!("Node.js installed successfully");
        Ok(())
    }

    fn install_rust(&self, venv: &mut VirtualEnvironment) -> Result<(), VenvError> {
        println!("Installing Rust toolchain in venv: {}", venv.name);
        
        // Find system Rust
        let rustc_path = which::which("rustc")
            .map_err(|_| VenvError::RuntimeNotFound("Rust not found on system".to_string()))?;
        
        // Copy rustc to venv
        let venv_rustc = venv.bin_path.join("rustc");
        fs::copy(&rustc_path, &venv_rustc)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to copy rustc: {}", e)))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&venv_rustc).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&venv_rustc, perms).unwrap();
        }
        
        // Create Rust lib directory
        let rust_lib = venv.lib_path.join("rust");
        fs::create_dir_all(&rust_lib)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to create Rust lib dir: {}", e)))?;
        
        venv.rust_path = Some(venv_rustc);
        println!("Rust installed successfully");
        Ok(())
    }

    fn install_c_compiler(&self, venv: &mut VirtualEnvironment) -> Result<(), VenvError> {
        println!("Installing C compiler in venv: {}", venv.name);
        
        // Find system GCC
        let gcc_path = which::which("gcc")
            .map_err(|_| VenvError::RuntimeNotFound("GCC not found on system".to_string()))?;
        
        // Copy GCC to venv
        let venv_gcc = venv.bin_path.join("gcc");
        fs::copy(&gcc_path, &venv_gcc)
            .map_err(|e| VenvError::InstallFailed(format!("Failed to copy GCC: {}", e)))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&venv_gcc).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&venv_gcc, perms).unwrap();
        }
        
        venv.c_compiler_path = Some(venv_gcc);
        println!("C compiler installed successfully");
        Ok(())
    }

    /// Install a package in the virtual environment
    pub fn install_package(&mut self, id: &str, language: Language, package: &str) -> Result<(), VenvError> {
        let venv = self.venvs.get(id)
            .ok_or_else(|| VenvError::NotFound(id.to_string()))?;
        
        match language {
            Language::Python => self.install_python_package(venv, package)?,
            Language::JavaScript => self.install_npm_package(venv, package)?,
            Language::Rust => self.install_cargo_crate(venv, package)?,
            Language::C => return Err(VenvError::UnsupportedOperation("C libraries must be installed manually".to_string())),
        }
        
        Ok(())
    }

    fn install_python_package(&self, venv: &VirtualEnvironment, package: &str) -> Result<(), VenvError> {
        println!("Installing Python package: {}", package);
        
        let python_bin = venv.python_path.as_ref()
            .ok_or_else(|| VenvError::RuntimeNotFound("Python not installed in venv".to_string()))?;
        
        // Run pip install
        let output = Command::new(python_bin)
            .args(&["-m", "pip", "install", package])
            .env("PYTHONUSERBASE", &venv.lib_path)
            .output()
            .map_err(|e| VenvError::InstallFailed(format!("Failed to run pip: {}", e)))?;
        
        if !output.status.success() {
            return Err(VenvError::InstallFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        println!("Package installed: {}", package);
        Ok(())
    }

    fn install_npm_package(&self, venv: &VirtualEnvironment, package: &str) -> Result<(), VenvError> {
        println!("Installing npm package: {}", package);
        
        let node_bin = venv.nodejs_path.as_ref()
            .ok_or_else(|| VenvError::RuntimeNotFound("Node.js not installed in venv".to_string()))?;
        
        // Find npm (usually in same directory as node)
        let npm_bin = node_bin.parent().unwrap().join("npm");
        
        // Run npm install
        let output = Command::new(npm_bin)
            .args(&["install", package])
            .current_dir(&venv.lib_path)
            .output()
            .map_err(|e| VenvError::InstallFailed(format!("Failed to run npm: {}", e)))?;
        
        if !output.status.success() {
            return Err(VenvError::InstallFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        println!("Package installed: {}", package);
        Ok(())
    }

    fn install_cargo_crate(&self, venv: &VirtualEnvironment, crate_name: &str) -> Result<(), VenvError> {
        println!("Installing Cargo crate: {}", crate_name);
        
        let rustc_bin = venv.rust_path.as_ref()
            .ok_or_else(|| VenvError::RuntimeNotFound("Rust not installed in venv".to_string()))?;
        
        // Find cargo (usually in same directory as rustc)
        let cargo_bin = rustc_bin.parent().unwrap().join("cargo");
        
        // Run cargo install
        let output = Command::new(cargo_bin)
            .args(&["install", crate_name])
            .env("CARGO_HOME", &venv.lib_path)
            .output()
            .map_err(|e| VenvError::InstallFailed(format!("Failed to run cargo: {}", e)))?;
        
        if !output.status.success() {
            return Err(VenvError::InstallFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        println!("Crate installed: {}", crate_name);
        Ok(())
    }

    /// Destroy a virtual environment
    pub fn destroy(&mut self, id: &str) -> Result<(), VenvError> {
        let venv = self.venvs.remove(id)
            .ok_or_else(|| VenvError::NotFound(id.to_string()))?;
        
        // Remove venv directory
        fs::remove_dir_all(&venv.root_path)
            .map_err(|e| VenvError::DestroyFailed(format!("Failed to remove venv directory: {}", e)))?;
        
        println!("Destroyed virtual environment: {}", venv.name);
        Ok(())
    }

    /// List all virtual environments
    pub fn list(&self) -> Vec<&VirtualEnvironment> {
        self.venvs.values().collect()
    }

    /// Get a virtual environment by ID
    pub fn get(&self, id: &str) -> Option<&VirtualEnvironment> {
        self.venvs.get(id)
    }
}

/// Language enum for runtime installation
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Python,
    JavaScript,
    Rust,
    C,
}

/// Virtual environment errors
#[derive(Debug)]
pub enum VenvError {
    NotFound(String),
    CreationFailed(String),
    InstallFailed(String),
    RuntimeNotFound(String),
    DestroyFailed(String),
    UnsupportedOperation(String),
}

impl std::fmt::Display for VenvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenvError::NotFound(msg) => write!(f, "Virtual environment not found: {}", msg),
            VenvError::CreationFailed(msg) => write!(f, "Failed to create venv: {}", msg),
            VenvError::InstallFailed(msg) => write!(f, "Failed to install: {}", msg),
            VenvError::RuntimeNotFound(msg) => write!(f, "Runtime not found: {}", msg),
            VenvError::DestroyFailed(msg) => write!(f, "Failed to destroy venv: {}", msg),
            VenvError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
        }
    }
}

impl std::error::Error for VenvError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_venv() {
        let mut manager = VenvManager::new();
        let id = manager.create("test_venv", SecurityLevel::Restricted).unwrap();
        assert!(manager.get(&id).is_some());
    }

    #[test]
    fn test_activate_deactivate() {
        let mut manager = VenvManager::new();
        let id = manager.create("test_venv", SecurityLevel::Restricted).unwrap();
        
        manager.activate(&id).unwrap();
        assert!(manager.get(&id).unwrap().activated);
        
        manager.deactivate(&id).unwrap();
        assert!(!manager.get(&id).unwrap().activated);
    }
}
