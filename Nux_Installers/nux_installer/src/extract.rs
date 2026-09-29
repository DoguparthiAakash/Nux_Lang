use std::fs;
use std::io;
use std::path::PathBuf;

pub fn extract_zip_archive<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    target_dir: &PathBuf,
) -> Result<(), String> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).map_err(|e| e.to_string())?;
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).ok();
                }
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
                Err(e) => return Err(format!("Failed to create {}: {}", outpath.display(), e)),
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
