mod compiler;
mod vm;
mod lexer;
mod high_level;
mod editor;
mod transpiler;
mod platform;
mod versioning;
mod micro_vm;
mod flasher;
mod jit_backend;
pub mod backends;
mod project;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Nux compiler – no banner by default (Rust-style, clean output)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "init" => {
            if args.len() < 3 {
                println!("Usage: nux init <project_name>");
                return;
            }
            if let Err(e) = project::init_project(&args[2]) {
                println!("Error initializing project: {}", e);
            }
        },
        "venv" => {
            if let Err(e) = project::create_venv() {
                println!("Error creating virtual environment: {}", e);
            }
        },
        "build" | "compile" => {
            let mut spath = String::new();
            if args.len() < 3 {
                // Check if nux.toml exists
                if let Some(config) = project::parse_nux_toml() {
                    // Try to build main.nux by default if it exists
                    if std::path::Path::new("main.nux").exists() {
                        spath = "main.nux".to_string();
                    } else if let Some(hw) = config.target_hardware {
                        if std::path::Path::new(&hw).exists() {
                            spath = hw;
                        } else {
                            println!("Usage: nux build <source.nux> [output.nuxi]");
                            return;
                        }
                    } else {
                        println!("Usage: nux build <source.nux> [output.nuxi]");
                        return;
                    }
                } else {
                    println!("Usage: nux build <source.nux> [output.nuxi]");
                    return;
                }
            } else {
                spath = args[2].clone();
            }
            let source_path = &spath;
            let output_path = if args.len() > 3 { &args[3] } else { "out.nuxi" };

            // Preprocess Imports
            let mut visited = std::collections::HashSet::new();
            let source = match process_imports(std::path::Path::new(source_path), &mut visited) {
                Ok(s) => s,
                Err(e) => {
                    println!("Error processing imports: {}", e);
                    return;
                }
            };
            
            eprintln!("\x1b[1;36m   Compiling\x1b[0m {}", source_path);

            // Try High Level First (heuristic: if contains "print(")
            // Or just try compile_high_level, if error, try asm.
            use lexer::Span;
            use high_level::CompileError;

            let result: Result<Vec<u8>, Vec<CompileError>> = if source.contains("print(") || source.contains(";") || source.contains("func ") {
                 // println!("Compiling as High-Level Nux...");
                 high_level::compile_high_level(&source)
            } else {
                 // println!("Compiling as NuxASM...");
                 compiler::compile(&source).map_err(|e| vec![CompileError { message: e, span: Span { line: 0, col: 0 } }])
            };

            match result {
                Ok(bytes) => {
                    match fs::File::create(output_path) {
                        Ok(mut f) => {
                            if let Err(e) = f.write_all(&bytes) {
                                eprintln!("\x1b[1;31merror\x1b[0m: could not write output file: {}", e);
                            } else {
                                eprintln!("\x1b[1;32m    Finished\x1b[0m -> \x1b[1m{}\x1b[0m", output_path);
                            }
                        },
                        Err(e) => eprintln!("\x1b[1;31merror\x1b[0m: could not create output file: {}", e),
                    }
                },
                Err(errors) => {
                    let err_count = errors.len();
                    for e in &errors {
                        eprintln!("\x1b[1;31merror\x1b[0m: {}", e.message);
                    }
                    eprintln!("\x1b[1;31merror\x1b[0m: could not compile `{}` due to {} previous error{}",
                        source_path, err_count, if err_count == 1 { "" } else { "s" });
                }
            }
        },
        "run" => {
            let mut use_jit = false;
            let mut file_path_idx = 2;
            
            if args.len() > 2 {
                if args[2] == "--jit" || args[2] == "-j" {
                    use_jit = true;
                    file_path_idx = 3;
                }
            }
            
            let mut spath = String::new();
            if args.len() <= file_path_idx {
                if let Some(config) = project::parse_nux_toml() {
                    if std::path::Path::new("main.nux").exists() {
                        spath = "main.nux".to_string();
                    } else if let Some(hw) = config.target_hardware {
                        if std::path::Path::new(&hw).exists() {
                            spath = hw;
                        } else {
                            println!("Usage: nux run [--jit] <file.nux or file.nuxi>");
                            return;
                        }
                    } else {
                        println!("Usage: nux run [--jit] <file.nux or file.nuxi>");
                        return;
                    }
                } else {
                    println!("Usage: nux run [--jit] <file.nux or file.nuxi>");
                    return;
                }
            } else {
                spath = args[file_path_idx].clone();
            }
            let path = &spath;
            
            // Case 1: Running a compiled binary directly (.nuxi)
            if path.ends_with(".nuxi") {
                match fs::read(path) {
                    Ok(bytes) => {
                        if use_jit {
                            jit_backend::execute_jit(&bytes, None);
                        } else {
                            let mut machine = vm::NuxVm::new(bytes);
                            
                            use platform::Platform;
                            let mut platform: Box<dyn Platform> = if cfg!(feature = "gui") {
                                Box::new(platform::desktop::DesktopPlatform::new())
                            } else {
                                Box::new(platform::headless::HeadlessPlatform::new())
                            };
                            
                            machine.run(Some(platform.as_mut()));
                        }
                    },
                    Err(e) => println!("Error reading binary file: {}", e),
                }
                return;
            }

            // Case 2: Running Source Code (.nux) -> Compile & Run with Versioning
            // Preprocess Imports
            let mut visited = std::collections::HashSet::new();
            let source_content = match process_imports(std::path::Path::new(path), &mut visited) {
                Ok(s) => s,
                Err(e) => {
                    println!("Error processing imports: {}", e);
                    return;
                }
            };

            // ── Nux TUI colors ──────────────────────────────────────────────────
            // Teal (Nux brand)    = \x1b[38;5;81m    Bold = \x1b[1;38;5;81m
            // Gold (accent)       = \x1b[38;5;220m
            // Red (issue)         = \x1b[38;5;203m
            // Green (success)     = \x1b[38;5;78m
            // Dim grey (gutter)   = \x1b[38;5;242m
            // Reset               = \x1b[0m
            // ────────────────────────────────────────────────────────────────────

            let teal   = "\x1b[1;38;5;81m";
            let gold   = "\x1b[38;5;220m";
            let red    = "\x1b[1;38;5;203m";
            let green  = "\x1b[1;38;5;78m";
            let grey   = "\x1b[38;5;242m";
            let bold   = "\x1b[1m";
            let reset  = "\x1b[0m";

            // ── Header bar ──────────────────────────────────────────────────────
            let fname = std::path::Path::new(path)
                .file_name().map(|f| f.to_str().unwrap_or(path)).unwrap_or(path);
            eprintln!("{grey}╭─ {teal}◆ nux{grey} ─────────────────────────────────{reset}");
            eprintln!("{grey}│{reset}  {bold}{}{reset}  {grey}·  building ...{reset}", fname);
            eprintln!("{grey}╰────────────────────────────────────────{reset}");

            // ── Compile ─────────────────────────────────────────────────────────
            use lexer::Span;
            use high_level::CompileError;

            let result: Result<Vec<u8>, Vec<CompileError>> = if source_content.contains("print(") || source_content.contains(";") || source_content.contains("func ") || source_content.contains("fun ") || source_content.contains("fn ") {
                high_level::compile_high_level(&source_content)
            } else {
                compiler::compile(&source_content).map_err(|e| vec![CompileError { message: e, span: Span { line: 0, col: 0 } }])
            };

            let compiled_bytes = match result {
                Ok(b) => b,
                Err(errors) => {
                    let err_count = errors.len();
                    let lines: Vec<&str> = source_content.lines().collect();

                    eprintln!();
                    eprintln!("{}╳ nux found {} issue{} in {}{}{}",
                        red, err_count,
                        if err_count == 1 { "" } else { "s" },
                        bold, fname, reset);
                    eprintln!();

                    for (idx, e) in errors.iter().enumerate() {
                        // Numbered issue header
                        eprintln!("{}  issue #{}{}{}{} · {}{}{}",
                            grey, reset,
                            gold, idx + 1, reset,
                            grey, e.message, reset);

                        let mut line_num = e.span.line;
                        let mut col_num  = e.span.col;

                        if line_num > lines.len() {
                            line_num = lines.len();
                            if line_num > 0 { col_num = lines[line_num - 1].len() + 1; }
                        }

                        if line_num > 0 {
                            let line_str = lines[line_num - 1];
                            // Box-drawing source frame
                            eprintln!("{}  ┌─ {}{}:{}{}", grey, reset, path, line_num, if col_num > 0 { format!(":{}", col_num) } else { String::new() });
                            eprintln!("{}  │{reset}", grey);
                            eprintln!("{}  │{reset} {}{:>3}{reset}{grey} ┊{reset}  {}",
                                grey, grey, line_num, line_str);

                            // Caret squiggle under the token
                            eprint!("{}  │{reset}      ", grey);
                            for _ in 0..(col_num.saturating_sub(1)) { eprint!(" "); }
                            eprintln!("{}^{reset}", red);
                            eprintln!("{}  └─{reset}", grey);
                        }
                        eprintln!();
                    }

                    // Footer summary
                    eprintln!("{}  ✗ build stopped — {} issue{} must be resolved{}",
                        red, err_count, if err_count == 1 { "" } else { "s" }, reset);
                    eprintln!();
                    return;
                }
            };

            // 2. Check for 'bin' and '-fis' flags
            let save_bin = args.iter().any(|arg| arg == "bin");
            let use_fis = args.iter().any(|arg| arg == "-fis");

            // 3. Save binary based on flags
            if save_bin {
                let path_obj = std::path::Path::new(path);
                let stem = path_obj.file_stem().unwrap().to_str().unwrap();
                let binary_name = format!("{}.nuxi", stem);
                
                let dir_path = if let Some(cache_path) = project::get_venv_cache_path() {
                    cache_path
                } else {
                    let dir_name = format!("{}_nux", stem);
                    let dp = std::path::PathBuf::from(&dir_name);
                    if let Err(e) = fs::create_dir_all(&dp) {
                        eprintln!("{}  ✗ could not create directory: {}{}", red, e, reset);
                        return;
                    }
                    dp
                };

                let binary_path = dir_path.join(&binary_name);

                if use_fis {
                    let versioning = versioning::BinaryVersioning::new(path_obj, 5);
                    match versioning.save_version(&compiled_bytes) {
                        Ok(saved_path) => {
                            eprintln!("{grey}├─ {green}✦ compiled{reset}  {} {grey}→{reset} {bold}{}{reset}",
                                fname, saved_path.display());
                        }
                        Err(e) => {
                            eprintln!("{}  ✗ could not save versioned binary: {}{}", red, e, reset);
                            return;
                        }
                    }
                } else {
                    if let Err(e) = fs::write(&binary_path, &compiled_bytes) {
                        eprintln!("{}  ✗ could not write binary: {}{}", red, e, reset);
                        return;
                    }
                    eprintln!("{grey}├─ {green}✦ compiled{reset}  {} {grey}→{reset} {bold}{}{reset}",
                        fname, binary_path.display());
                }
            } else {
                eprintln!("{grey}├─ {green}✦ compiled{reset}  {}  {grey}(memory only){reset}", fname);
            }

            // 4. Run
            eprintln!("{grey}╰─ {teal}▶ running{reset}  {bold}{}{reset}", fname);
            eprintln!();

            if use_jit {
                jit_backend::execute_jit(&compiled_bytes, None);
            } else {
                let mut machine = vm::NuxVm::new(compiled_bytes);
    
                use platform::Platform;
                let mut platform: Box<dyn Platform> = if cfg!(feature = "gui") {
                    Box::new(platform::desktop::DesktopPlatform::new())
                } else {
                    Box::new(platform::headless::HeadlessPlatform::new())
                };
    
                machine.run(Some(platform.as_mut()));
            }
        },
        "edit" => {
             if args.len() < 3 {
                println!("Usage: nux edit <file>");
                return;
             }
             editor::run(&args[2]);
        },
        "translate" => {
             if args.len() < 3 {
                println!("Usage: nux translate <source.nux> [--target c|asm] [--profile std|embedded|extreme|nano|legacy]");
                return;
             }
             let source_path = &args[2];
             
             // Parse Flags
             let mut target_str = "c";
             let mut profile_str = "std";
             
             let mut i = 3;
             while i < args.len() {
                 match args[i].as_str() {
                     "--target" => {
                         if i+1 < args.len() { target_str = &args[i+1]; i+=1; }
                     },
                     "--profile" => {
                         if i+1 < args.len() { profile_str = &args[i+1]; i+=1; }
                     },
                     _ => {}
                 }
                 i += 1;
             }
             
             let profile = match profile_str {
                 "embedded" => transpiler::TranspileProfile::Embedded,
                 "extreme" => transpiler::TranspileProfile::Extreme,
                 "nano" => transpiler::TranspileProfile::Nano,
                 "legacy" => transpiler::TranspileProfile::Legacy,
                 _ => transpiler::TranspileProfile::Standard,
             };
             
             // Preprocess Imports
             let mut visited = std::collections::HashSet::new();
             let source = match process_imports(std::path::Path::new(source_path), &mut visited) {
                Ok(s) => s,
                Err(e) => { println!("Error processing imports: {}", e); return; }
             };
             
             match target_str {
                 "c" => {
                     println!("Compiling to ASM...");
                     let asm_source = match high_level::compile_to_asm_source(&source) {
                        Ok(s) => s,
                        Err(errors) => { 
                            println!("Compilation Failed with {} errors:", errors.len()); 
                            for e in errors { println!("Error: {}", e.message); }
                            return; 
                        }
                     };
                     let c_code = transpiler::transpile_to_c(&asm_source, &profile);
                     let out_name = format!("{}.c", source_path);
                     if let Err(e) = fs::write(&out_name, c_code) {
                         println!("Error writing output: {}", e);
                     } else {
                         println!("✅ Generated C source: {}", out_name);
                     }
                 },
                 "asm" => {
                     println!("Compiling to ASM...");
                     let asm_source = match high_level::compile_to_asm_source(&source) {
                        Ok(s) => s,
                        Err(errors) => { 
                            println!("Compilation Failed with {} errors:", errors.len()); 
                            for e in errors { println!("Error: {}", e.message); }
                            return; 
                        }
                     };
                     let out_name = format!("{}.nux.asm", source_path);
                     if let Err(e) = fs::write(&out_name, asm_source) {
                         println!("Error writing output: {}", e);
                     } else {
                         println!("✅ Generated NuxASM source: {}", out_name);
                     }
                 },
                 "micro" => {
                     println!("Compiling to Nux Bytecode...");
                     let compiled_bytes = match high_level::compile_high_level(&source) {
                        Ok(b) => b,
                        Err(errors) => {
                            println!("Compilation Failed with {} errors:", errors.len()); 
                            for e in errors { println!("Error: {}", e.message); }
                            return; 
                        }
                     };
                     
                     println!("Generating Micro VM C Code...");
                     let c_code = micro_vm::generate_micro_vm(&compiled_bytes);
                     let out_name = format!("{}.micro.c", source_path);
                     if let Err(e) = fs::write(&out_name, c_code) {
                         println!("Error writing output: {}", e);
                     } else {
                         println!("✅ Generated Micro VM C source: {}", out_name);
                         println!("   You can now compile this file using any standard C compiler.");
                     }
                 },
                 _ => println!("Unknown target: {}. Supported: c, asm, micro", target_str),
             }
        },
        "build-native" => {
            if args.len() < 3 {
                println!("Usage: nux build-native <source.nux> [output_binary] [--profile std|embedded]");
                return;
            }
            let source_path = &args[2];
            let output_path = if args.len() > 3 && !args[3].starts_with("-") { &args[3] } else { "a.out" };

             let mut profile = transpiler::TranspileProfile::Standard;
             
             if args.contains(&"--profile".to_string()) {
                 let idx = args.iter().position(|r| r == "--profile").unwrap();
                 if idx + 1 < args.len() {
                     profile = match args[idx+1].as_str() {
                         "embedded" => transpiler::TranspileProfile::Embedded,
                         "extreme" => transpiler::TranspileProfile::Extreme,
                         _ => transpiler::TranspileProfile::Standard,
                     };
                 }
             }

            // Preprocess Imports
            let mut visited = std::collections::HashSet::new();
            let source = match process_imports(std::path::Path::new(source_path), &mut visited) {
               Ok(s) => s,
               Err(e) => { println!("Error processing imports: {}", e); return; }
            };
            
            println!("Transpiling to ASM...");
            let asm_source = match high_level::compile_to_asm_source(&source) {
                Ok(s) => s,
                Err(errors) => { 
                    println!("High-Level Compilation Failed with {} errors:", errors.len()); 
                    for e in errors {
                        println!("Error at line {}: {}", e.span.line, e.message);
                    }
                    return; 
                }
            };
            
            println!("Transpiling to C and Compiling...");
            let config = transpiler::TranspilerConfig {
                target: transpiler::TranspileTarget::C,
                profile: profile,
            };
            
            match transpiler::transpile_and_compile(&asm_source, output_path, &config) {
                Ok(_) => println!("✅ Build Process Completed."),
                Err(e) => println!("❌ Build Failed: {}", e),
            }
        },
        "version" => {
            println!("Nux SDK v0.4.0-portable (JIT & Threads Enabled)");
        },
        "update" => {
            println!("Checking for updates...");
            // 1. Git Pull
            let output = std::process::Command::new("git")
                .arg("pull")
                .output();
                
            match output {
                Ok(o) => {
                    if o.status.success() {
                        println!("Git Pull Successful.");
                        println!("{}", String::from_utf8_lossy(&o.stdout));
                        
                        // 2. Rebuild
                        println!("Rebuilding Nux...");
                        let build = std::process::Command::new("cargo")
                            .arg("build")
                            .arg("--release")
                            .output();
                            
                        match build {
                            Ok(b) => {
                                if b.status.success() {
                                    println!("Build Successful.");
                                    
                                    // 3. Install
                                    let src = "target/release/nux";
                                    let dest = "/home/aakash/.local/bin/nux"; // Hardcoded for this env or use current_exe?
                                    // Better: use std::env::current_exe();
                                    
                                    if let Ok(exe) = std::env::current_exe() {
                                        // We are running 'nux'. If we overwrite ourselves while running, Linux allows it (usually unlinks).
                                        // But safer to copy to `dest` if known.
                                        match fs::copy(src, dest) {
                                            Ok(_) => println!("✅ Updated nux binary at {}", dest),
                                            Err(e) => println!("Failed to install binary: {}", e),
                                        }
                                    } else {
                                        println!("Could not determine current executable path.");
                                    }
                                } else {
                                    println!("❌ Build Failed:\n{}", String::from_utf8_lossy(&b.stderr));
                                }
                            },
                            Err(e) => println!("Failed to run cargo: {}", e),
                        }
                    } else {
                        println!("❌ Git Pull Failed:\n{}", String::from_utf8_lossy(&o.stderr));
                    }
                },
                Err(e) => println!("Failed to execute git: {}", e),
            }
        },
        "build-micro" => {
            if args.len() < 3 {
                println!("Usage: nux build-micro <source.nux> [output.micro.c]");
                return;
            }
            let source_path = &args[2];
            let output_path = if args.len() > 3 { &args[3] } else { "out.micro.c" };

            // Preprocess Imports for CUG
            let mut visited = std::collections::HashSet::new();
            let source = match process_imports(std::path::Path::new(source_path), &mut visited) {
                Ok(s) => s,
                Err(e) => { println!("Error processing imports: {}", e); return; }
            };
            
            // Auto-detect High-Level vs ASM
            let result = if source.contains("print(") || source.contains(";") || source.contains("func ") {
                high_level::compile_high_level(&source)
            } else {
                compiler::compile(&source).map_err(|e| vec![high_level::CompileError{message: e, span: lexer::Span{line:0,col:0}}])
            };

            match result {
                Ok(bytes) => {
                    let c_code = micro_vm::generate_micro_vm(&bytes);
                    match fs::write(output_path, c_code) {
                        Ok(_) => println!("✅ Generated Micro VM C file at {}", output_path),
                        Err(e) => println!("❌ Failed to write C file: {}", e),
                    }
                }
                Err(errors) => {
                    for e in errors {
                        println!("❌ Compilation failed: {}", e);
                    }
                }
            }
        },
        "deploy" => {
            if args.len() < 5 {
                println!("Usage: nux deploy <file.nux> --arch <esp32|riscv|avr> --port <COM3>");
                return;
            }
            let source_path = &args[2];
            let mut arch = "";
            let mut port = "";
            
            for i in 3..args.len() {
                if args[i] == "--arch" && i + 1 < args.len() {
                    arch = &args[i + 1];
                }
                if args[i] == "--port" && i + 1 < args.len() {
                    port = &args[i + 1];
                }
            }
            
            if arch.is_empty() || port.is_empty() {
                println!("Missing --arch or --port arguments.");
                return;
            }
            
            println!("Compiling {} for {}...", source_path, arch);
            // In full implementation, we'd invoke the native emitter here
            let dummy_payload = vec![0x00, 0x01, 0x02];
            
            if let Err(e) = flasher::deploy_to_port(port, &dummy_payload) {
                println!("Deploy Failed: {}", e);
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Nux Language Portable SDK");
    println!("Usage:");
    println!("  nux deploy <source.nux> --arch <arch> --port <port> - Compile to native machine code & flash over serial");
    println!("  nux build <source.nux> [output.nuxi]  - Compile (Auto-detects HighLevel/ASM)");
    println!("  nux build-native <src> [out] [--profile] - Compile direct to machine code via C transpiler");
    println!("  nux run   <source.nux>                - Compile & run (no binary saved)");
    println!("  nux run   <source.nux> bin            - Compile, save binary & run");
    println!("  nux run   <source.nux> bin -fis       - Compile, save with versioning & run");
    println!("  nux run   <binary.nuxi>               - Run a pre-compiled binary");
    println!("  nux edit  <file>                      - Open IDE/Editor");
    println!("  nux update                            - Update Nux from GitHub");
    println!("  nux version                           - Show version");
    println!("");
    println!("Flags:");
    println!("  bin     Save compiled binary to <filename>_nux/<filename>.nuxi");
    println!("  -fis    Force Incremental Save - saves binaries with version control");
    println!("          (keeps last 5 versions as v1, v2, v3, etc.)");
}

// Simple Import Preprocessor
// Recursively reads files and replaces `import "file";` with content.
// Detects cycles.
fn process_imports(path: &std::path::Path, visited: &mut std::collections::HashSet<std::path::PathBuf>) -> Result<String, String> {
    let canonical_path = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(_) => path.to_path_buf(), // Fallback if can't canonicalize (e.g. std/math.nux if searching path?)
    };
    
    // Simple verification for now: if path doesn't exist, try looking in lib/ relative to binary or cwd.
    // But fs::read_to_string checks existence.
    
    if visited.contains(&canonical_path) {
        // Circular dependency or already imported. 
        // We can just return empty string to avoid duplication?
        // If imported A -> B, and C -> B, B should be included ONCE if we want single-definition.
        // Nux functions can be redefined? No.
        // So yes, return empty if already visited.
        return Ok(String::new());
    }
    
    visited.insert(canonical_path.clone());
    
    let content = fs::read_to_string(path).map_err(|e| format!("Could not read file {:?}: {}", path, e))?;
    
    let mut out = String::new();
    
    for line in content.lines() {
        let mut trimmed = line.trim();
        
        // Handle .nuxg syntax remapping
        if trimmed.starts_with("@hardware") {
            continue; // Skip hardware tag for now
        }
        
        if trimmed.starts_with("register ") {
            // register NAME ADDRESS -> const NAME = ADDRESS;
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() == 3 {
                let name = parts[1];
                let addr = parts[2];
                out.push_str(&format!("const {} = {};\n", name, addr));
            }
            continue;
        }

        let is_import = trimmed.starts_with("import ") && trimmed.ends_with(";");
        let is_link = trimmed.starts_with("link ") && trimmed.ends_with("\"");
        
        if is_import || is_link {
            let start = trimmed.find('"').ok_or("Invalid import syntax")? + 1;
            let end = trimmed.rfind('"').ok_or("Invalid import syntax")?;
            let raw_import = &trimmed[start..end];
            
            let import_path_str = if raw_import.ends_with(".nux") || raw_import.ends_with(".nuxel") || raw_import.ends_with(".nuxg") {
                raw_import.to_string()
            } else {
                raw_import.replace(".", "/")
            };
            
            // Determine which extensions to check
            let mut extensions_to_try = vec![];
            if import_path_str.ends_with(".nux") || import_path_str.ends_with(".nuxel") || import_path_str.ends_with(".nuxg") {
                extensions_to_try.push(import_path_str.clone());
            } else {
                extensions_to_try.push(format!("{}.nuxel", import_path_str)); // CUG lib priority
                extensions_to_try.push(format!("{}.nuxg", import_path_str));  // Hardware definitions
                extensions_to_try.push(format!("{}.nux", import_path_str));   // Standard fallback
            }
            
            // Search Paths
            let mut resolved_path = None;
            
            for ext in &extensions_to_try {
                // -1. Check VENV
                if let Some(venv_lib) = project::get_venv_lib_path() {
                    let p_venv = venv_lib.join(ext);
                    if p_venv.exists() { resolved_path = Some(p_venv); break; }
                }

                // 0. Environment Variable NUX_LIB
                if let Ok(nux_lib_env) = std::env::var("NUX_LIB") {
                    let p_env = std::path::Path::new(&nux_lib_env).join(ext);
                    if p_env.exists() { resolved_path = Some(p_env); break; }
                }
                
                // 1. Relative to current file
                let parent = path.parent().unwrap_or(std::path::Path::new("."));
                let p1 = parent.join(ext);
                if p1.exists() { resolved_path = Some(p1); break; }
                
                // 2. Relative to current file's 'lib' folder (Project Lib)
                let p2 = parent.join("lib").join(ext);
                if p2.exists() { resolved_path = Some(p2); break; }

                // 3. Current Working Directory (CWD)
                if let Ok(cwd) = std::env::current_dir() {
                    // 3a. CWD Root
                    let p3a = cwd.join(ext);
                    if p3a.exists() { resolved_path = Some(p3a); break; }
                    
                    // 3b. CWD Lib
                    let p3b = cwd.join("lib").join(ext);
                    if p3b.exists() { resolved_path = Some(p3b); break; }
                    
                    // 3c. Nux Source Tree Dev Fallback (nux_portable/lib)
                    let p3c = cwd.join("nux_portable").join("lib").join(ext);
                    if p3c.exists() { resolved_path = Some(p3c); break; }
                }
                
                // 4. System/Executable Libs
                if let Ok(exe_path) = std::env::current_exe() {
                    if let Some(exe_dir) = exe_path.parent() {
                        let p4 = exe_dir.join(ext);
                        if p4.exists() { resolved_path = Some(p4); break; }
                        
                        let p5 = exe_dir.join("lib").join(ext);
                        if p5.exists() { resolved_path = Some(p5); break; }
                    }
                }
            }
            
            if resolved_path.is_none() {
                 // Try one last thing: Maybe the user provided "graphics2d" and it is in "lib/graphics2d.nux"
                 // but we only checked "lib/graphics2d.nux" (correct).
                 // What if it is in "std"?
                 // Standard library usually in "lib".
                 return Err(format!("Import not found: {}", import_path_str));
            }
            
            let import_path = resolved_path.unwrap();
            
            let result = process_imports(&import_path, visited)?;
            out.push_str(&result);
            out.push('\n');
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    
    Ok(out)
}
