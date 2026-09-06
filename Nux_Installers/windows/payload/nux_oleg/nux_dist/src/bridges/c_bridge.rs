// C/C++ Bridge - Provides FFI for C and C++ libraries using libffi
// Enables calling C functions and loading shared libraries

use crate::ffi_manager::{LanguageRuntime, LanguageBlock};
use crate::type_marshaller::{NuxValue, CValue, TypeMarshaller};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// C/C++ runtime bridge
pub struct CBridge {
    initialized: bool,
    marshaller: Arc<Mutex<TypeMarshaller>>,
    loaded_libraries: HashMap<usize, CLibraryHandle>,
    next_lib_id: usize,
}

#[derive(Debug)]
struct CLibraryHandle {
    path: String,
    lib_name: String,
    // In production: libloading::Library or libffi bindings
}

impl CBridge {
    pub fn new(marshaller: Arc<Mutex<TypeMarshaller>>) -> Self {
        CBridge {
            initialized: false,
            marshaller,
            loaded_libraries: HashMap::new(),
            next_lib_id: 0,
        }
    }

    /// Load a C shared library (.so, .dll, .dylib)
    fn load_c_library(&mut self, lib_path: &str) -> Result<usize, String> {
        // In production, use libloading:
        // let library = unsafe {
        //     libloading::Library::new(lib_path)
        //         .map_err(|e| format!("Failed to load library: {}", e))?
        // };

        let lib_id = self.next_lib_id;
        self.next_lib_id += 1;

        let handle = CLibraryHandle {
            path: lib_path.to_string(),
            lib_name: format!("c_lib_{}", lib_id),
        };

        self.loaded_libraries.insert(lib_id, handle);
        println!("Mock: Loaded C library: {}", lib_path);
        Ok(lib_id)
    }

    /// Call a C function from a loaded library
    fn call_c_function(
        &mut self,
        lib_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
        return_type: &str,
    ) -> Result<NuxValue, String> {
        let _handle = self.loaded_libraries.get(&lib_id)
            .ok_or_else(|| format!("Library ID {} not found", lib_id))?;

        // In production, use libffi to call C functions:
        // 1. Get function pointer from library
        // 2. Prepare libffi CIF (Call Interface)
        // 3. Convert Nux args to C types
        // 4. Call function using ffi_call
        // 5. Convert result back to Nux

        // Example with libffi:
        // unsafe {
        //     let func: libloading::Symbol<unsafe extern fn(...) -> ...> = 
        //         handle.library.get(function_name.as_bytes())?;
        //     
        //     // Setup libffi
        //     let mut cif: ffi_cif = std::mem::zeroed();
        //     let arg_types = self.get_ffi_types(&args);
        //     let return_type = self.get_ffi_type(return_type);
        //     
        //     ffi_prep_cif(&mut cif, FFI_DEFAULT_ABI, args.len(), return_type, arg_types);
        //     
        //     // Prepare arguments
        //     let c_args = args.iter().map(|a| self.nux_to_c(a)).collect();
        //     
        //     // Call function
        //     let mut result: *mut c_void = std::ptr::null_mut();
        //     ffi_call(&mut cif, func, &mut result, c_args.as_ptr());
        //     
        //     // Convert result
        //     self.c_to_nux(&result, return_type)
        // }

        // Mock implementation
        Ok(NuxValue::String(format!(
            "Called C function {} with {} args, return type: {}",
            function_name,
            args.len(),
            return_type
        )))
    }

    /// Convert Nux value to C value
    fn nux_to_c(&self, nux_val: &NuxValue) -> CValue {
        match nux_val {
            NuxValue::Null => CValue::Void,
            NuxValue::Bool(b) => CValue::Bool(*b),
            NuxValue::Int(i) => {
                if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                    CValue::Int(*i as i32)
                } else {
                    CValue::Long(*i)
                }
            }
            NuxValue::Float(f) => CValue::Double(*f),
            NuxValue::String(s) => CValue::String(s.clone()),
            NuxValue::Array(arr) => {
                let c_arr: Vec<CValue> = arr.iter().map(|v| self.nux_to_c(v)).collect();
                CValue::Array(c_arr)
            }
            _ => CValue::Void,
        }
    }

    /// Convert C value to Nux value
    fn c_to_nux(&self, c_val: &CValue) -> NuxValue {
        match c_val {
            CValue::Void => NuxValue::Null,
            CValue::Bool(b) => NuxValue::Bool(*b),
            CValue::Char(c) => NuxValue::Int(*c as i64),
            CValue::Short(s) => NuxValue::Int(*s as i64),
            CValue::Int(i) => NuxValue::Int(*i as i64),
            CValue::Long(l) => NuxValue::Int(*l),
            CValue::UChar(u) => NuxValue::Int(*u as i64),
            CValue::UShort(u) => NuxValue::Int(*u as i64),
            CValue::UInt(u) => NuxValue::Int(*u as i64),
            CValue::ULong(u) => NuxValue::Int(*u as i64),
            CValue::Float(f) => NuxValue::Float(*f as f64),
            CValue::Double(d) => NuxValue::Float(*d),
            CValue::String(s) => NuxValue::String(s.clone()),
            CValue::Array(arr) => {
                let nux_arr: Vec<NuxValue> = arr.iter().map(|v| self.c_to_nux(v)).collect();
                NuxValue::Array(nux_arr)
            }
            CValue::Pointer(_) => NuxValue::Null, // Pointers not directly supported
        }
    }

    /// Compile C code to a shared library
    fn compile_c_code(&self, code: &str, lib_name: &str) -> Result<String, String> {
        // In production:
        // 1. Write code to temporary .c file
        // 2. Compile with gcc/clang: gcc -shared -fPIC -o lib.so code.c
        // 3. Return path to compiled library

        let lib_path = format!("/tmp/lib{}.so", lib_name);
        
        // In production:
        // let temp_file = format!("/tmp/{}.c", lib_name);
        // std::fs::write(&temp_file, code)?;
        // std::process::Command::new("gcc")
        //     .args(&["-shared", "-fPIC", "-o", &lib_path, &temp_file])
        //     .output()?;

        println!("Mock: Compiling C code to {}", lib_path);
        Ok(lib_path)
    }

    /// Load common system libraries
    pub fn load_system_library(&mut self, lib_name: &str) -> Result<usize, String> {
        // Common system libraries:
        // - libc.so (standard C library)
        // - libm.so (math library)
        // - libpthread.so (POSIX threads)
        // - libssl.so (OpenSSL)
        // - libsqlite3.so (SQLite)

        let lib_path = match lib_name {
            "c" => "libc.so.6",
            "m" => "libm.so.6",
            "pthread" => "libpthread.so.0",
            "ssl" => "libssl.so.3",
            "crypto" => "libcrypto.so.3",
            "sqlite3" => "libsqlite3.so.0",
            _ => return Err(format!("Unknown system library: {}", lib_name)),
        };

        self.load_c_library(lib_path)
    }
}

