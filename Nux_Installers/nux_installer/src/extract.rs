use std::fs;
use std::io;
use std::path::PathBuf;

fn ensure_dir_exists(path: &std::path::Path) -> Result<(), String> {
    if path.exists() {
        if path.is_dir() {
            return Ok(());
        }
        std::fs::remove_file(path).ok();
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            ensure_dir_exists(parent)?;
        }
    }
    std::fs::create_dir(path).or_else(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(format!("Failed to create dir {}: {}", path.display(), e))
        }
    })
}

pub fn extract_zip_archive<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    target_dir: &PathBuf,
) -> Result<(), String> {
    ensure_dir_exists(target_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            ensure_dir_exists(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                ensure_dir_exists(p)?;
            }
            if outpath.exists() {
                if outpath.is_dir() {
                    fs::remove_dir_all(&outpath).ok();
                } else {
                    fs::remove_file(&outpath).ok();
                }
            }
            let mut outfile = match fs::File::create(&outpath) {
                Ok(f) => f,
                Err(e) => return Err(format!("Failed to create file {}: {}", outpath.display(), e)),
            };
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }

        // On Unix, try to preserve executable permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).ok();
            }
        }
    }
    Ok(())
}
