mod compiler;
mod vm;
mod lexer;
mod high_level;
mod editor;
mod transpiler;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    println!("NUX COMPILER v0.2 (Float Support)");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "build" | "compile" => {
            if args.len() < 3 {
                println!("Usage: nux build <source.nux> [output.nuxi]");
                return;
            }
            let source_path = &args[2];
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
            
            // Debug: Dump preprocessed source
            // println!("--- Preprocessed Source ---\n{}\n---------------------------", source);

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
                                println!("Error writing output file: {}", e);
                            } else {
                                println!("Successfully compiled to {}", output_path);
                            }
                        },
                        Err(e) => println!("Error creating output file: {}", e),
                    }
                },
                Err(errors) => {
                    println!("❌ Compilation Failed with {} errors:", errors.len());
                    for e in errors {
                        println!("Error: {}", e.message);
                    }
                }
            }
        },
        "run" => {
            if args.len() < 3 {
                println!("Usage: nux run <file.nux or file.nuxi>");
                return;
            }
            let path = &args[2];
            
            // Case 1: Running a compiled binary directly (.nuxi)
            if path.ends_with(".nuxi") {
                match fs::read(path) {
                    Ok(bytes) => {
                        let mut machine = vm::NuxVm::new(bytes);
                        machine.run();
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

            // 1. Compile (Check for Errors)
            println!("Compiling {}...", path);
            use lexer::Span;
            use high_level::CompileError;

            let result: Result<Vec<u8>, Vec<CompileError>> = if source_content.contains("print(") || source_content.contains(";") || source_content.contains("func ") {
                 high_level::compile_high_level(&source_content)
            } else {
                 compiler::compile(&source_content).map_err(|e| vec![CompileError { message: e, span: Span { line: 0, col: 0 } }])
            };

            let compiled_bytes = match result {
                Ok(b) => b,
                 Err(errors) => {
                    println!("❌ Compilation Failed with {} errors:", errors.len());
                    
                    let lines: Vec<&str> = source_content.lines().collect();

                    for e in errors {
                        println!("Error: {}", e.message);
                        
                        let mut line_num = e.span.line;
                        let mut col_num = e.span.col;
                        
                         // Handle EOF or Out of Bounds
                        if line_num > lines.len() {
                            line_num = lines.len();
                            if line_num > 0 { col_num = lines[line_num - 1].len() + 1; }
                        }
                        
                        if line_num > 0 {
                            let line_str = lines[line_num - 1];
                            println!("  --> {}:{}:{}", path, line_num, col_num);
                            println!("   |");
                            println!("{:3}| {}", line_num, line_str);
                            
                            // Print pointer
                            print!("   | ");
                            for _ in 0..(col_num.saturating_sub(1)) { print!(" "); }
                            println!("^");
                            println!("   |");
                        }
                        println!(""); // Spacer
                    }
                    return; 
                }
            };

            // 2. Prepare Directory structure
            let path_obj = std::path::Path::new(path);
            let stem = path_obj.file_stem().unwrap().to_str().unwrap();
            let dir_name = format!("{}_nux", stem);
            
            if let Err(e) = fs::create_dir_all(&dir_name) {
                println!("Error creating directory {}: {}", dir_name, e);
                return;
            }

            let binary_name = format!("{}.nuxi", stem);
            let binary_path = std::path::Path::new(&dir_name).join(&binary_name);

            // 3. Versioning (Backup old .nuxi if exists)
            if binary_path.exists() {
                let backup_name = format!("{}.v1", binary_name);
                let backup_path = std::path::Path::new(&dir_name).join(backup_name);
                
                if let Err(e) = fs::rename(&binary_path, &backup_path) {
                    println!("Warning: Failed to backup old binary: {}", e);
                } else {
                    println!("Saved previous version to {}", backup_path.display());
                }
            }

            // 4. Write New Binary
            if let Err(e) = fs::write(&binary_path, &compiled_bytes) {
                println!("Error writing new binary: {}", e);
                return;
            }
            println!("✅ Successfully compiled to {}", binary_path.display());

            // 5. Run
            println!("Running...");
            let mut machine = vm::NuxVm::new(compiled_bytes);
            machine.run();
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
                println!("Usage: nux translate <source.nux> [--target c|asm] [--profile std|embedded]");
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
                 _ => transpiler::TranspileProfile::Standard,
             };
             
             // Preprocess Imports
             let mut visited = std::collections::HashSet::new();
             let source = match process_imports(std::path::Path::new(source_path), &mut visited) {
                Ok(s) => s,
                Err(e) => { println!("Error processing imports: {}", e); return; }
             };
             
             println!("Compiling to ASM...");
             let asm_source = match high_level::compile_to_asm_source(&source) {
                Ok(s) => s,
                Err(errors) => { 
                    println!("Compilation Failed with {} errors:", errors.len()); 
                    for e in errors { println!("Error: {}", e.message); }
                    return; 
                }
             };
             
             match target_str {
                 "c" => {
                     let c_code = transpiler::transpile_to_c(&asm_source, &profile);
                     let out_name = format!("{}.c", source_path);
                     if let Err(e) = fs::write(&out_name, c_code) {
                         println!("Error writing output: {}", e);
                     } else {
                         println!("✅ Generated C source: {}", out_name);
                     }
                 },
                 "asm" => {
                     let out_name = format!("{}.nux.asm", source_path);
                     if let Err(e) = fs::write(&out_name, asm_source) {
                         println!("Error writing output: {}", e);
                     } else {
                         println!("✅ Generated NuxASM source: {}", out_name);
                     }
                 },
                 _ => println!("Unknown target: {}", target_str),
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
                 if idx + 1 < args.len() && args[idx+1] == "embedded" {
                     profile = transpiler::TranspileProfile::Embedded;
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
                        println!("Error: {}", e.message);
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
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Nux Language Portable SDK");
    println!("Usage:");
    println!("  nux build <source.nux> [output.nuxi]  - Compile (Auto-detects HighLevel/ASM)");
    println!("  nux run   <binary.nuxi>               - Run a Nux binary");
    println!("  nux edit  <file>                      - Open IDE/Editor");
    println!("  nux update                            - Update Nux from GitHub");
    println!("  nux version                           - Show version");
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
        let trimmed = line.trim();
        if trimmed.starts_with("import ") && trimmed.ends_with(";") {
            // import "filename";
            let start = trimmed.find('"').ok_or("Invalid import syntax")? + 1;
            let end = trimmed.rfind('"').ok_or("Invalid import syntax")?;
            let import_path_str = &trimmed[start..end];
            
            // Resolve path relative to current file
            let parent = path.parent().unwrap_or(std::path::Path::new("."));
            let mut import_path = parent.join(import_path_str);
            
            // "Universal" Library Search: If local path doesn't exist, check standard locations
            if !import_path.exists() {
                 if let Ok(exe_path) = std::env::current_exe() {
                     if let Some(exe_dir) = exe_path.parent() {
                         let lib_path = exe_dir.join(import_path_str);
                         if lib_path.exists() {
                             import_path = lib_path;
                         } else {
                             // Check exe_dir/lib/
                             let lib_sub_path = exe_dir.join("lib").join(import_path_str);
                             // Need to handle "lib/io.nux" vs "io.nux" in lib folder
                             if lib_sub_path.exists() {
                                 import_path = lib_sub_path;
                             }
                         }
                     }
                 }
            }
            
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
