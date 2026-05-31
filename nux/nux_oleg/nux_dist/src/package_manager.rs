use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io::{self, Cursor};
use std::env;
use semver::VersionReq;
use zip::ZipArchive;
use crate::bonfort_config::{BonfortConfig, BonfortLock, LockedPackage, resolve_version};

pub const GLOBAL_NUX_LIB: &str = "/usr/local/lib/nux";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallTarget {
    Local,
    Venv,
    Global,
}

impl InstallTarget {
    pub fn get_path(&self) -> PathBuf {
        match self {
            InstallTarget::Venv => {
                if let Ok(env_path) = env::var("NUX_LIB_PATH") {
                    PathBuf::from(env_path).join("external")
                } else {
                    Path::new("lib").join("external")
                }
            }
            InstallTarget::Global => PathBuf::from(GLOBAL_NUX_LIB).join("external"),
            InstallTarget::Local => Path::new("lib").join("external"),
        }
    }

    pub fn auto_detect() -> Self {
        if env::var("NUX_LIB_PATH").is_ok() {
            InstallTarget::Venv
        } else if Path::new("nux.toml").exists() || Path::new("lib").exists() {
            InstallTarget::Local
        } else if Path::new(GLOBAL_NUX_LIB).exists() {
            InstallTarget::Global
        } else {
            InstallTarget::Local
        }
    }
}

// Mock Registry for initial implementation
fn get_registry() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("std", "https://github.com/nux-lang/std/archive/refs/heads/main.zip"); 
    // Use a real repo for testing (octocat/Hello-World)
    m.insert("gui", "https://github.com/octocat/Hello-World/archive/refs/heads/master.zip");
    m.insert("ai", "https://github.com/nux-lang/ai/archive/refs/heads/main.zip");
    m.insert("game", "https://github.com/nux-lang/game/archive/refs/heads/main.zip");
    m
}

fn download_and_extract(url: &str, target_dir: &Path) -> Result<(), String> {
    let response = reqwest::blocking::get(url).map_err(|e| format!("Failed to download: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    
    let bytes = response.bytes().map_err(|e| format!("Failed to read bytes: {}", e))?;
    let cursor = Cursor::new(bytes);
    
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Failed to parse zip: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // Remove the top-level directory from the path
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let mut components = path.components();
                components.next(); // Skip the root directory
                let rest = components.as_path();
                if rest.as_os_str().is_empty() { continue; }
                target_dir.join(rest)
            },
            None => continue,
        };
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create dir: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| format!("Failed to create dir: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| format!("Failed to create file: {}", e))?;
            io::copy(&mut file, &mut outfile).map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }
    
    Ok(())
}

pub fn install(package_name: &str, version_req: &str, target: InstallTarget) {
    println!("Installing package: {} (version: {})", package_name, version_req);
    
    // SemVer validation
    let _req = match VersionReq::parse(version_req) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Invalid version requirement '{}': {}", version_req, e);
            if version_req != "latest" && version_req != "*" {
                return;
            }
            VersionReq::STAR
        }
    };

    let registry = get_registry();
    let url = match registry.get(package_name) {
        Some(u) => *u,
        None => {
            eprintln!("Error: Package '{}' not found in registry.", package_name);
            return;
        }
    };
    
    let target_base = target.get_path();
    println!("Target path: {}", target_base.display());

    if !target_base.exists() {
        if let Err(e) = fs::create_dir_all(&target_base) {
            eprintln!("Error creating {}: {}", target_base.display(), e);
            return;
        }
    }
    
    let package_dir = target_base.join(package_name);
    if package_dir.exists() {
         println!("Removing existing installation...");
         let _ = fs::remove_dir_all(&package_dir);
    }
    
    if let Err(e) = fs::create_dir_all(&package_dir) {
        eprintln!("Error creating package directory: {}", e);
        return;
    }
    
    println!("Downloading and extracting from: {}", url);
    
    if let Err(e) = download_and_extract(url, &package_dir) {
        eprintln!("Installation failed: {}", e);
        let _ = fs::remove_dir_all(&package_dir);
        return;
    }
    
    println!("Successfully installed {} v{}", package_name, version_req);
}