impl LanguageRuntime for CBridge {
    fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // Check if C compiler is available
        if !is_c_compiler_available() {
            return Err("C compiler (gcc/clang) not found".to_string());
        }

        self.initialized = true;
        Ok(())
    }

    fn execute_block(&mut self, block: &LanguageBlock) -> Result<NuxValue, String> {
        // Compile the C code block to a shared library
        let lib_name = format!("block_{}", self.next_lib_id);
        let lib_path = self.compile_c_code(&block.code, &lib_name)?;
        
        // Load the library
        let lib_id = self.load_c_library(&lib_path)?;
        
        // Return the library ID
        Ok(NuxValue::Int(lib_id as i64))
    }

    fn load_module(&mut self, module_path: &str) -> Result<usize, String> {
        // module_path could be:
        // - A path to a .so/.dll/.dylib file
        // - A system library name (e.g., "c", "m", "pthread")
        // - A path to a .c/.h file to compile

        if module_path.ends_with(".so") || module_path.ends_with(".dll") || module_path.ends_with(".dylib") {
            self.load_c_library(module_path)
        } else if module_path.ends_with(".c") {
            // Compile C source file
            let code = std::fs::read_to_string(module_path)
                .map_err(|e| format!("Failed to read C file: {}", e))?;
            let lib_name = module_path.trim_end_matches(".c");
            let lib_path = self.compile_c_code(&code, lib_name)?;
            self.load_c_library(&lib_path)
        } else {
            // Assume it's a system library name
            self.load_system_library(module_path)
        }
    }

    fn call_function(
        &mut self,
        module_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        // Default return type is int
        self.call_c_function(module_id, function_name, args, "int")
    }

    fn eval_expression(&mut self, expression: &str) -> Result<NuxValue, String> {
        // Wrap expression in a function and compile
        let code = format!(
            r#"
            int eval_expr() {{
                return {};
            }}
            "#,
            expression
        );
        
        let lib_name = "expr_eval";
        let lib_path = self.compile_c_code(&code, lib_name)?;
        let lib_id = self.load_c_library(&lib_path)?;
        
        self.call_function(lib_id, "eval_expr", vec![])
    }

    fn shutdown(&mut self) -> Result<(), String> {
        // Unload all libraries
        self.loaded_libraries.clear();
        self.initialized = false;
        Ok(())
    }

    fn name(&self) -> &str {
        "C/C++"
    }
}

/// Helper functions for C integration

/// Check if C compiler is available
pub fn is_c_compiler_available() -> bool {
    // In production:
    // std::process::Command::new("gcc")
    //     .arg("--version")
    //     .output()
    //     .is_ok() || 
    // std::process::Command::new("clang")
    //     .arg("--version")
    //     .output()
    //     .is_ok()
    true // Mock
}

/// Get C compiler version
pub fn get_c_compiler_version() -> Result<String, String> {
    // In production:
    // let output = std::process::Command::new("gcc")
    //     .arg("--version")
    //     .output()
    //     .map_err(|e| e.to_string())?;
    // Ok(String::from_utf8_lossy(&output.stdout).lines().next().unwrap().to_string())
    
    Ok("gcc 11.4.0 (mock)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_bridge_initialization() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = CBridge::new(marshaller);
        assert!(bridge.initialize().is_ok());
        assert_eq!(bridge.name(), "C/C++");
    }

    #[test]
    fn test_nux_to_c_conversion() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let bridge = CBridge::new(marshaller);
        
        let nux_val = NuxValue::Int(42);
        let c_val = bridge.nux_to_c(&nux_val);
        
        match c_val {
            CValue::Int(i) => assert_eq!(i, 42),
            _ => panic!("Expected CValue::Int"),
        }
    }

    #[test]
    fn test_c_compiler_availability() {
        assert!(is_c_compiler_available());
    }

    #[test]
    fn test_load_system_library() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = CBridge::new(marshaller);
        bridge.initialize().unwrap();
        
        let result = bridge.load_system_library("c");
        assert!(result.is_ok());
    }
}
