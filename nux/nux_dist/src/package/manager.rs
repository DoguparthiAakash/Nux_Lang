// Package Manager (nuxpm) - Cargo/npm-like package manager for Nux
// Handles dependency resolution, installation, and version management

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

/// Package manager for Nux
pub struct PackageManager {
    registry: PackageRegistry,
    resolver: DependencyResolver,
    cache_dir: PathBuf,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: HashMap<String, VersionConstraint>,
    pub dev_dependencies: HashMap<String, VersionConstraint>,
    pub license: String,
    pub repository: Option<String>,
}

/// Version constraint (semantic versioning)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionConstraint {
    Exact(String),           // =1.2.3
    GreaterThan(String),     // >1.2.3
    GreaterOrEqual(String),  // >=1.2.3
    LessThan(String),        // <1.2.3
    LessOrEqual(String),     // <=1.2.3
    Caret(String),           // ^1.2.3 (compatible)
    Tilde(String),           // ~1.2.3 (patch updates)
    Wildcard(String),        // 1.2.*
}

/// Package registry (local + remote)
pub struct PackageRegistry {
    local_packages: HashMap<String, Vec<Package>>,
    remote_url: String,
}

/// Dependency resolver
pub struct DependencyResolver {
    resolved: HashMap<String, String>,
}

/// Lock file for reproducible builds
#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    pub version: String,
    pub packages: HashMap<String, LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub source: PackageSource,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Registry,
    Git { url: String, rev: String },
    Path { path: String },
}

impl PackageManager {
    pub fn new() -> Self {
        let cache_dir = Self::get_cache_directory();
        
        PackageManager {
            registry: PackageRegistry::new(),
            resolver: DependencyResolver::new(),
            cache_dir,
        }
    }

    fn get_cache_directory() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()))
                .join(".nux").join("cache")
        }
        
        #[cfg(target_os = "macos")]
        {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()))
                .join(".nux").join("cache")
        }
        
        #[cfg(target_os = "windows")]
        {
            PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Temp".to_string()))
                .join(".nux").join("cache")
        }
    }

    /// Initialize package manager
    pub fn initialize(&self) -> Result<(), PackageError> {
        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        println!("[NUXPM] Package manager initialized");
        Ok(())
    }

    /// Install a package
    pub fn install(&mut self, package_name: &str, constraint: Option<VersionConstraint>) -> Result<(), PackageError> {
        println!("[NUXPM] Installing package: {}", package_name);

        // Resolve version
        let version = if let Some(constraint) = constraint {
            self.resolver.resolve_version(package_name, &constraint)?
        } else {
            self.registry.get_latest_version(package_name)?
        };

        // Check if already installed
        if self.is_installed(package_name, &version) {
            println!("[NUXPM] Package {} v{} already installed", package_name, version);
            return Ok(());
        }

        // Download package
        let package = self.registry.fetch_package(package_name, &version)?;

        // Resolve dependencies
        let dependencies = self.resolver.resolve_dependencies(&package)?;

        // Install dependencies first
        for (dep_name, dep_version) in dependencies {
            if !self.is_installed(&dep_name, &dep_version) {
                self.install(&dep_name, Some(VersionConstraint::Exact(dep_version)))?;
            }
        }

        // Install package
        self.install_package(&package)?;

        println!("[NUXPM] Package {} v{} installed successfully", package_name, version);
        Ok(())
    }

    /// Uninstall a package
    pub fn uninstall(&mut self, package_name: &str) -> Result<(), PackageError> {
        println!("[NUXPM] Uninstalling package: {}", package_name);

        let package_dir = self.cache_dir.join(package_name);
        if package_dir.exists() {
            fs::remove_dir_all(&package_dir)
                .map_err(|e| PackageError::IoError(e.to_string()))?;
        }

        println!("[NUXPM] Package {} uninstalled", package_name);
        Ok(())
    }

    /// Update a package
    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        println!("[NUXPM] Updating package: {}", package_name);

        // Get latest version
        let latest_version = self.registry.get_latest_version(package_name)?;

        // Uninstall old version
        self.uninstall(package_name)?;

        // Install latest version
        self.install(package_name, Some(VersionConstraint::Exact(latest_version)))?;

        println!("[NUXPM] Package {} updated", package_name);
        Ok(())
    }

    /// Search for packages
    pub fn search(&self, query: &str) -> Vec<Package> {
        self.registry.search(query)
    }

    /// List installed packages
    pub fn list_installed(&self) -> Vec<(String, String)> {
        let mut packages = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    // Read version from package metadata
                    if let Ok(version) = self.get_installed_version(&name) {
                        packages.push((name, version));
                    }
                }
            }
        }

        packages
    }

    /// Generate lock file
    pub fn generate_lock_file(&self, project_dir: &Path) -> Result<(), PackageError> {
        let lock_file = LockFile {
            version: "1".to_string(),
            packages: HashMap::new(),
        };

        let lock_path = project_dir.join("nux.lock");
        let lock_content = serde_json::to_string_pretty(&lock_file)
            .map_err(|e| PackageError::SerializationError(e.to_string()))?;

        fs::write(&lock_path, lock_content)
            .map_err(|e| PackageError::IoError(e.to_string()))?;

        println!("[NUXPM] Lock file generated: {}", lock_path.display());
        Ok(())
    }

    /// Install from lock file
    pub fn install_from_lock(&mut self, project_dir: &Path) -> Result<(), PackageError> {
        let lock_path = project_dir.join("nux.lock");
        
        if !lock_path.exists() {
            return Err(PackageError::LockFileNotFound);
        }

        let lock_content = fs::read_to_string(&lock_path)
            .map_err(|e| PackageError::IoError(e.to_string()))?;

        let lock_file: LockFile = serde_json::from_str(&lock_content)
            .map_err(|e| PackageError::SerializationError(e.to_string()))?;

        println!("[NUXPM] Installing from lock file...");

        for (name, locked_pkg) in lock_file.packages {
            self.install(&name, Some(VersionConstraint::Exact(locked_pkg.version)))?;
        }

        println!("[NUXPM] All packages installed from lock file");
        Ok(())
    }

    // Helper methods

    fn is_installed(&self, package_name: &str, version: &str) -> bool {
        let package_dir = self.cache_dir.join(package_name).join(version);
        package_dir.exists()
    }

    fn install_package(&self, package: &Package) -> Result<(), PackageError> {
        let package_dir = self.cache_dir.join(&package.name).join(&package.version);
        fs::create_dir_all(&package_dir)
            .map_err(|e| PackageError::IoError(e.to_string()))?;

        // Write package metadata
        let metadata_path = package_dir.join("package.json");
        let metadata = serde_json::to_string_pretty(package)
            .map_err(|e| PackageError::SerializationError(e.to_string()))?;

        fs::write(&metadata_path, metadata)
            .map_err(|e| PackageError::IoError(e.to_string()))?;

        Ok(())
    }

    fn get_installed_version(&self, package_name: &str) -> Result<String, PackageError> {
        let package_dir = self.cache_dir.join(package_name);
        
        // Find first version directory
        if let Ok(mut entries) = fs::read_dir(&package_dir) {
            if let Some(entry) = entries.next() {
                if let Ok(entry) = entry {
                    if let Ok(version) = entry.file_name().into_string() {
                        return Ok(version);
                    }
                }
            }
        }

        Err(PackageError::PackageNotFound(package_name.to_string()))
    }
}

