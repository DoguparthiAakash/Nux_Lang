use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

#[derive(Debug, Clone)]
pub struct CuxMetadata {
    pub title: String,
    pub lang: String,
    pub bindings: Vec<String>,
    pub raw_code: String,
}

pub fn parse_cux_file(content: &str) -> Result<CuxMetadata, String> {
    let mut title = "Extension".to_string();
    let mut lang = "C".to_string();
    let mut bindings = Vec::new();
    let mut raw_code = String::new();
    
    let mut lines = content.lines();
    
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with(".title") {
            title = trimmed.replace(".title", "").trim().trim_matches('"').to_string();
        } else if trimmed.starts_with(".lang") {
            lang = trimmed.replace(".lang", "").trim().trim_matches('"').to_string();
        } else if trimmed.starts_with(".bindings") {
            // Read until end of bindings
            while let Some(b_line) = lines.next() {
                let b_trim = b_line.trim();
                if b_trim.is_empty() {
                    break;
                }
                bindings.push(b_trim.to_string());
            }
        } else {
            raw_code.push_str(line);
            raw_code.push('\n');
        }
    }
    
    Ok(CuxMetadata {
        title,
        lang,
        bindings,
        raw_code,
    })
}

pub fn compile_cux(input_file: &str) -> Result<PathBuf, String> {
    let content = fs::read_to_string(input_file).map_err(|e| e.to_string())?;
    let metadata = parse_cux_file(&content)?;
    
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    let target_dir = current_dir.join("target").join("cux");
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    
    let base_name = Path::new(input_file)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
        
    let ext = if metadata.lang.to_uppercase() == "CUDA" { "cu" } else { "c" };
    let temp_source = target_dir.join(format!("{}.{}", base_name, ext));
    
    fs::write(&temp_source, metadata.raw_code).map_err(|e| e.to_string())?;
    
    // Windows dynamic library extension
    let out_dll = target_dir.join(format!("{}.dll", base_name));
    
    if metadata.lang.to_uppercase() == "CUDA" {
        // Compile using nvcc
        let status = Command::new("nvcc")
            .arg("--shared")
            .arg("-o")
            .arg(&out_dll)
            .arg(&temp_source)
            .status()
            .map_err(|e| format!("Failed to invoke nvcc: {}", e))?;
            
        if !status.success() {
            return Err("CUDA compilation failed".to_string());
        }
    } else {
        // Fallback to gcc or standard c compiler for C
        let status = Command::new("gcc")
            .arg("-shared")
            .arg("-o")
            .arg(&out_dll)
            .arg(&temp_source)
            .status()
            .map_err(|e| format!("Failed to invoke gcc: {}", e))?;
            
        if !status.success() {
            return Err("C compilation failed".to_string());
        }
    }
    
    Ok(out_dll)
}
