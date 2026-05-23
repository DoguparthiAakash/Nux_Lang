// FFI Manager - Central coordinator for foreign function interface
// Manages language runtimes, foreign function calls, and cross-language interop

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::polyglot_parser::{ForeignLanguage, LanguageBlock, ExternalImport};
use crate::type_marshaller::{TypeMarshaller, NuxValue};

/// FFI Manager - coordinates all foreign language interactions
pub struct FFIManager {
    /// Type marshaller for value conversions
    marshaller: Arc<Mutex<TypeMarshaller>>,
    
    /// Registered language runtimes
    runtimes: HashMap<ForeignLanguage, Box<dyn LanguageRuntime>>,
    
    /// Loaded external modules
    loaded_modules: HashMap<String, ModuleHandle>,
    
    /// Registered foreign functions
    foreign_functions: HashMap<String, ForeignFunction>,
    
    /// Security sandbox configuration
    sandbox_config: SandboxConfig,
}

/// Handle to a loaded foreign module
#[derive(Debug, Clone)]
pub struct ModuleHandle {
    pub language: ForeignLanguage,
    pub module_path: String,
    pub module_id: usize,
}

/// Represents a foreign function that can be called from Nux
#[derive(Debug, Clone)]
pub struct ForeignFunction {
    pub language: ForeignLanguage,
    pub module: String,
    pub name: String,
    pub signature: FunctionSignature,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub params: Vec<String>,
    pub return_type: String,
}

/// Security sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub level: SandboxLevel,
    pub allowed_filesystem_paths: Vec<String>,
    pub allow_network: bool,
    pub allow_system_calls: bool,
    pub max_memory_mb: usize,
    pub max_execution_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SandboxLevel {
    Trusted,      // No restrictions
    Restricted,   // Limited access
    Isolated,     // Separate process
}

impl Default for SandboxConfig {
    fn default() -> Self {
        SandboxConfig {
            level: SandboxLevel::Restricted,
            allowed_filesystem_paths: vec![],
            allow_network: false,
            allow_system_calls: false,
            max_memory_mb: 512,
            max_execution_time_ms: 30000,
        }
    }
}

/// Trait for language runtime implementations
pub trait LanguageRuntime: Send + Sync {
    /// Initialize the runtime
    fn initialize(&mut self) -> Result<(), String>;
    
    /// Execute a code block
    fn execute_block(&mut self, block: &LanguageBlock) -> Result<NuxValue, String>;
    
    /// Load an external module
    fn load_module(&mut self, module_path: &str) -> Result<usize, String>;
    
    /// Call a foreign function
    fn call_function(
        &mut self,
        module_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String>;
    
    /// Evaluate an inline expression
    fn eval_expression(&mut self, expression: &str) -> Result<NuxValue, String>;
    
    /// Shutdown the runtime
    fn shutdown(&mut self) -> Result<(), String>;
    
    /// Get runtime name
    fn name(&self) -> &str;
}

impl FFIManager {
    pub fn new() -> Self {
        FFIManager {
            marshaller: Arc::new(Mutex::new(TypeMarshaller::new())),
            runtimes: HashMap::new(),
            loaded_modules: HashMap::new(),
            foreign_functions: HashMap::new(),
            sandbox_config: SandboxConfig::default(),
        }
    }

    /// Register a language runtime
    pub fn register_runtime(
        &mut self,
        language: ForeignLanguage,
        runtime: Box<dyn LanguageRuntime>,
    ) -> Result<(), String> {
        if self.runtimes.contains_key(&language) {
            return Err(format!("Runtime for {:?} already registered", language));
        }
        
        self.runtimes.insert(language, runtime);
        Ok(())
    }

    /// Initialize a language runtime
    pub fn initialize_runtime(&mut self, language: &ForeignLanguage) -> Result<(), String> {
        let runtime = self.runtimes.get_mut(language)
            .ok_or_else(|| format!("Runtime for {:?} not registered", language))?;
        
        runtime.initialize()
    }

    /// Execute a language block
    pub fn execute_block(&mut self, block: &LanguageBlock) -> Result<NuxValue, String> {
        let runtime = self.runtimes.get_mut(&block.language)
            .ok_or_else(|| format!("Runtime for {:?} not available", block.language))?;
        
        runtime.execute_block(block)
    }

    /// Load an external module
    pub fn load_module(&mut self, import: &ExternalImport) -> Result<ModuleHandle, String> {
        let module_key = format!("{}:{}", import.language.to_str(), import.module_path);
        
        // Check if already loaded
        if let Some(handle) = self.loaded_modules.get(&module_key) {
            return Ok(handle.clone());
        }
        
        let runtime = self.runtimes.get_mut(&import.language)
            .ok_or_else(|| format!("Runtime for {:?} not available", import.language))?;
        
        let module_id = runtime.load_module(&import.module_path)?;
        
        let handle = ModuleHandle {
            language: import.language.clone(),
            module_path: import.module_path.clone(),
            module_id,
        };
        
        self.loaded_modules.insert(module_key, handle.clone());
        Ok(handle)
    }

