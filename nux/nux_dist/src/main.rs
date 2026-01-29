use nux::{compile_to_asm, assemble};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Nux Compiler v0.1.0");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  nux <file.nux>              - Compile to assembly and display");
        eprintln!("  nux compile <file.nux> <output.nuxi>  - Compile to bytecode");
        process::exit(1);
    }
    
    let mode = if args.len() >= 3 && args[1] == "compile" {
        "compile"
    } else {
        "asm"
    };
    
    let input_file = if mode == "compile" { &args[2] } else { &args[1] };
    
    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    println!("Compiling {}...", input_file);
    
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
    
    if mode == "compile" {
        // Assemble to bytecode
        let bytecode = match assemble(&asm) {
            Ok(bc) => bc,
            Err(e) => {
                eprintln!("\nAssembly error: {}", e);
                process::exit(1);
            }
        };
        
        let output_file = &args[3];
        match fs::write(output_file, bytecode) {
            Ok(_) => println!("\n✓ Compiled to {}", output_file),
            Err(e) => {
                eprintln!("\nError writing output: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("\n✓ Compilation successful!");
        println!("\n--- Generated Assembly ---");
        println!("{}", asm);
    }
}
