use super::PlatformInstaller;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub struct WindowsInstaller;

impl PlatformInstaller for WindowsInstaller {
    fn get_install_dir(&self) -> PathBuf {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(prog_files) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion") {
            if let Ok(pf_dir) = prog_files.get_value::<String, _>("ProgramFilesDir") {
                return PathBuf::from(pf_dir).join("Nux");
            }
        }
        PathBuf::from("C:\\Program Files\\Nux")
    }

    fn detect_existing(&self) -> Option<PathBuf> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(env) = hklm.open_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment") {
            if let Ok(nux_home) = env.get_value::<String, _>("NUX_HOME") {
                return Some(PathBuf::from(nux_home));
            }
        }
        None
    }

    fn install_env_vars(&self, install_dir: &PathBuf) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (env, _) = hklm.create_subkey_with_flags(
            "System\\CurrentControlSet\\Control\\Session Manager\\Environment",
            KEY_READ | KEY_WRITE,
        ).map_err(|e| e.to_string())?;

        let bin_dir = install_dir.join("bin").to_string_lossy().to_string();
        env.set_value("NUX_HOME", &install_dir.to_string_lossy().to_string()).map_err(|e| e.to_string())?;

        let mut path: String = env.get_value("Path").unwrap_or_default();
        if !path.contains(&bin_dir) {
            if !path.ends_with(';') && !path.is_empty() {
                path.push(';');
            }
            path.push_str(&bin_dir);
            env.set_value("Path", &path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn remove_env_vars(&self, install_dir: &PathBuf) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let env = hklm.open_subkey_with_flags(
            "System\\CurrentControlSet\\Control\\Session Manager\\Environment",
            KEY_READ | KEY_WRITE,
        ).map_err(|e| e.to_string())?;

        let _ = env.delete_value("NUX_HOME");

        let bin_dir = install_dir.join("bin").to_string_lossy().to_string();
        if let Ok(path) = env.get_value::<String, _>("Path") {
            let mut parts: Vec<&str> = path.split(';').collect();
            parts.retain(|&p| p != bin_dir && p != format!("{};", bin_dir) && !p.is_empty());
            let new_path = parts.join(";");
            env.set_value("Path", &new_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn create_file_associations(&self, install_dir: &PathBuf) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let bin_path = install_dir.join("bin").join("nux.exe").to_string_lossy().to_string();
        
        let (classes, _) = hklm.create_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE).map_err(|e| e.to_string())?;
        
        let (nux_ext, _) = classes.create_subkey(".nux").map_err(|e| e.to_string())?;
        nux_ext.set_value("", &"NuxSourceFile").map_err(|e| e.to_string())?;
        
        let (nux_file, _) = classes.create_subkey("NuxSourceFile").map_err(|e| e.to_string())?;
        nux_file.set_value("", &"Nux Source Code").map_err(|e| e.to_string())?;
        let (nux_icon, _) = nux_file.create_subkey("DefaultIcon").map_err(|e| e.to_string())?;
        nux_icon.set_value("", &format!("{},0", bin_path)).map_err(|e| e.to_string())?;

        let (ncx_ext, _) = classes.create_subkey(".ncx").map_err(|e| e.to_string())?;
        ncx_ext.set_value("", &"NuxBytecodeFile").map_err(|e| e.to_string())?;

        let (ncx_file, _) = classes.create_subkey("NuxBytecodeFile").map_err(|e| e.to_string())?;
        ncx_file.set_value("", &"Nux Executable Bytecode").map_err(|e| e.to_string())?;
        let (ncx_icon, _) = ncx_file.create_subkey("DefaultIcon").map_err(|e| e.to_string())?;
        ncx_icon.set_value("", &format!("{},0", bin_path)).map_err(|e| e.to_string())?;

        let (ncx_cmd, _) = ncx_file.create_subkey("shell\\open\\command").map_err(|e| e.to_string())?;
        ncx_cmd.set_value("", &format!("\"{}\" run \"%1\" %*", bin_path)).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn remove_file_associations(&self) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(classes) = hklm.open_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE) {
            let _ = classes.delete_subkey_all(".nux");
            let _ = classes.delete_subkey_all("NuxSourceFile");
            let _ = classes.delete_subkey_all(".ncx");
            let _ = classes.delete_subkey_all("NuxBytecodeFile");
        }
        Ok(())
    }
}
