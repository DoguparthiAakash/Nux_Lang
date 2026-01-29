// Rust Bridge - Loads Rust dynamic libraries and provides FFI
// Enables calling Rust code from Nux via dynamic library loading

use crate::ffi_manager::{LanguageRuntime, LanguageBlock};
use crate::type_marshaller::{NuxValue, RustValue, TypeMarshaller};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Rust runtime bridge
pub struct RustBridge {
    initialized: bool,
    marshaller: Arc<Mutex<TypeMarshaller>>,
    loaded_libraries: HashMap<usize, LibraryHandle>,
    next_lib_id: usize,
    temp_build_dir: String,
}

#[derive(Debug)]
struct LibraryHandle {
    path: String,
    lib_name: String,
    // In production: libloading::Library
    // library: libloading::Library,
}

impl RustBridge {
    pub fn new(marshaller: Arc<Mutex<TypeMarshaller>>) -> Self {
        RustBridge {
            initialized: false,
            marshaller,
            loaded_libraries: HashMap::new(),
            next_lib_id: 0,
            temp_build_dir: "/tmp/nux_rust_builds".to_string(),
        }
    }

    /// Compile Rust code to a dynamic library
    fn compile_rust_code(&self, code: &str, lib_name: &str) -> Result<String, String> {
        // In production:
        // 1. Create temporary Cargo project
        // 2. Write code to src/lib.rs
        // 3. Add [lib] crate-type = ["cdylib"] to Cargo.toml
        // 4. Run cargo build --release
        // 5. Return path to .so/.dll file

        // Mock implementation
        let lib_path = format!("{}/lib{}.so", self.temp_build_dir, lib_name);
        
        // In production:
        // std::fs::create_dir_all(&self.temp_build_dir)?;
        // std::fs::write(format!("{}/src/lib.rs", self.temp_build_dir), code)?;
        // std::process::Command::new("cargo")
        //     .args(&["build", "--release"])
        //     .current_dir(&self.temp_build_dir)
        //     .output()?;

        println!("Mock: Compiling Rust code to {}", lib_path);
        Ok(lib_path)
    }

    /// Load a Rust dynamic library
    fn load_rust_library(&mut self, lib_path: &str) -> Result<usize, String> {
        // In production, use libloading:
        // let library = unsafe {
        //     libloading::Library::new(lib_path)
        //         .map_err(|e| format!("Failed to load library: {}", e))?
        // };

        let lib_id = self.next_lib_id;
        self.next_lib_id += 1;

        let handle = LibraryHandle {
            path: lib_path.to_string(),
            lib_name: format!("rust_lib_{}", lib_id),
        };

        self.loaded_libraries.insert(lib_id, handle);
        Ok(lib_id)
    }

    /// Call a Rust function from a loaded library
    fn call_rust_function(
        &mut self,
        lib_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        let _handle = self.loaded_libraries.get(&lib_id)
            .ok_or_else(|| format!("Library ID {} not found", lib_id))?;

        // In production, use libloading to call the function:
        // unsafe {
        //     let func: libloading::Symbol<unsafe extern fn(...) -> ...> = 
        //         handle.library.get(function_name.as_bytes())?;
        //     
        //     // Convert Nux args to Rust types
        //     let rust_args = args.iter().map(|a| self.nux_to_rust(a)).collect();
        //     
        //     // Call the function
        //     let result = func(rust_args);
        //     
        //     // Convert result back to Nux
        //     self.rust_to_nux(&result)
        // }

        // Mock implementation
        Ok(NuxValue::String(format!(
            "Called Rust function {} with {} args",
            function_name,
            args.len()
        )))
    }

    /// Convert Nux value to Rust value
    fn nux_to_rust(&self, nux_val: &NuxValue) -> RustValue {
        match nux_val {
            NuxValue::Null => RustValue::Unit,
            NuxValue::Bool(b) => RustValue::Bool(*b),
            NuxValue::Int(i) => RustValue::I64(*i),
            NuxValue::Float(f) => RustValue::F64(*f),
            NuxValue::String(s) => RustValue::String(s.clone()),
            NuxValue::Array(arr) => {
                let rust_arr: Vec<RustValue> = arr.iter().map(|v| self.nux_to_rust(v)).collect();
                RustValue::Vec(rust_arr)
            }
            NuxValue::Map(map) => {
                let mut rust_struct = HashMap::new();
                for (k, v) in map.iter() {
                    rust_struct.insert(k.clone(), self.nux_to_rust(v));
                }
                RustValue::Struct(rust_struct)
            }
            _ => RustValue::Unit,
        }
    }

    /// Convert Rust value to Nux value
    fn rust_to_nux(&self, rust_val: &RustValue) -> NuxValue {
        match rust_val {
            RustValue::Unit => NuxValue::Null,
            RustValue::Bool(b) => NuxValue::Bool(*b),
            RustValue::I8(i) => NuxValue::Int(*i as i64),
            RustValue::I16(i) => NuxValue::Int(*i as i64),
            RustValue::I32(i) => NuxValue::Int(*i as i64),
            RustValue::I64(i) => NuxValue::Int(*i),
            RustValue::U8(u) => NuxValue::Int(*u as i64),
            RustValue::U16(u) => NuxValue::Int(*u as i64),
            RustValue::U32(u) => NuxValue::Int(*u as i64),
            RustValue::U64(u) => NuxValue::Int(*u as i64),
            RustValue::F32(f) => NuxValue::Float(*f as f64),
            RustValue::F64(f) => NuxValue::Float(*f),
            RustValue::String(s) => NuxValue::String(s.clone()),
            RustValue::Vec(vec) => {
                let nux_arr: Vec<NuxValue> = vec.iter().map(|v| self.rust_to_nux(v)).collect();
                NuxValue::Array(nux_arr)
            }
            RustValue::Struct(map) => {
                let mut nux_map = HashMap::new();
                for (k, v) in map.iter() {
                    nux_map.insert(k.clone(), self.rust_to_nux(v));
                }
                NuxValue::Map(nux_map)
            }
            RustValue::Pointer(_) => NuxValue::Null, // Pointers not directly supported
        }
    }

