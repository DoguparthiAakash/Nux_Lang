use std::process::Command;

pub enum LinkerType {
    MSVC,
    GCC,
    Clang,
}

pub struct BuildToolchain {
    pub linker: LinkerType,
}

impl BuildToolchain {
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            // Simplified detection: In a real scenario we'd query the registry or vswhere.exe
            // to find the exact path to link.exe from Visual Studio Build Tools.
            BuildToolchain { linker: LinkerType::MSVC }
        }
        #[cfg(target_os = "linux")]
        {
            BuildToolchain { linker: LinkerType::GCC }
        }
        #[cfg(target_os = "macos")]
        {
            BuildToolchain { linker: LinkerType::Clang }
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            BuildToolchain { linker: LinkerType::GCC } // BSD fallback
        }
    }

    pub fn link_executable(&self, obj_file: &str, output_exe: &str) -> Result<(), String> {
        match self.linker {
            LinkerType::MSVC => {
                println!("[Build Tools] Invoking MSVC link.exe to build {}...", output_exe);
                // Command::new("link.exe").arg(obj_file).arg(format!("/OUT:{}", output_exe)).status()
                Ok(())
            },
            LinkerType::GCC | LinkerType::Clang => {
                let cc = match self.linker {
                    LinkerType::GCC => "gcc",
                    _ => "clang",
                };
                println!("[Build Tools] Invoking {} to build {}...", cc, output_exe);
                // Command::new(cc).arg(obj_file).arg("-o").arg(output_exe).status()
                Ok(())
            }
        }
    }
}
