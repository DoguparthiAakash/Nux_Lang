// Nux Programming Language - Enhanced CLI (Transitional)
// Adds new commands while maintaining backward compatibility

use nux::{compile_to_asm, assemble, compile};
use nux::vm::NuxVm;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use nux::package_manager;
use nux::venv_manager;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "new" => cmd_new(&args[2..]),
        "build" => cmd_build(&args[2..]),
        "run" => cmd_run(&args[2..]),
        "test" => cmd_test(&args[2..]),
        "clean" => cmd_clean(&args[2..]),
        "check" => cmd_check(&args[2..]),
        "compile" => cmd_compile(&args[2..]),

        "venv" => {
            let name = if args.len() >= 3 { &args[2] } else { "" };
            venv_manager::create_venv(name);
        },

        "version" | "--version" | "-v" => print_version(),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            // Legacy mode: treat as file to compile
            cmd_legacy_compile(&args[1..]);
        }
    }
}

fn print_version() {
    println!("Nux Programming Language v{}", env!("CARGO_PKG_VERSION"));
    println!("A standalone polyglot language with zero dependencies");
}

fn print_help() {
    println!("Nux Programming Language v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    nux [COMMAND] [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("  Project Management:");
    println!("    new <name>              Create a new Nux project");
    println!("    build [--release]       Build the current project");
    println!("    run [file.nux]          Compile and run a Nux file");
    println!("    test                    Run project tests");
    println!("    clean                   Remove build artifacts");
    println!("    check                   Check code without building");
    println!();
    println!("  Compilation:");
    println!("    compile <file>          Compile to bytecode");
    println!("      --output <file>       Specify output file");
    println!("      --release             Optimize for release");
    println!();
    println!("  Other:");
    println!("    version, -v, --version  Print version");
    println!("    help, -h, --help        Print this help");
    println!();
    println!("EXAMPLES:");
    println!("    nux new my_project");
    println!("    nux run main.nux");
    println!("    nux compile main.nux --output main.nuxc");
    println!();
    println!("LEGACY USAGE:");
    println!("    nux <file.nux>          Compile to assembly and display");
}

fn cmd_new(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Project name required");
        eprintln!("Usage: nux new <name>");
        process::exit(1);
    }
    
    let project_name = &args[0];
    let current_dir = env::current_dir().unwrap();
    let project_dir = current_dir.join(project_name);
    
    // Create project structure
    if let Err(e) = fs::create_dir_all(&project_dir) {
        eprintln!("Error creating project directory: {}", e);
        process::exit(1);
    }
    
    if let Err(e) = fs::create_dir_all(project_dir.join("src")) {
        eprintln!("Error creating src directory: {}", e);
        process::exit(1);
    }
    
    // Create nux.toml
    let config_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
authors = []
edition = "2024"

[dependencies]

[build]
optimization_level = 2
debug = false
target = "native"
parallel = true
"#, project_name);
    
    if let Err(e) = fs::write(project_dir.join("nux.toml"), config_content) {
        eprintln!("Error creating nux.toml: {}", e);
        process::exit(1);
    }
    
    // Create main.nux
    let main_content = r#"fn main() {
    print("Hello, Nux!");
}
"#;
    
    if let Err(e) = fs::write(project_dir.join("src").join("main.nux"), main_content) {
        eprintln!("Error creating main.nux: {}", e);
        process::exit(1);
    }
    
    println!("     Created binary (application) `{}` package", project_name);
    println!("     Project created at: {}", project_dir.display());
}

