use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{FunctionValue, IntValue, PointerValue, BasicValueEnum};
use inkwell::types::BasicTypeEnum;
use inkwell::OptimizationLevel;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};

use crate::nvm::bytecode::{Opcode, BytecodeChunk, Value as NvmValue};
use crate::jit::{CompilationTier, CompiledFunction, JitError};

pub struct LlvmTranslator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> LlvmTranslator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Target::initialize_all(&InitializationConfig::default());
        let module = context.create_module("nux_jit_module");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            execution_engine,
        }
    }

    pub fn translate(&self, function_id: usize, chunk: &BytecodeChunk) -> Result<CompiledFunction, JitError> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let fn_name = format!("nux_func_{}", function_id);
        let function = self.module.add_function(&fn_name, fn_type, None);

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // Simple stack simulation (up to 128 items for JIT POC)
        let stack_array_type = i64_type.array_type(128);
        let stack_ptr = self.builder.build_alloca(stack_array_type, "stack").unwrap();
        
        let mut stack_top = 0; // Compile-time track of stack height

        let mut offset = 0;
        while offset < chunk.code.len() {
            let opcode_byte = chunk.code[offset];
            let opcode = Opcode::from_u8(opcode_byte).ok_or_else(|| {
                JitError::CompilationFailed(format!("Unknown opcode: {}", opcode_byte))
            })?;

            match opcode {
                Opcode::LOAD_CONST => {
                    let const_idx = chunk.code[offset + 1] as usize;
                    let val = &chunk.constants[const_idx];
                    let llvm_val = match val {
                        NvmValue::Int(i) => i64_type.const_int(*i as u64, true),
                        _ => return Err(JitError::UnsupportedOpcode(Opcode::LOAD_CONST)),
                    };
                    
                    let i32_type = self.context.i32_type();
                    let ptr = unsafe {
                        self.builder.build_gep(
                            i64_type, 
                            stack_ptr, 
                            &[i32_type.const_int(0, false), i32_type.const_int(stack_top as u64, false)], 
                            "push_ptr"
                        ).unwrap()
                    };
                    self.builder.build_store(ptr, llvm_val).unwrap();
                    stack_top += 1;
                    offset += 2;
                }
                Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV => {
                    stack_top -= 1;
                    let i32_type = self.context.i32_type();
                    let b_ptr = unsafe { self.builder.build_gep(i64_type, stack_ptr, &[i32_type.const_int(0, false), i32_type.const_int(stack_top as u64, false)], "b_ptr").unwrap() };
                    let b = self.builder.build_load(i64_type, b_ptr, "b").unwrap().into_int_value();

                    stack_top -= 1;
                    let a_ptr = unsafe { self.builder.build_gep(i64_type, stack_ptr, &[i32_type.const_int(0, false), i32_type.const_int(stack_top as u64, false)], "a_ptr").unwrap() };
                    let a = self.builder.build_load(i64_type, a_ptr, "a").unwrap().into_int_value();

                    let res = match opcode {
                        Opcode::ADD => self.builder.build_int_add(a, b, "add_res").unwrap(),
                        Opcode::SUB => self.builder.build_int_sub(a, b, "sub_res").unwrap(),
                        Opcode::MUL => self.builder.build_int_mul(a, b, "mul_res").unwrap(),
                        Opcode::DIV => self.builder.build_int_signed_div(a, b, "div_res").unwrap(),
                        _ => unreachable!(),
                    };
                    
                    let out_ptr = unsafe { self.builder.build_gep(i64_type, stack_ptr, &[i32_type.const_int(0, false), i32_type.const_int(stack_top as u64, false)], "out_ptr").unwrap() };
                    self.builder.build_store(out_ptr, res).unwrap();
                    stack_top += 1;
                    offset += 1;
                }
                Opcode::RETURN => {
                    stack_top -= 1;
                    let i32_type = self.context.i32_type();
                    let ret_ptr = unsafe { self.builder.build_gep(i64_type, stack_ptr, &[i32_type.const_int(0, false), i32_type.const_int(stack_top as u64, false)], "ret_ptr").unwrap() };
                    let ret_val = self.builder.build_load(i64_type, ret_ptr, "ret_val").unwrap().into_int_value();
                    self.builder.build_return(Some(&ret_val)).unwrap();
                    offset += 1;
                }
                _ => return Err(JitError::UnsupportedOpcode(opcode)),
            }
        }

        if !self.module.verify().is_ok() {
            return Err(JitError::CompilationFailed("LLVM Module verification failed".to_string()));
        }

        Ok(CompiledFunction {
            function_id,
            tier: CompilationTier::OptimizingJIT,
            native_code: vec![],
            entry_point: 0,
        })
    }

    pub fn execute(&self, function_id: usize) -> Result<i64, JitError> {
        let fn_name = format!("nux_func_{}", function_id);
        unsafe {
            let func: JitFunction<unsafe extern "C" fn() -> i64> = self.execution_engine
                .get_function(&fn_name)
                .map_err(|_| JitError::CompilationFailed("Could not find function".to_string()))?;
            Ok(func.call())
        }
    }

    pub fn compile_aot(&self, output_path: &std::path::Path) -> Result<(), JitError> {
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).map_err(|e| JitError::CompilationFailed(e.to_string()))?;
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| JitError::CompilationFailed("Could not create target machine".to_string()))?;

        target_machine
            .write_to_file(&self.module, FileType::Object, output_path)
            .map_err(|e| JitError::CompilationFailed(e.to_string()))?;

        Ok(())
    }
}