    /// Load a Cargo crate
    pub fn load_cargo_crate(&mut self, crate_name: &str) -> Result<usize, String> {
        // In production:
        // 1. Check if crate is installed
        // 2. If not, run: cargo install <crate_name>
        // 3. Find the compiled library
        // 4. Load it using libloading

        println!("Mock: Loading Cargo crate: {}", crate_name);
        self.load_rust_library(&format!("/usr/lib/lib{}.so", crate_name))
    }
}

impl LanguageRuntime for RustBridge {
    fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // Check if Rust toolchain is available
        if !is_rust_available() {
            return Err("Rust toolchain not found. Please install Rust from rustup.rs".to_string());
        }

        // Create temp build directory
        // std::fs::create_dir_all(&self.temp_build_dir)?;

        self.initialized = true;
        Ok(())
    }

    fn execute_block(&mut self, block: &LanguageBlock) -> Result<NuxValue, String> {
        // Compile the Rust code block to a dynamic library
        let lib_name = format!("block_{}", self.next_lib_id);
        let lib_path = self.compile_rust_code(&block.code, &lib_name)?;
        
        // Load the library
        let lib_id = self.load_rust_library(&lib_path)?;
        
        // Return the library ID as a reference
        Ok(NuxValue::Int(lib_id as i64))
    }

    fn load_module(&mut self, module_path: &str) -> Result<usize, String> {
        // module_path could be:
        // - A path to a .so/.dll file
        // - A Cargo crate name
        // - A path to a Rust source file to compile

        if module_path.ends_with(".so") || module_path.ends_with(".dll") || module_path.ends_with(".dylib") {
            self.load_rust_library(module_path)
        } else if module_path.ends_with(".rs") {
            // Compile Rust source file
            let code = std::fs::read_to_string(module_path)
                .map_err(|e| format!("Failed to read Rust file: {}", e))?;
            let lib_name = module_path.trim_end_matches(".rs");
            let lib_path = self.compile_rust_code(&code, lib_name)?;
            self.load_rust_library(&lib_path)
        } else {
            // Assume it's a Cargo crate name
            self.load_cargo_crate(module_path)
        }
    }

    fn call_function(
        &mut self,
        module_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        self.call_rust_function(module_id, function_name, args)
    }

    fn eval_expression(&mut self, expression: &str) -> Result<NuxValue, String> {
        // Wrap expression in a function and compile
        let code = format!(
            r#"
            #[no_mangle]
            pub extern "C" fn eval_expr() -> i64 {{
                {}
            }}
            "#,
            expression
        );
        
        let lib_name = "expr_eval";
        let lib_path = self.compile_rust_code(&code, lib_name)?;
        let lib_id = self.load_rust_library(&lib_path)?;
        
        self.call_rust_function(lib_id, "eval_expr", vec![])
    }

    fn shutdown(&mut self) -> Result<(), String> {
        // Unload all libraries
        self.loaded_libraries.clear();
        self.initialized = false;
        Ok(())
    }

    fn name(&self) -> &str {
        "Rust"
    }
}

/// Helper functions for Rust integration

/// Check if Rust is available
pub fn is_rust_available() -> bool {
    // In production:
    // std::process::Command::new("rustc")
    //     .arg("--version")
    //     .output()
    //     .is_ok()
    true // Mock
}

/// Get Rust version
pub fn get_rust_version() -> Result<String, String> {
    // In production:
    // let output = std::process::Command::new("rustc")
    //     .arg("--version")
    //     .output()
    //     .map_err(|e| e.to_string())?;
    // Ok(String::from_utf8_lossy(&output.stdout).to_string())
    
    Ok("rustc 1.75.0 (mock)".to_string())
}

/// Install a Cargo crate
pub fn install_cargo_crate(crate_name: &str) -> Result<(), String> {
    // In production:
    // std::process::Command::new("cargo")
    //     .args(&["install", crate_name])
    //     .output()
    //     .map_err(|e| e.to_string())?;
    
    println!("Mock: Installing Cargo crate: {}", crate_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_bridge_initialization() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = RustBridge::new(marshaller);
        assert!(bridge.initialize().is_ok());
        assert_eq!(bridge.name(), "Rust");
    }

    #[test]
    fn test_nux_to_rust_conversion() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let bridge = RustBridge::new(marshaller);
        
        let nux_val = NuxValue::Int(42);
        let rust_val = bridge.nux_to_rust(&nux_val);
        
        match rust_val {
            RustValue::I64(i) => assert_eq!(i, 42),
            _ => panic!("Expected RustValue::I64"),
        }
    }

    #[test]
    fn test_rust_availability() {
        assert!(is_rust_available());
    }
}