pub fn install_from_config(target: InstallTarget) {
    let bonfort_toml = Path::new("nux.toml");
    let bonfort_lock = Path::new("nux.lock");
    
    if !bonfort_toml.exists() {
        eprintln!("Error: nux.toml not found. Run 'nux new' to create a project.");
        return;
    }

    let config = match BonfortConfig::from_file(bonfort_toml) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading nux.toml: {}", e);
            return;
        }
    };

    println!("Resolving dependencies for package: {}", config.package.name);
    
    let mut lock = if bonfort_lock.exists() {
        BonfortLock::from_file(bonfort_lock).unwrap_or_else(|_| BonfortLock::new())
    } else {
        BonfortLock::new()
    };

    let mut installed_at_least_one = false;

    for (name, version_req) in &config.dependencies {
        // Check if already in lock and matches version_req
        let mut needs_install = true;
        
        if let Some(locked) = lock.get_package(name) {
            if resolve_version(version_req, &locked.version) {
                 println!("✓ {} {} (locked)", name, locked.version);
                 needs_install = false;
            }
        }

        if needs_install {
            // In a real system, we'd query the registry for latest matching version
            // For mock, we just say we found something
            let resolved_version = "1.0.0"; // Mock
            install(name, version_req, target);
            
            lock.add_package(LockedPackage {
                name: name.clone(),
                version: resolved_version.to_string(),
                source: "registry".to_string(),
                checksum: None,
                dependencies: Vec::new(),
            });
            installed_at_least_one = true;
        }
    }

    if installed_at_least_one {
        if let Err(e) = lock.to_file(bonfort_lock) {
            eprintln!("Warning: Could not save nux.lock: {}", e);
        } else {
            println!("✓ Created/Updated nux.lock");
        }
    } else if !config.dependencies.is_empty() {
        println!("All dependencies satisfied by nux.lock");
    } else {
        println!("No dependencies found in nux.toml");
    }
}

pub fn install_from_file(file_path: &str, target: InstallTarget) {
    println!("Installing from requirements file: {}", file_path);
    
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading requirements file '{}': {}", file_path, e);
            return;
        }
    };
    
    let mut count = 0;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        install(trimmed, "*", target);
        count += 1;
    }
    
    if count == 0 {
        println!("No packages found in requirements file.");
    } else {
        println!("✓ Installed {} packages from {}", count, file_path);
    }
}

pub fn remove(package_name: &str, target: InstallTarget) {
    println!("Removing package: {}", package_name);
    
    let target_base = target.get_path();
    let target_dir = target_base.join(package_name);
    
    if target_dir.exists() {
        match fs::remove_dir_all(&target_dir) {
            Ok(_) => println!("✓ Package removed from {}.", target_base.display()),
            Err(e) => eprintln!("Error removing package: {}", e),
        }
    } else {
        println!("Package not found in {}.", target_base.display());
    }
}

pub fn update() {
    println!("Updating package registry...");
    // In this mock version, nothing to download
    println!("✓ Registry updated (Mock).");
}

pub fn update_package(package_name: &str, target: InstallTarget) {
    println!("Updating package: {}", package_name);
    
    let target_base = target.get_path();
    let package_dir = target_base.join(package_name);
    if !package_dir.exists() {
        eprintln!("Package '{}' is not installed in {}", package_name, target_base.display());
        return;
    }
    
    // Reinstall to get latest version
    println!("Reinstalling to latest version...");
    install(package_name, "*", target);
}

pub fn update_all(target: InstallTarget) {
    println!("Updating all packages in {}...", target.get_path().display());
    
    let target_base = target.get_path();

    if target_base.exists() {
        if let Ok(entries) = fs::read_dir(&target_base) {
            let mut packages = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            if let Ok(name) = entry.file_name().into_string() {
                                packages.push(name);
                            }
                        }
                    }
                }
            }
            
            if packages.is_empty() {
                println!("No packages to update");
                return;
            }
            
            for package in packages {
                update_package(&package, target);
            }
            
            println!("✓ All packages updated");
        }
    } else {
        println!("No packages installed in {}", target_base.display());
    }
}

pub fn list(target: InstallTarget) {
    let target_base = target.get_path();
    println!("Installed Packages (Target: {:?}, Path: {}):", target, target_base.display());
    
    if target_base.exists() {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(target_base) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                             if let Ok(name) = entry.file_name().into_string() {
                                 println!(" - {}", name);
                                 count += 1;
                             }
                        }
                    }
                }
            }
        }
        if count == 0 {
            println!(" (No packages found)");
        }
    } else {
        println!(" (Directory does not exist)");
    }
}