impl PackageRegistry {
    fn new() -> Self {
        PackageRegistry {
            local_packages: HashMap::new(),
            remote_url: "https://packages.nux-lang.org".to_string(),
        }
    }

    fn fetch_package(&self, name: &str, version: &str) -> Result<Package, PackageError> {
        // In production, this would fetch from remote registry
        // For now, create a mock package
        Ok(Package {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Package {}", name),
            authors: vec!["Nux Community".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            license: "MIT".to_string(),
            repository: None,
        })
    }

    fn get_latest_version(&self, name: &str) -> Result<String, PackageError> {
        // In production, this would query the registry
        Ok("1.0.0".to_string())
    }

    fn search(&self, query: &str) -> Vec<Package> {
        // In production, this would search the registry
        vec![]
    }
}

impl DependencyResolver {
    fn new() -> Self {
        DependencyResolver {
            resolved: HashMap::new(),
        }
    }

    fn resolve_version(&self, package_name: &str, constraint: &VersionConstraint) -> Result<String, PackageError> {
        // Simplified version resolution
        match constraint {
            VersionConstraint::Exact(v) => Ok(v.clone()),
            _ => Ok("1.0.0".to_string()),
        }
    }

    fn resolve_dependencies(&self, package: &Package) -> Result<HashMap<String, String>, PackageError> {
        let mut resolved = HashMap::new();

        for (name, constraint) in &package.dependencies {
            let version = self.resolve_version(name, constraint)?;
            resolved.insert(name.clone(), version);
        }

        Ok(resolved)
    }
}

/// Package manager errors
#[derive(Debug)]
pub enum PackageError {
    IoError(String),
    PackageNotFound(String),
    VersionConflict(String),
    DependencyError(String),
    LockFileNotFound,
    SerializationError(String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::IoError(msg) => write!(f, "IO error: {}", msg),
            PackageError::PackageNotFound(name) => write!(f, "Package not found: {}", name),
            PackageError::VersionConflict(msg) => write!(f, "Version conflict: {}", msg),
            PackageError::DependencyError(msg) => write!(f, "Dependency error: {}", msg),
            PackageError::LockFileNotFound => write!(f, "Lock file not found"),
            PackageError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for PackageError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_creation() {
        let pm = PackageManager::new();
        assert!(pm.cache_dir.to_str().unwrap().contains(".nux"));
    }

    #[test]
    fn test_package_installation() {
        let mut pm = PackageManager::new();
        pm.initialize().unwrap();

        let result = pm.install("test_package", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_constraint() {
        let constraint = VersionConstraint::Exact("1.2.3".to_string());
        assert!(matches!(constraint, VersionConstraint::Exact(_)));
    }
}
