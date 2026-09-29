use super::PlatformInstaller;
use std::path::PathBuf;
use directories::UserDirs;
use std::fs;

pub struct UnixInstaller;

impl PlatformInstaller for UnixInstaller {
    fn get_install_dir(&self) -> PathBuf {
        if let Some(user_dirs) = UserDirs::new() {
            return user_dirs.home_dir().join(".nux");
        }
        PathBuf::from("/usr/local/nux")
    }

    fn detect_existing(&self) -> Option<PathBuf> {
        let path = self.get_install_dir();
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    fn install_env_vars(&self, install_dir: &PathBuf) -> Result<(), String> {
        if let Some(user_dirs) = UserDirs::new() {
            let home = user_dirs.home_dir();
            let bin_path = install_dir.join("bin").to_string_lossy().to_string();
            let export_str = format!("\nexport NUX_HOME=\"{}\"\nexport PATH=\"$NUX_HOME/bin:$PATH\"\n", install_dir.to_string_lossy());

            let profiles = [home.join(".bashrc"), home.join(".zshrc"), home.join(".profile")];
            for profile in profiles.iter() {
                if profile.exists() {
                    let content = fs::read_to_string(profile).unwrap_or_default();
                    if !content.contains("NUX_HOME") {
                        let _ = fs::write(profile, format!("{}{}", content, export_str));
                    }
                }
            }
        }
        Ok(())
    }

    fn remove_env_vars(&self, _install_dir: &PathBuf) -> Result<(), String> {
        // We typically don't automatically remove lines from .bashrc to avoid corrupting it,
        // but we could implement basic string replacement here if needed.
        Ok(())
    }

    fn create_file_associations(&self, _install_dir: &PathBuf) -> Result<(), String> {
        // Unix desktop file associations (MIME types) could be added here
        Ok(())
    }

    fn remove_file_associations(&self) -> Result<(), String> {
        Ok(())
    }
}
