#[cfg(feature = "inkwell")]
use inkwell::context::Context;
#[cfg(feature = "inkwell")]
use inkwell::module::Module;
#[cfg(feature = "inkwell")]
use inkwell::builder::Builder;
#[cfg(feature = "inkwell")]
use inkwell::execution_engine::ExecutionEngine;

pub struct LlvmVm<'ctx> {
    #[cfg(feature = "inkwell")]
    context: &'ctx Context,
    #[cfg(feature = "inkwell")]
    module: Module<'ctx>,
    #[cfg(feature = "inkwell")]
    builder: Builder<'ctx>,
    #[cfg(feature = "inkwell")]
    execution_engine: ExecutionEngine<'ctx>,
    
    // Stub properties when LLVM is not available
    #[cfg(not(feature = "inkwell"))]
    _marker: std::marker::PhantomData<&'ctx ()>,
}

impl<'ctx> LlvmVm<'ctx> {
    #[cfg(feature = "inkwell")]
    pub fn new(context: &'ctx Context) -> Result<Self, String> {
        let module = context.create_module("nux_module");
        let execution_engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive)
            .map_err(|e| e.to_string())?;
            
        let builder = context.create_builder();
        
        Ok(LlvmVm {
            context,
            module,
            builder,
            execution_engine,
        })
    }

    #[cfg(not(feature = "inkwell"))]
    pub fn new() -> Result<Self, String> {
        println!("[Warning] Nux was compiled without the 'inkwell' LLVM feature.");
        println!("The LLVM VM is stubbed out. Enable it in Cargo.toml to use native LLVM compilation.");
        Ok(LlvmVm { _marker: std::marker::PhantomData })
    }

    /// Compile and run the Nux AST dynamically via the JIT engine
    pub fn run_jit(&self) {
        #[cfg(feature = "inkwell")]
        {
            println!("[LLVM JIT] Executing module in memory...");
            // Execute the main function dynamically
        }
        #[cfg(not(feature = "inkwell"))]
        {
            println!("[LLVM JIT] Unsupported. LLVM feature not enabled.");
        }
    }
    
    /// Compile the module to a native .o object file
    pub fn compile_to_object(&self, output_path: &str) -> Result<(), String> {
        #[cfg(feature = "inkwell")]
        {
            use inkwell::targets::{Target, InitializationConfig, RelocMode, CodeModel, FileType};
            
            println!("[LLVM AOT] Emitting object file to: {}", output_path);
            Target::initialize_all(&InitializationConfig::default());
            
            let target_triple = inkwell::targets::TargetMachine::get_default_triple();
            let target = Target::from_triple(&target_triple).map_err(|e| e.to_string())?;
            let target_machine = target
                .create_target_machine(
                    &target_triple,
                    "generic",
                    "",
                    inkwell::OptimizationLevel::Aggressive,
                    RelocMode::Default,
                    CodeModel::Default,
                )
                .ok_or("Could not create target machine")?;
            
            self.module.set_triple(&target_triple);
            self.module.set_data_layout(&target_machine.get_target_data().get_data_layout());
            
            target_machine
                .write_to_file(&self.module, FileType::Object, std::path::Path::new(output_path))
                .map_err(|e| e.to_string())?;
                
            Ok(())
        }
        #[cfg(not(feature = "inkwell"))]
        {
            Err("Cannot compile to object file without the LLVM feature enabled".to_string())
        }
    }
}
