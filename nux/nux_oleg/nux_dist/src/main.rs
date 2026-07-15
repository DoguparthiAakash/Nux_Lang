// Nux Programming Language - Enhanced CLI (Transitional)
// Adds new commands while maintaining backward compatibility

use nux::{compile_to_asm, assemble, compile};
use nux::vm::NuxVm;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::io::{Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

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
        "build-ext" => cmd_build_ext(&args[2..]),
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
    let logo_color1 = (0, 200, 255);
    let logo_color2 = (150, 50, 255);
    
    println!();
    println!("  {} {} {}", 
        "-".truecolor(logo_color1.0, logo_color1.1, logo_color1.2).bold(),
        "Nux".truecolor(logo_color2.0, logo_color2.1, logo_color2.2).bold(),
        env!("CARGO_PKG_VERSION").truecolor(150, 150, 150)
    );
    println!("  {}", "A High-Performance AI Programming Language".truecolor(100, 100, 100).italic());
}

fn print_help() {
    print_version();
    println!();
    
    let box_color = (60, 60, 70);
    let category_color = (255, 100, 150);
    let cmd_color = (0, 200, 255);
    let desc_color = (180, 180, 180);
    
    println!("  {}", "o Usage".truecolor(255, 255, 255).bold());
    println!("  {} {} {}
", "- ".truecolor(box_color.0, box_color.1, box_color.2), "nux".truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(), "<command> [args]".truecolor(100, 100, 100));
    
    println!("  {}", "o Commands".truecolor(255, 255, 255).bold());
    
    let cmds = vec![
        ("o  Project", vec![
            ("new <name>", "Create a new Nux workspace"),
            ("build", "Compile project to bytecode"),
            ("run [file]", "Execute a script or project"),
            ("test", "Run test suite"),
            ("clean", "Remove build artifacts"),
        ]),
        ("o  Ecosystem", vec![
            ("pkg <cmd>", "Manage packages (install, remove, list)"),
            ("venv <cmd>", "Manage isolated virtual environments"),
        ]),
        ("o  Advanced", vec![
            ("compile <file>", "Compile to .nuxc executable"),
            ("build-ext <f>", "Compile .cux native extension"),
            ("repl", "Start the interactive console"),
        ]),
    ];
    
    for (i, (category, group)) in cmds.iter().enumerate() {
        println!("  {} {}", "-".truecolor(box_color.0, box_color.1, box_color.2), category.truecolor(category_color.0, category_color.1, category_color.2).bold());
        for (j, (cmd, desc)) in group.iter().enumerate() {
            let is_last_group = i == cmds.len() - 1;
            let prefix = if is_last_group { " " } else { "" };
            let sub_prefix = if j == group.len() - 1 { "-" } else { "-" };
            println!("  {}   {} {:<16} {}", 
                prefix.truecolor(box_color.0, box_color.1, box_color.2),
                sub_prefix.truecolor(box_color.0, box_color.1, box_color.2),
                cmd.truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(),
                desc.truecolor(desc_color.0, desc_color.1, desc_color.2)
            );
        }
        if i != cmds.len() - 1 {
            println!("  {}", "".truecolor(box_color.0, box_color.1, box_color.2));
        }
    }
    println!();
}
fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.bright_magenta} {msg}").unwrap());
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
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
    
    if !current_dir.join("nux.toml").exists() {
        eprintln!("\n  {} Not a Nux project. Run {} to initialize.", "✕".red().bold(), "nux new".cyan());
        process::exit(1);
    }
    
    let main_file = current_dir.join("src").join("main.nux");
    if !main_file.exists() {
        eprintln!("\n  {} Source file {} missing.", "✕".red().bold(), "src/main.nux".cyan());
        process::exit(1);
    }
    
    let pb = create_spinner("Compiling project...");
    
    let source = fs::read_to_string(&main_file).unwrap_or_else(|_| {
        pb.finish_and_clear();
        eprintln!("  {} Failed to read main.nux", "✕".red().bold());
        process::exit(1);
    });
    
    match compile(&source) {
        Ok(bytecode) => {
            let target_dir = current_dir.join("target");
            let build_dir = if release {
                target_dir.join("release")
            } else {
                target_dir.join("debug")
            };
            
            fs::create_dir_all(&build_dir).unwrap();
            
            let project_name = current_dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output");
            
            let output_file = build_dir.join(format!("{}.nuxc", project_name));
            fs::write(&output_file, bytecode).unwrap();
            
            pb.finish_and_clear();
            let mode = if release { "release" } else { "debug" };
            println!("  {} {} [{}] → {}", "✔".green().bold(), project_name.white().bold(), mode.dimmed(), output_file.display().to_string().cyan());
        }
        Err(errors) => {
            pb.finish_and_clear();
            print_errors(&source, errors, main_file.to_str().unwrap_or("src/main.nux"));
            process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) {
    if args.is_empty() {
        let current_dir = env::current_dir().unwrap();
        if current_dir.join("nux.toml").exists() {
            cmd_build(&[]);
            let project_name = current_dir.file_name().and_then(|n| n.to_str()).unwrap_or("output");
            let bytecode_file = current_dir.join("target").join("debug").join(format!("{}.nuxc", project_name));
            
            if let Ok(bytecode) = fs::read(&bytecode_file) {
                println!("  {} {}\n", "▶".bright_magenta(), project_name.white().bold());
                let mut vm = NuxVm::new(bytecode);
                vm.run();
            } else {
                eprintln!("  {} Build output not found", "✕".red().bold());
                process::exit(1);
            }
        } else {
            eprintln!("  {} No file specified", "✕".red().bold());
            process::exit(1);
        }
    } else {
        let input_file = &args[0];
        
        if input_file.ends_with(".nuxc") {
            let bytecode = fs::read(input_file).unwrap();
            println!("  {} {}\n", "▶".bright_magenta(), input_file.white().bold());
            let mut vm = NuxVm::new(bytecode);
            vm.run();
        } else {
            let source = fs::read_to_string(input_file).unwrap();
            
            let pb = create_spinner(&format!("Compiling {}...", input_file));
            
            match compile(&source) {
                Ok(bytecode) => {
                    pb.finish_and_clear();
                    println!("  {} {}\n", "▶".bright_magenta(), input_file.white().bold());
                    let mut vm = NuxVm::new(bytecode);
                    vm.run();
                }
                Err(errors) => {
                    pb.finish_and_clear();
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
        eprintln!("  {} Input file required", "✕".red().bold());
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
            eprintln!("  {} Error reading file '{}': {}", "✕".red().bold(), input_file, e);
            process::exit(1);
        }
    };
    
    let pb = create_spinner(&format!("Compiling {}...", input_file));
    let _ = nux::compile_to_asm(&source).map(|asm| fs::write("debug.asm", asm));
    
    match compile(&source) {
        Ok(bytecode) => {
            if standalone {
                let exe_path = env::current_exe().unwrap();
                fs::copy(&exe_path, &output_file).unwrap();
                
                let mut out_file = fs::OpenOptions::new().append(true).open(&output_file).unwrap();
                out_file.write_all(&bytecode).unwrap();
                let bc_len = bytecode.len() as u64;
                out_file.write_all(&bc_len.to_le_bytes()).unwrap();
                out_file.write_all(b"NUX_STANDALONE").unwrap();
                
                pb.finish_and_clear();
                println!("  {} Standalone Executable: {}", "✔".green().bold(), output_file.cyan());
            } else {
                fs::write(&output_file, bytecode).unwrap();
                pb.finish_and_clear();
                println!("  {} Compiled: {}", "✔".green().bold(), output_file.cyan());
            }
        }
        Err(errors) => {
            pb.finish_and_clear();
            print_errors(&source, errors, input_file);
            process::exit(1);
        }
    }
}

fn cmd_build_ext(args: &[String]) {
    if args.is_empty() {
        eprintln!("  {} Input .cux file required", "✕".red().bold());
        process::exit(1);
    }
    
    let input_file = &args[0];
    let pb = create_spinner(&format!("Building CUX extension: {}...", input_file));
    
    match nux::cux::compile_cux(input_file) {
        Ok(out_path) => {
            pb.finish_and_clear();
            println!("  {} Extension compiled successfully: {}", "✔".green().bold(), out_path.display().to_string().cyan());
        }
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("  {} Failed to compile extension: {}", "✕".red().bold(), e);
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
    let repl_path = "repl.nux";
    if std::path::Path::new(repl_path).exists() {
        let code = std::fs::read_to_string(repl_path).unwrap();
        match compile(&code) {
            Ok(bytecode) => {
                let mut vm = NuxVm::new(bytecode);
                vm.run();
            }
            Err(errors) => {
                print_errors(&code, errors, repl_path);
            }
        }
        return;
    }

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
    println!("\n  {} Compilation aborted due to {} error(s)\n", "◆".truecolor(255, 50, 50).bold(), errors.len().to_string().truecolor(255, 255, 255).bold());
    
    let lines: Vec<&str> = source.lines().collect();

    for err in errors {
        println!("  {} {}", "Error:".truecolor(255, 50, 50).bold(), err.message.truecolor(220, 220, 220).bold());
        println!("  {} {}:{}:{}\n", "╰─►".truecolor(100, 100, 100), file_name.truecolor(0, 255, 255), err.span.line, err.span.col);
        
        let line_idx = if err.span.line > 0 { err.span.line - 1 } else { 0 };
        
        if line_idx < lines.len() {
            let line_str = lines[line_idx];
            let line_num_str = format!("{:>4}", err.span.line);
            
            let highlighted_line = highlight_syntax(line_str);
            println!(" {} │ {}", line_num_str.truecolor(80, 80, 80), highlighted_line);
            
            let col_idx = if err.span.col > 0 { err.span.col - 1 } else { 0 };
            
            let mut pointer_padding = String::new();
            for (i, c) in line_str.chars().enumerate() {
                if i >= col_idx { break; }
                if c == '\t' {
                    pointer_padding.push('\t');
                } else {
                    pointer_padding.push(' ');
                }
            }
            
            println!("      │ {}{}", pointer_padding, "▲".truecolor(255, 50, 50).bold());
        }
        println!();
    }
}