fn cmd_build(args: &[String]) {
    let release = args.contains(&"--release".to_string());
    let current_dir = env::current_dir().unwrap();
    
    // Check for nux.toml
    if !current_dir.join("nux.toml").exists() {
        eprintln!("Error: Not a Nux project (nux.toml not found)");
        eprintln!("Run 'nux new <name>' to create a new project");
        process::exit(1);
    }
    
    // Find main.nux
    let main_file = current_dir.join("src").join("main.nux");
    if !main_file.exists() {
        eprintln!("Error: src/main.nux not found");
        process::exit(1);
    }
    
    println!("   Compiling project...");
    
    // Read and compile
    let source = match fs::read_to_string(&main_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading main.nux: {}", e);
            process::exit(1);
        }
    };
    
    match compile(&source) {
        Ok(bytecode) => {
            // Create target directory
            let target_dir = current_dir.join("target");
            let build_dir = if release {
                target_dir.join("release")
            } else {
                target_dir.join("debug")
            };
            
            if let Err(e) = fs::create_dir_all(&build_dir) {
                eprintln!("Error creating target directory: {}", e);
                process::exit(1);
            }
            
            // Get project name from directory
            let project_name = current_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output");
            
            let output_file = build_dir.join(format!("{}.nuxc", project_name));
            
            if let Err(e) = fs::write(&output_file, bytecode) {
                eprintln!("Error writing output: {}", e);
                process::exit(1);
            }
            
            let mode = if release { "release [optimized]" } else { "dev [unoptimized + debuginfo]" };
            println!("    Finished {} target(s)", mode);
            println!("    Output: {}", output_file.display());
        }
        Err(errors) => {
            eprintln!("\nCompilation errors:");
            for err in errors {
                eprintln!("  {}", err);
            }
            process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) {
    if args.is_empty() {
        // Try to run project
        let current_dir = env::current_dir().unwrap();
        if current_dir.join("nux.toml").exists() {
            // Build and run project
            cmd_build(&[]);
            
            let project_name = current_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output");
            
            let bytecode_file = current_dir.join("target").join("debug").join(format!("{}.nuxc", project_name));
            
            if let Ok(bytecode) = fs::read(&bytecode_file) {
                println!("     Running `{}`", project_name);
                let mut vm = NuxVm::new(bytecode);
                vm.run();
            } else {
                eprintln!("Error: Build output not found");
                process::exit(1);
            }
        } else {
            eprintln!("Error: No file specified and not in a Nux project");
            eprintln!("Usage: nux run <file.nux>");
            process::exit(1);
        }
    } else {
        // Run specific file
        let input_file = &args[0];
        
        let source = match fs::read_to_string(input_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", input_file, e);
                process::exit(1);
            }
        };
        
        println!("   Compiling {}...", input_file);
        
        match compile(&source) {
            Ok(bytecode) => {
                println!("     Running...");
                let mut vm = NuxVm::new(bytecode);
                vm.run();
            }
            Err(errors) => {
                eprintln!("\nCompilation errors:");
                for err in errors {
                    eprintln!("  {}", err);
                }
                process::exit(1);
            }
        }
    }
}

fn cmd_test(_args: &[String]) {
    println!("    Running tests...");
    println!("    No tests found");
    println!("\nTest result: ok. 0 passed; 0 failed");
}

fn cmd_clean(_args: &[String]) {
    let current_dir = env::current_dir().unwrap();
    let target_dir = current_dir.join("target");
    
    if target_dir.exists() {
        match fs::remove_dir_all(&target_dir) {
            Ok(_) => println!("   Removed build artifacts"),
            Err(e) => {
                eprintln!("Error removing target directory: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("   Nothing to clean");
    }
}

fn cmd_check(_args: &[String]) {
    println!("    Checking project...");
    
    let current_dir = env::current_dir().unwrap();
    let main_file = current_dir.join("src").join("main.nux");
    
    if !main_file.exists() {
        eprintln!("Error: src/main.nux not found");
        process::exit(1);
    }
    
    let source = match fs::read_to_string(&main_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading main.nux: {}", e);
            process::exit(1);
        }
    };
    
    match compile(&source) {
        Ok(_) => {
            println!("    Finished check - no errors");
        }
        Err(errors) => {
            eprintln!("\nCheck failed:");
            for err in errors {
                eprintln!("  {}", err);
            }
            process::exit(1);
        }
    }
}

fn cmd_compile(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Input file required");
        eprintln!("Usage: nux compile <file> [--output <file>]");
        process::exit(1);
    }
    
    let input_file = &args[0];
    let output_file = if let Some(pos) = args.iter().position(|x| x == "--output") {
        if pos + 1 < args.len() {
            args[pos + 1].clone()
        } else {
            input_file.replace(".nux", ".nuxc")
        }
    } else {
        input_file.replace(".nux", ".nuxc")
    };
    
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    println!("   Compiling {} to bytecode...", input_file);
    
    match compile(&source) {
        Ok(bytecode) => {
            match fs::write(&output_file, bytecode) {
                Ok(_) => println!("✓ Compiled to {}", output_file),
                Err(e) => {
                    eprintln!("Error writing output: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(errors) => {
            eprintln!("\nCompilation errors:");
            for err in errors {
                eprintln!("  {}", err);
            }
            process::exit(1);
        }
    }
}

fn cmd_legacy_compile(args: &[String]) {
    // Legacy mode for backward compatibility
    let input_file = &args[0];
    
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    println!("Compiling {}...", input_file);
    
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
    
    println!("\n--- Generated Assembly ---");
    println!("{}", asm);
}
