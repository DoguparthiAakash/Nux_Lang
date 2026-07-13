use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

pub fn init_project(name: &str) -> Result<(), String> {
    let dir = Path::new(name);
    if dir.exists() {
        return Err(format!("Directory '{}' already exists.", name));
    }

    fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    // Create nux.toml
    let toml_path = dir.join("nux.toml");
    let mut toml_file = fs::File::create(&toml_path).map_err(|e| e.to_string())?;
    
    let toml_content = format!(
r#"[project]
name = "{}"
version = "0.1.0"
target_hardware = "board.nuxg"

[dependencies]
"#, name);
    
    toml_file.write_all(toml_content.as_bytes()).map_err(|e| e.to_string())?;

    // Create a default board.nuxg
    let board_path = dir.join("board.nuxg");
    let mut board_file = fs::File::create(&board_path).map_err(|e| e.to_string())?;
    board_file.write_all(b"@hardware(\"DefaultBoard\")\n\n# Link your hardware libraries here\n# link \"lib/gpio.nuxel\"\n").map_err(|e| e.to_string())?;

    // Create main.nux
    let main_path = dir.join("main.nux");
    let mut main_file = fs::File::create(&main_path).map_err(|e| e.to_string())?;
    main_file.write_all(b"import \"board.nuxg\";\n\nfunc main() {\n    print(12345);\n}\n\nmain();\n").map_err(|e| e.to_string())?;

    println!("Successfully created project '{}'.", name);
    println!("Run `nux venv` inside the folder to isolate your environment.");
    
    Ok(())
}

pub fn create_venv() -> Result<(), String> {
    let env_dir = Path::new(".nuxenv");
    
    if env_dir.exists() {
        return Err(String::from("Virtual environment already exists in this directory."));
    }

    fs::create_dir_all(env_dir.join("lib")).map_err(|e| e.to_string())?;
    fs::create_dir_all(env_dir.join("cache")).map_err(|e| e.to_string())?;

    // Add .gitignore inside .nuxenv to prevent it from being committed by default
    let gitignore_path = env_dir.join(".gitignore");
    if let Ok(mut f) = fs::File::create(&gitignore_path) {
        let _ = f.write_all(b"*\n");
    }

    println!("Initialized empty Nux virtual environment in .nuxenv/");
    Ok(())
}

pub fn get_venv_lib_path() -> Option<PathBuf> {
    let env_dir = Path::new(".nuxenv");
    if env_dir.exists() && env_dir.is_dir() {
        let lib_dir = env_dir.join("lib");
        if lib_dir.exists() {
            return Some(lib_dir);
        }
    }
    None
}

pub fn get_venv_cache_path() -> Option<PathBuf> {
    let env_dir = Path::new(".nuxenv");
    if env_dir.exists() && env_dir.is_dir() {
        let cache_dir = env_dir.join("cache");
        if cache_dir.exists() {
            return Some(cache_dir);
        }
    }
    None
}

pub struct ProjectConfig {
    pub name: String,
    pub target_hardware: Option<String>,
}

pub fn parse_nux_toml() -> Option<ProjectConfig> {
    let path = Path::new("nux.toml");
    if !path.exists() {
        return None;
    }
    
    if let Ok(content) = fs::read_to_string(path) {
        let mut name = String::new();
        let mut target_hardware = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("name =") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    name = parts[1].trim().trim_matches('"').to_string();
                }
            } else if line.starts_with("target_hardware =") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    target_hardware = Some(parts[1].trim().trim_matches('"').to_string());
                }
            }
        }

        if !name.is_empty() {
            return Some(ProjectConfig { name, target_hardware });
        }
    }
    None
}
