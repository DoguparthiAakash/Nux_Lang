use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn create_venv(name: &str) {
    let venv_name = if name.is_empty() { ".venv" } else { name };
    let venv_path = Path::new(venv_name);
    
    if venv_path.exists() {
        println!("Virtual environment '{}' already exists.", venv_name);
        return;
    }
    
    println!("Creating virtual environment in '{}'...", venv_name);
    
    // Create directory structure
    // .venv/
    //   bin/
    //     activate
    //     nux (symlink? or wrapper?) -> For now just activate script
    //   lib/
    //     external/
    
    let bin_dir = venv_path.join("bin");
    let lib_dir = venv_path.join("lib");
    let external_dir = lib_dir.join("external");
    
    if let Err(e) = fs::create_dir_all(&bin_dir) {
        eprintln!("Error creating bin directory: {}", e);
        return;
    }
    
    if let Err(e) = fs::create_dir_all(&external_dir) {
         eprintln!("Error creating lib directory: {}", e);
         return;
    }
    
    // Copy nux binary to venv/bin for true isolation
    match std::env::current_exe() {
        Ok(current_exe) => {
            println!("Copying nux binary from: {}", current_exe.display());
            let target_exe = bin_dir.join("nux");
            match fs::copy(&current_exe, &target_exe) {
                Ok(_) => {
                    println!("✓ Copied nux binary to venv");
                    // Make executable
                    #[cfg(unix)]
                    {
                        if let Ok(metadata) = fs::metadata(&target_exe) {
                            let mut perms = metadata.permissions();
                            perms.set_mode(0o755);
                            let _ = fs::set_permissions(&target_exe, perms);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Could not copy nux binary: {}", e);
                }
            }
            
            // Also copy bonfort binary (package manager)
            if let Some(parent) = current_exe.parent() {
                let bonfort_exe = parent.join("bonfort");
                if bonfort_exe.exists() {
                    let target_bonfort = bin_dir.join("bonfort");
                    match fs::copy(&bonfort_exe, &target_bonfort) {
                        Ok(_) => {
                            println!("✓ Copied bonfort binary to venv");
                            #[cfg(unix)]
                            {
                                if let Ok(metadata) = fs::metadata(&target_bonfort) {
                                    let mut perms = metadata.permissions();
                                    perms.set_mode(0o755);
                                    let _ = fs::set_permissions(&target_bonfort, perms);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Could not copy bonfort binary: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not determine current executable path: {}", e);
        }
    }
    
    // Create activate script (Bash/Zsh)
    let activate_path = bin_dir.join("activate");
    let abs_lib_path = std::env::current_dir().unwrap().join(&lib_dir);
    
    let abs_bin_path = std::env::current_dir().unwrap().join(&bin_dir);
    
    let activate_content = format!(r#"# This file must be used with "source bin/activate" *from bash*
# you cannot run it directly

deactivate () {{
    # reset old environment variables
    if [ -n "${{_OLD_NUX_LIB_PATH:+x}}" ]; then
        export NUX_LIB_PATH="$_OLD_NUX_LIB_PATH"
        unset _OLD_NUX_LIB_PATH
    else
        unset NUX_LIB_PATH
    fi
    
    if [ -n "${{_OLD_PATH:+x}}" ]; then
        export PATH="$_OLD_PATH"
        unset _OLD_PATH
    fi
    
    if [ -n "${{_OLD_PS1:+x}}" ]; then
        export PS1="$_OLD_PS1"
        unset _OLD_PS1
    fi
    
    if [ -n "${{BASH:-}}" ] || [ -n "${{ZSH_VERSION:-}}" ]; then
        hash -r 2> /dev/null
    fi
    
    unset NUX_VENV
    if [ ! "${{1-}}" = "nondestructive" ]; then
    # Self destruct!
        unset -f deactivate
    fi
}}

# unset irrelevant variables
deactivate nondestructive

export NUX_VENV="{}"
export _OLD_NUX_LIB_PATH="${{NUX_LIB_PATH:-}}"
export NUX_LIB_PATH="{}"

export _OLD_PATH="${{PATH:-}}"
export PATH="{}:$PATH"

export _OLD_PS1="${{PS1:-}}"
export PS1="({}) ${{PS1:-}}"

hash -r 2> /dev/null
"#, venv_name, abs_lib_path.display(), abs_bin_path.display(), venv_name);

    if let Err(e) = fs::write(&activate_path, activate_content) {
        eprintln!("Error creating activate script: {}", e);
        return;
    }
    
    // Make executable? It's sourced, but good practice.
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&activate_path).unwrap().permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(&activate_path, perms);
    }

    println!("✓ Created virtual environment.");
    println!("Activate with: source {}/bin/activate", venv_name);
}
