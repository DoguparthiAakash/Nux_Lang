use nux::{compile_to_asm, assemble};
use nux::vm::NuxVm;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Nux Compiler v0.1.0");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  nux <file.nux>                  - Compile to assembly and display");
        eprintln!("  nux compile <file.nux> <out>    - Compile to bytecode file");
        eprintln!("  nux run <file.nux>              - Compile and Run");
        process::exit(1);
    }
    
    let mode = if args[1] == "compile" {
        "compile"
    } else if args[1] == "run" {
        "run"
    } else {
        "asm"
    };
    
    let input_file = if mode == "asm" { &args[1] } else { &args[2] };
    
    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    if mode == "asm" {
        println!("Compiling {}...", input_file);
    }
    
    // Compile to assembly
    let asm = match compile_to_asm(&source) {
        Ok(assembly) => assembly,
        Err(errors) => {
            eprintln!("\nCompilation errors:");
            for err in errors {
                eprintln!("  {}", err);
            }
            process::exit(1);
        }
    };
    
    if mode == "asm" {
        println!("\n--- Generated Assembly ---");
        println!("{}", asm);
    } else {
        // Assemble to bytecode
        let bytecode = match assemble(&asm) {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("\nAssembly error: {}", e);
                process::exit(1);
            }
        };

        if mode == "compile" {
            if args.len() < 4 {
                 eprintln!("Output file required for compile mode");
                 process::exit(1);
            }
            let output_file = &args[3];
            match fs::write(output_file, bytecode) {
                Ok(_) => println!("\n✓ Compiled to {}", output_file),
                Err(e) => {
                    eprintln!("\nError writing output: {}", e);
                    process::exit(1);
                }
            }
        } else if mode == "run" {
            // Run VM
            let mut vm = NuxVm::new(bytecode);
            vm.run();
        }
    }
}
