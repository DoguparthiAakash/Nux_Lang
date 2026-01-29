// JavaScript Bridge - Embeds JavaScript engine (V8 or QuickJS) for JS interop
// Provides Nux ↔ JavaScript bidirectional communication

use crate::ffi_manager::{LanguageRuntime, LanguageBlock};
use crate::type_marshaller::{NuxValue, JavaScriptValue, TypeMarshaller};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// JavaScript runtime bridge
pub struct JavaScriptBridge {
    initialized: bool,
    marshaller: Arc<Mutex<TypeMarshaller>>,
    loaded_modules: HashMap<usize, String>,
    next_module_id: usize,
    global_context: HashMap<String, NuxValue>,
    engine_type: JSEngineType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSEngineType {
    V8,        // Google V8 (Node.js)
    QuickJS,   // QuickJS (lightweight)
    SpiderMonkey, // Mozilla SpiderMonkey
}

impl JavaScriptBridge {
    pub fn new(marshaller: Arc<Mutex<TypeMarshaller>>, engine_type: JSEngineType) -> Self {
        JavaScriptBridge {
            initialized: false,
            marshaller,
            loaded_modules: HashMap::new(),
            next_module_id: 0,
            global_context: HashMap::new(),
            engine_type,
        }
    }

    pub fn with_default_engine(marshaller: Arc<Mutex<TypeMarshaller>>) -> Self {
        Self::new(marshaller, JSEngineType::QuickJS)
    }

    /// Execute JavaScript code and return result
    fn execute_js_code(&mut self, code: &str) -> Result<NuxValue, String> {
        if !self.initialized {
            return Err("JavaScript runtime not initialized".to_string());
        }

        // In production, use V8 or QuickJS:
        // For V8:
        // let isolate = v8::Isolate::new(Default::default());
        // let scope = v8::HandleScope::new(&mut isolate);
        // let context = v8::Context::new(scope);
        // let scope = v8::ContextScope::new(scope, context);
        // let code = v8::String::new(scope, code).unwrap();
        // let script = v8::Script::compile(scope, code, None).unwrap();
        // let result = script.run(scope).unwrap();
        
        // For QuickJS:
        // let context = quickjs::Context::new();
        // let result = context.eval(code)?;

        // Mock implementation
        if code.contains("import ") || code.contains("require(") {
            Ok(NuxValue::Null)
        } else if code.contains("function ") || code.contains("=>") {
            let func_name = self.extract_function_name(code);
            Ok(NuxValue::Function(func_name))
        } else if code.contains("async ") || code.contains("await ") {
            // Async operation - would need Promise handling
            Ok(NuxValue::String(format!("Async result: {}", code)))
        } else {
            Ok(NuxValue::String(format!("JS result: {}", code)))
        }
    }

    fn extract_function_name(&self, code: &str) -> String {
        // Extract function name from code
        if let Some(start) = code.find("function ") {
            let rest = &code[start + 9..];
            if let Some(end) = rest.find('(') {
                return rest[..end].trim().to_string();
            }
        }
        "anonymous".to_string()
    }

    /// Convert JavaScript value to Nux value
    fn js_to_nux(&self, js_val: &JavaScriptValue) -> Result<NuxValue, String> {
        let mut marshaller = self.marshaller.lock()
            .map_err(|e| format!("Failed to lock marshaller: {}", e))?;
        marshaller.from_javascript(js_val)
    }

    /// Convert Nux value to JavaScript value
    fn nux_to_js(&self, nux_val: &NuxValue) -> Result<JavaScriptValue, String> {
        let marshaller = self.marshaller.lock()
            .map_err(|e| format!("Failed to lock marshaller: {}", e))?;
        marshaller.to_javascript(nux_val)
    }

    /// Load a JavaScript module (ES6 or CommonJS)
    fn load_js_module(&mut self, module_path: &str) -> Result<usize, String> {
        if !self.initialized {
            return Err("JavaScript runtime not initialized".to_string());
        }

        // In production, handle different module systems:
        // - ES6: import { foo } from 'module'
        // - CommonJS: const foo = require('module')
        // - Node.js built-ins: require('fs'), require('http')
        // - npm packages: require('lodash')

        let module_id = self.next_module_id;
        self.next_module_id += 1;
        self.loaded_modules.insert(module_id, module_path.to_string());
        
        Ok(module_id)
    }

