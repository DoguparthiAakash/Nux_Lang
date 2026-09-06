use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

pub struct BinaryVersioning {
    source_path: PathBuf,
    output_dir: PathBuf,
    max_versions: usize,
}

impl BinaryVersioning {
    pub fn new(source_path: &Path, max_versions: usize) -> Self {
        let source_stem = source_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let output_dir = source_path.parent()
            .unwrap_or(Path::new("."))
            .join(format!("{}_nux", source_stem));
        
        Self {
            source_path: source_path.to_path_buf(),
            output_dir,
            max_versions,
        }
    }
    
    pub fn save_version(&self, binary: &[u8]) -> Result<PathBuf, String> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        // Find next version number
        let next_version = self.find_next_version()?;
        
        // Generate versioned filename
        let source_stem = self.source_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let versioned_path = self.output_dir.join(format!("{}.nuxi.v{}", source_stem, next_version));
        let symlink_path = self.output_dir.join(format!("{}.nuxi", source_stem));
        
        // Write binary to versioned file
        let mut file = fs::File::create(&versioned_path)
            .map_err(|e| format!("Failed to create versioned binary: {}", e))?;
        file.write_all(binary)
            .map_err(|e| format!("Failed to write binary: {}", e))?;
        
        // Update symlink to point to latest version
        #[cfg(unix)]
        {
            // Remove old symlink if exists
            let _ = fs::remove_file(&symlink_path);
            
            // Create new symlink
            std::os::unix::fs::symlink(
                versioned_path.file_name().unwrap(),
                &symlink_path
            ).map_err(|e| format!("Failed to create symlink: {}", e))?;
        }
        
        #[cfg(not(unix))]
        {
            // On Windows, just copy the file
            fs::copy(&versioned_path, &symlink_path)
                .map_err(|e| format!("Failed to copy to main binary: {}", e))?;
        }
        
        // Cleanup old versions
        self.cleanup_old_versions()?;
        
        println!("Saved previous version to {}", versioned_path.display());
        
        Ok(symlink_path)
    }
    
    fn find_next_version(&self) -> Result<usize, String> {
        let source_stem = self.source_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let prefix = format!("{}.nuxi.v", source_stem);
        
        let mut max_version = 0;
        
        if let Ok(entries) = fs::read_dir(&self.output_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with(&prefix) {
                        if let Some(version_str) = filename.strip_prefix(&prefix) {
                            if let Ok(version) = version_str.parse::<usize>() {
                                max_version = max_version.max(version);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(max_version + 1)
    }
    
    fn cleanup_old_versions(&self) -> Result<(), String> {
        let source_stem = self.source_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let prefix = format!("{}.nuxi.v", source_stem);
        
        // Collect all version files
        let mut versions: Vec<(usize, PathBuf)> = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.output_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with(&prefix) {
                        if let Some(version_str) = filename.strip_prefix(&prefix) {
                            if let Ok(version) = version_str.parse::<usize>() {
                                versions.push((version, entry.path()));
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by version number (descending)
        versions.sort_by(|a, b| b.0.cmp(&a.0));
        
        // Keep only max_versions, delete the rest
        for (_, path) in versions.iter().skip(self.max_versions) {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("Warning: Failed to remove old version {}: {}", path.display(), e);
            }
        }
        
        Ok(())
    }
}
