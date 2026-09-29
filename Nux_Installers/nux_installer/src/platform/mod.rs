#[cfg(windows)]
pub mod windows;
#[cfg(not(windows))]
pub mod unix;

#[cfg(windows)]
pub use windows::*;
#[cfg(not(windows))]
pub use unix::*;

use std::path::PathBuf;

pub trait PlatformInstaller {
    fn get_install_dir(&self) -> PathBuf;
    fn detect_existing(&self) -> Option<PathBuf>;
    fn install_env_vars(&self, install_dir: &PathBuf) -> Result<(), String>;
    fn remove_env_vars(&self, install_dir: &PathBuf) -> Result<(), String>;
    fn create_file_associations(&self, install_dir: &PathBuf) -> Result<(), String>;
    fn remove_file_associations(&self) -> Result<(), String>;
}
