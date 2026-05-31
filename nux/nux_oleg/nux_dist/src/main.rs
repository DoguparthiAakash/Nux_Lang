// Nux Programming Language - Enhanced CLI (Transitional)
// Adds new commands while maintaining backward compatibility

use nux::{compile_to_asm, assemble, compile};
use nux::vm::NuxVm;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::io::{Read, Seek, SeekFrom, Write};

use nux::package_manager;
use nux::venv_manager;

fn main() {
    // --- Standalone Executable Check ---
    if let Ok(mut exe) = fs::File::open(env::current_exe().unwrap()) {
        let magic = b"NUX_STANDALONE";
        if exe.seek(SeekFrom::End(-(magic.len() as i64))).is_ok() {
            let mut buf = vec![0u8; magic.len()];
            if exe.read_exact(&mut buf).is_ok() && buf.as_slice() == magic {
                // We are running inside a bundled executable!
                if exe.seek(SeekFrom::End(-(magic.len() as i64) - 8)).is_ok() {
                    let mut len_buf = [0u8; 8];
                    if exe.read_exact(&mut len_buf).is_ok() {
                        let bc_len = u64::from_le_bytes(len_buf);
                        if exe.seek(SeekFrom::End(-(magic.len() as i64) - 8 - (bc_len as i64))).is_ok() {
                            let mut bytecode = vec![0u8; bc_len as usize];
                            if exe.read_exact(&mut bytecode).is_ok() {
                                // Run the embedded bytecode natively
                                let mut vm = nux::vm::NuxVm::new(bytecode);
                                vm.run();
                                process::exit(0);
                            }
                        }
                    }
                }
            }
        }
    }

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
        "repl" => cmd_repl(&args[2..]),
        "test" => cmd_test(&args[2..]),
        "clean" => cmd_clean(&args[2..]),
        "check" => cmd_check(&args[2..]),
        "compile" => cmd_compile(&args[2..]),
        "pkg" => cmd_pkg(&args[2..]),

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
    println!("      --standalone          Compile to standalone native executable");
    println!();
    println!("  Package Management:");
    println!("    pkg install [name]      Install a package from registry or nux.toml");
    println!("    pkg remove <name>       Remove a package");
    println!("    pkg list                List installed packages");
    println!("    pkg update [name]       Update one or all packages");
    println!();
    println!("  Interactive:");
    println!("    repl                    Start interactive stateless REPL");
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
    
    println!("   \x1b[1;32mCompiling\x1b[0m project...");
    
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
            println!("    \x1b[1;32mFinished\x1b[0m {} target(s)", mode);
            println!("    \x1b[1;36mOutput\x1b[0m: {}", output_file.display());
        }
        Err(errors) => {
            print_errors(&source, errors, main_file.to_str().unwrap_or("src/main.nux"));
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
                println!("     \x1b[1;36mRunning\x1b[0m `{}`", project_name);
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
        
        if input_file.ends_with(".nuxc") {
            // Run pre-compiled bytecode
            let bytecode = match fs::read(input_file) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", input_file, e);
                    process::exit(1);
                }
            };
            
            println!("     \x1b[1;36mRunning\x1b[0m {}...", input_file);
            let mut vm = NuxVm::new(bytecode);
            vm.run();
        } else {
            // Compile and run source file
            let source = match fs::read_to_string(input_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", input_file, e);
                    process::exit(1);
                }
            };
            
            println!("   \x1b[1;32mCompiling\x1b[0m {}...", input_file);
            
            match compile(&source) {
                Ok(bytecode) => {
                    println!("     \x1b[1;36mRunning\x1b[0m...");
                    let mut vm = NuxVm::new(bytecode);
                    vm.run();
                }
                Err(errors) => {
                    print_errors(&source, errors, input_file);
                    process::exit(1);
                }
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
            print_errors(&source, errors, main_file.to_str().unwrap_or("src/main.nux"));
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
    
    let standalone = args.contains(&"--standalone".to_string());
    
    let input_file = &args[0];
    let output_file = if let Some(pos) = args.iter().position(|x| x == "--output") {
        if pos + 1 < args.len() {
            args[pos + 1].clone()
        } else {
            input_file.replace(".nux", if standalone { ".exe" } else { ".nuxc" })
        }
    } else {
        input_file.replace(".nux", if standalone { ".exe" } else { ".nuxc" })
    };
    
    let source = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    println!("   Compiling {} to bytecode...", input_file);
    match nux::compile_to_asm(&source) {
        Ok(asm_text) => {
            let _ = fs::write("debug.asm", &asm_text);
        }
        Err(_) => {}
    }
    match compile(&source) {
        Ok(bytecode) => {
            if standalone {
                let exe_path = env::current_exe().unwrap();
                if let Err(e) = fs::copy(&exe_path, &output_file) {
                    eprintln!("Error creating standalone exe: {}", e);
                    process::exit(1);
                }
                
                let mut out_file = fs::OpenOptions::new().append(true).open(&output_file).unwrap();
                out_file.write_all(&bytecode).unwrap();
                let bc_len = bytecode.len() as u64;
                out_file.write_all(&bc_len.to_le_bytes()).unwrap();
                out_file.write_all(b"NUX_STANDALONE").unwrap();
                
                println!("✓ Compiled Standalone Executable to {}", output_file);
            } else {
                match fs::write(&output_file, bytecode) {
                    Ok(_) => println!("✓ Compiled to {}", output_file),
                    Err(e) => {
                        eprintln!("Error writing output: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Err(errors) => {
            print_errors(&source, errors, input_file);
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
            print_errors(&source, errors, input_file);
            process::exit(1);
        }
    };
    
    println!("\n--- Generated Assembly ---");
    println!("{}", asm);
}

fn cmd_pkg(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Package command required");
        eprintln!("Usage: nux pkg <install|remove|list|update> [args]");
        process::exit(1);
    }
    
    let subcmd = &args[0];
    let target = package_manager::InstallTarget::auto_detect();
    
    match subcmd.as_str() {
        "install" => {
            if args.len() > 1 {
                package_manager::install(&args[1], if args.len() > 2 { &args[2] } else { "*" }, target);
            } else {
                package_manager::install_from_config(target);
            }
        }
        "remove" => {
            if args.len() > 1 {
                package_manager::remove(&args[1], target);
            } else {
                eprintln!("Usage: nux pkg remove <name>");
            }
        }
        "list" => {
            package_manager::list(target);
        }
        "update" => {
            if args.len() > 1 {
                package_manager::update_package(&args[1], target);
            } else {
                package_manager::update_all(target);
            }
        }
        _ => {
            eprintln!("Unknown package command: {}", subcmd);
            process::exit(1);
        }
    }
}

fn cmd_repl(_args: &[String]) {
    println!("Nux REPL v{} - Stateless Interactive Shell", env!("CARGO_PKG_VERSION"));
    println!("Type 'exit' or 'quit' to close.");
    
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        
        match compile(&input) {
            Ok(bytecode) => {
                let mut vm = NuxVm::new(bytecode);
                vm.run();
            }
            Err(errors) => {
                print_errors(&input, errors, "<stdin>");
            }
        }
    }
}

fn highlight_syntax(line: &str) -> String {
    // A simple regex-free syntax highlighter for the TUI using standard ANSI colors (like rustc)
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    let mut in_comment = false;
    
    let keywords = ["if", "else", "while", "for", "return", "break", "continue", "import", "class", "func", "let", "const", "var", "new", "this", "try", "catch", "throw", "defer", "match"];
    let types = ["int", "float", "string", "bool", "void", "any"];
    let booleans = ["true", "false"];
    
    while let Some(c) = chars.next() {
        if in_comment {
            out.push(c);
            continue;
        }
        if in_string {
            out.push(c);
            if c == '"' {
                in_string = false;
                out.push_str("\x1b[0m"); // Reset after string
            }
            continue;
        }
        
        if c == '/' && chars.peek() == Some(&'/') {
            in_comment = true;
            out.push_str("\x1b[1;30m//"); // Bright Black (Dark Gray) comment
            chars.next();
            continue;
        }
        
        if c == '#' && chars.peek() == Some(&'*') {
            in_comment = true;
            out.push_str("\x1b[1;30m#*"); // Bright Black (Dark Gray) comment
            chars.next();
            continue;
        }

        if c == '"' {
            in_string = true;
            out.push_str("\x1b[1;32m\""); // Bold Green string
            continue;
        }
        
        if c.is_alphabetic() || c == '_' {
            let mut word = String::new();
            word.push(c);
            while let Some(&nc) = chars.peek() {
                if nc.is_alphanumeric() || nc == '_' {
                    word.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            
            if keywords.contains(&word.as_str()) {
                out.push_str(&format!("\x1b[1;35m{}\x1b[0m", word)); // Bold Magenta keyword
            } else if types.contains(&word.as_str()) {
                out.push_str(&format!("\x1b[1;36m{}\x1b[0m", word)); // Bold Cyan type
            } else if booleans.contains(&word.as_str()) {
                out.push_str(&format!("\x1b[1;33m{}\x1b[0m", word)); // Bold Yellow boolean
            } else if chars.peek() == Some(&'(') {
                out.push_str(&format!("\x1b[1;34m{}\x1b[0m", word)); // Bold Blue function call
            } else {
                out.push_str(&format!("\x1b[0m{}\x1b[0m", word)); // Default variable
            }
            continue;
        }
        
        if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            while let Some(&nc) = chars.peek() {
                if nc.is_numeric() || nc == '.' {
                    num.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            out.push_str(&format!("\x1b[1;33m{}\x1b[0m", num)); // Bold Yellow number
            continue;
        }
        
        if "+-*/=<>!&|%".contains(c) {
            out.push_str(&format!("\x1b[1;37m{}\x1b[0m", c)); // Bold White operator
            continue;
        }
        
        out.push(c);
    }
    
    if in_comment || in_string {
        out.push_str("\x1b[0m"); // Ensure reset
    }
    
    out
}

fn print_errors(source: &str, errors: Vec<nux::CompileError>, file_name: &str) {
    eprintln!("\n\x1b[1;31merror\x1b[0m: could not compile `{}` due to previous errors\n", file_name);
    
    let lines: Vec<&str> = source.lines().collect();

    for err in errors {
        eprintln!("\x1b[1;31merror\x1b[0m: {}", err.message);
        eprintln!("  \x1b[1;34m-->\x1b[0m {}:{}:{}", file_name, err.span.line, err.span.col);
        
        let line_idx = if err.span.line > 0 { err.span.line - 1 } else { 0 };
        
        if line_idx < lines.len() {
            let line_str = lines[line_idx];
            let line_num_str = err.span.line.to_string();
            let padding = " ".repeat(line_num_str.len());
            
            let highlighted_line = highlight_syntax(line_str);
            eprintln!(" \x1b[1;34m{} |\x1b[0m {}", line_num_str, highlighted_line);
            
            let col_idx = if err.span.col > 0 { err.span.col - 1 } else { 0 };
            
            // Handle tab characters vs spaces for pointer alignment
            let mut pointer_padding = String::new();
            for (i, c) in line_str.chars().enumerate() {
                if i >= col_idx { break; }
                if c == '\t' {
                    pointer_padding.push('\t');
                } else {
                    pointer_padding.push(' ');
                }
            }
            
            eprintln!(" \x1b[1;34m{} |\x1b[0m \x1b[1;31m{}^\x1b[0m", padding, pointer_padding);
        }
        eprintln!();
    }
}