    /// Call a foreign function
    pub fn call_foreign_function(
        &mut self,
        language: &ForeignLanguage,
        module_path: &str,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        let module_key = format!("{}:{}", language.to_str(), module_path);
        
        let module_handle = self.loaded_modules.get(&module_key)
            .ok_or_else(|| format!("Module {} not loaded", module_key))?;
        
        let runtime = self.runtimes.get_mut(language)
            .ok_or_else(|| format!("Runtime for {:?} not available", language))?;
        
        runtime.call_function(module_handle.module_id, function_name, args)
    }

    /// Evaluate an inline expression
    pub fn eval_inline(
        &mut self,
        language: &ForeignLanguage,
        expression: &str,
    ) -> Result<NuxValue, String> {
        let runtime = self.runtimes.get_mut(language)
            .ok_or_else(|| format!("Runtime for {:?} not available", language))?;
        
        runtime.eval_expression(expression)
    }

    /// Register a foreign function for later calls
    pub fn register_function(
        &mut self,
        language: ForeignLanguage,
        module: String,
        name: String,
        signature: FunctionSignature,
    ) {
        let key = format!("{}::{}", module, name);
        let func = ForeignFunction {
            language,
            module,
            name,
            signature,
        };
        self.foreign_functions.insert(key, func);
    }

    /// Get registered function info
    pub fn get_function(&self, module: &str, name: &str) -> Option<&ForeignFunction> {
        let key = format!("{}::{}", module, name);
        self.foreign_functions.get(&key)
    }

    /// Set sandbox configuration
    pub fn set_sandbox_config(&mut self, config: SandboxConfig) {
        self.sandbox_config = config;
    }

    /// Get sandbox configuration
    pub fn get_sandbox_config(&self) -> &SandboxConfig {
        &self.sandbox_config
    }

    /// Shutdown all runtimes
    pub fn shutdown_all(&mut self) -> Result<(), String> {
        for (lang, runtime) in self.runtimes.iter_mut() {
            runtime.shutdown()
                .map_err(|e| format!("Failed to shutdown {:?} runtime: {}", lang, e))?;
        }
        Ok(())
    }

    /// Get type marshaller
    pub fn get_marshaller(&self) -> Arc<Mutex<TypeMarshaller>> {
        Arc::clone(&self.marshaller)
    }

    /// List all loaded modules
    pub fn list_loaded_modules(&self) -> Vec<String> {
        self.loaded_modules.keys().cloned().collect()
    }

    /// List all registered functions
    pub fn list_registered_functions(&self) -> Vec<String> {
        self.foreign_functions.keys().cloned().collect()
    }

    /// Check if a language runtime is available
    pub fn is_runtime_available(&self, language: &ForeignLanguage) -> bool {
        self.runtimes.contains_key(language)
    }
}

/// Mock runtime for testing
pub struct MockRuntime {
    name: String,
    initialized: bool,
}

impl MockRuntime {
    pub fn new(name: &str) -> Self {
        MockRuntime {
            name: name.to_string(),
            initialized: false,
        }
    }
}

impl LanguageRuntime for MockRuntime {
    fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    fn execute_block(&mut self, _block: &LanguageBlock) -> Result<NuxValue, String> {
        Ok(NuxValue::String("Mock execution result".to_string()))
    }

    fn load_module(&mut self, module_path: &str) -> Result<usize, String> {
        Ok(module_path.len()) // Mock module ID
    }

    fn call_function(
        &mut self,
        _module_id: usize,
        function_name: &str,
        _args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        Ok(NuxValue::String(format!("Called {}", function_name)))
    }

    fn eval_expression(&mut self, expression: &str) -> Result<NuxValue, String> {
        Ok(NuxValue::String(format!("Evaluated: {}", expression)))
    }

    fn shutdown(&mut self) -> Result<(), String> {
        self.initialized = false;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_runtime() {
        let mut manager = FFIManager::new();
        let runtime = Box::new(MockRuntime::new("test"));
        let result = manager.register_runtime(ForeignLanguage::Python, runtime);
        assert!(result.is_ok());
    }

    #[test]
    fn test_initialize_runtime() {
        let mut manager = FFIManager::new();
        let runtime = Box::new(MockRuntime::new("test"));
        manager.register_runtime(ForeignLanguage::Python, runtime).unwrap();
        let result = manager.initialize_runtime(&ForeignLanguage::Python);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sandbox_config() {
        let mut manager = FFIManager::new();
        let config = SandboxConfig {
            level: SandboxLevel::Isolated,
            allowed_filesystem_paths: vec!["/tmp".to_string()],
            allow_network: true,
            allow_system_calls: false,
            max_memory_mb: 1024,
            max_execution_time_ms: 60000,
        };
        manager.set_sandbox_config(config.clone());
        assert_eq!(manager.get_sandbox_config().level, SandboxLevel::Isolated);
    }
}