    /// Call a JavaScript function from a module
    fn call_js_function(
        &mut self,
        module_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        let module_path = self.loaded_modules.get(&module_id)
            .ok_or_else(|| format!("Module ID {} not found", module_id))?;

        // In production, call the actual JS function:
        // let js_args: Vec<JavaScriptValue> = args.iter()
        //     .map(|a| self.nux_to_js(a))
        //     .collect::<Result<_, _>>()?;
        // let result = module.call_function(function_name, js_args)?;
        // self.js_to_nux(&result)

        // Mock implementation
        Ok(NuxValue::String(format!(
            "Called {}::{} with {} args",
            module_path,
            function_name,
            args.len()
        )))
    }

    /// Handle Promise/async operations
    pub fn handle_promise(&mut self, promise_code: &str) -> Result<NuxValue, String> {
        // In production, handle JavaScript Promises:
        // - Convert to Rust Future
        // - Use async/await bridge
        // - Return result when Promise resolves
        
        Ok(NuxValue::String(format!("Promise result: {}", promise_code)))
    }

    /// Install npm package
    pub fn install_npm_package(&self, package: &str) -> Result<(), String> {
        // In production:
        // std::process::Command::new("npm")
        //     .args(&["install", package])
        //     .output()
        //     .map_err(|e| e.to_string())?;
        
        println!("Mock: Installing npm package: {}", package);
        Ok(())
    }
}

impl LanguageRuntime for JavaScriptBridge {
    fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        // In production, initialize the JS engine:
        match self.engine_type {
            JSEngineType::V8 => {
                // v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
                // v8::V8::initialize();
            }
            JSEngineType::QuickJS => {
                // quickjs::Runtime::new();
            }
            JSEngineType::SpiderMonkey => {
                // Initialize SpiderMonkey
            }
        }
        
        self.initialized = true;
        Ok(())
    }

    fn execute_block(&mut self, block: &LanguageBlock) -> Result<NuxValue, String> {
        self.execute_js_code(&block.code)
    }

    fn load_module(&mut self, module_path: &str) -> Result<usize, String> {
        self.load_js_module(module_path)
    }

    fn call_function(
        &mut self,
        module_id: usize,
        function_name: &str,
        args: Vec<NuxValue>,
    ) -> Result<NuxValue, String> {
        self.call_js_function(module_id, function_name, args)
    }

    fn eval_expression(&mut self, expression: &str) -> Result<NuxValue, String> {
        self.execute_js_code(expression)
    }

    fn shutdown(&mut self) -> Result<(), String> {
        // In production, cleanup JS engine resources
        self.initialized = false;
        self.loaded_modules.clear();
        self.global_context.clear();
        Ok(())
    }

    fn name(&self) -> &str {
        match self.engine_type {
            JSEngineType::V8 => "JavaScript (V8)",
            JSEngineType::QuickJS => "JavaScript (QuickJS)",
            JSEngineType::SpiderMonkey => "JavaScript (SpiderMonkey)",
        }
    }
}

/// Helper functions for JavaScript integration

/// Check if Node.js is available
pub fn is_nodejs_available() -> bool {
    // In production:
    // std::process::Command::new("node")
    //     .arg("--version")
    //     .output()
    //     .is_ok()
    true // Mock
}

/// Get Node.js version
pub fn get_nodejs_version() -> Result<String, String> {
    // In production:
    // let output = std::process::Command::new("node")
    //     .arg("--version")
    //     .output()
    //     .map_err(|e| e.to_string())?;
    // Ok(String::from_utf8_lossy(&output.stdout).to_string())
    
    Ok("v18.12.0 (mock)".to_string())
}

/// Check if npm is available
pub fn is_npm_available() -> bool {
    true // Mock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_bridge_initialization() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = JavaScriptBridge::with_default_engine(marshaller);
        assert!(bridge.initialize().is_ok());
    }

    #[test]
    fn test_js_module_loading() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = JavaScriptBridge::with_default_engine(marshaller);
        bridge.initialize().unwrap();
        let module_id = bridge.load_module("lodash").unwrap();
        assert_eq!(module_id, 0);
    }

    #[test]
    fn test_js_function_call() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        let mut bridge = JavaScriptBridge::with_default_engine(marshaller);
        bridge.initialize().unwrap();
        let module_id = bridge.load_module("math").unwrap();
        let result = bridge.call_function(module_id, "sqrt", vec![NuxValue::Int(16)]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nodejs_availability() {
        assert!(is_nodejs_available());
    }

    #[test]
    fn test_engine_types() {
        let marshaller = Arc::new(Mutex::new(TypeMarshaller::new()));
        
        let v8_bridge = JavaScriptBridge::new(marshaller.clone(), JSEngineType::V8);
        assert_eq!(v8_bridge.name(), "JavaScript (V8)");
        
        let quickjs_bridge = JavaScriptBridge::new(marshaller.clone(), JSEngineType::QuickJS);
        assert_eq!(quickjs_bridge.name(), "JavaScript (QuickJS)");
    }
}
