mod lexer;
mod compiler;
mod assembler;
mod vm;
mod native;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: nux <file.nux> [options]");
        eprintln!("       nux run <file.nux>");
        eprintln!("       nux compile <file.nux> <output.nuxi>");
        eprintln!("       nux native <file.nux> <output.o>");
        eprintln!("       nux llvm <file.nux> <output.o>");
        process::exit(1);
    }

    if args[1] == "native" {
        if args.len() < 3 {
            eprintln!("Error: Missing input file for native compilation");
            process::exit(1);
        }
        let input = Path::new(&args[2]);
        let output = if args.len() >= 4 {
            args[3].clone()
        } else {
            input.with_extension("o").to_string_lossy().to_string()
        };
        match native::compile_to_object(input, Path::new(&output)) {
            Ok(()) => println!("Native object written to {}", output),
            Err(error) => {
                eprintln!("Native compilation error: {}", error);
                process::exit(1);
            }
        }
        return;
    }

    if args[1] == "llvm" {
        if args.len() < 3 {
            eprintln!("Error: Missing input file for LLVM compilation");
            process::exit(1);
        }
        let input = Path::new(&args[2]);
        let output = if args.len() >= 4 {
            args[3].clone()
        } else {
            input.with_extension("o").to_string_lossy().to_string()
        };
        match native::compile_to_llvm_object(input, Path::new(&output)) {
            Ok(()) => println!("LLVM object written to {}", output),
            Err(error) => {
                eprintln!("LLVM compilation error: {}", error);
                process::exit(1);
            }
        }
        return;
    }
    
    let mode = if args.len() >= 3 && args[1] == "compile" {
        "compile"
    } else if args.len() >= 3 && args[1] == "run" {
        "run"
    } else if args[1].ends_with(".nux") {
        "run" // Implicit run for `nux file.nux`
    } else {
        "run"
    };
    
    // Adjust arg index
    let input_file = if args[1] == "compile" || args[1] == "run" { 
        if args.len() < 3 {
             eprintln!("Error: Missing input file");
             process::exit(1);
        }
        &args[2] 
    } else { 
        &args[1] 
    };
    
    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    // Compile high-level Nux to assembly
    // println!("Compiling {}...", input_file);
    let asm = match compiler::compile_to_asm_source(&source) {
        Ok(assembly) => assembly,
        Err(errors) => {
            eprintln!("Compilation errors:");
            for err in errors {
                eprintln!("  {}", err);
            }
            process::exit(1);
        }
    };
    
    // Assemble to bytecode
    let bytecode = match assembler::compile(&asm) {
        Ok(bc) => bc,
        Err(e) => {
                eprintln!("Assembly error: {}", e);
                println!("--- DEBUG: Generated ASM ---");
                println!("{}", asm);
                println!("----------------------------");
                process::exit(1);
        }
    };

    if mode == "compile" {
        let output_file = if args.len() >= 4 { &args[3] } else { "out.nuxi" };
        match fs::write(output_file, bytecode) {
            Ok(_) => println!("Compiled to {}", output_file),
            Err(e) => {
                eprintln!("Error writing output: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Run Mode
        // println!("Running...");
        let mut nux_vm = vm::NuxVm::new(bytecode);
        nux_vm.run();
        // println!("Done.");
    }
}
