use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

mod transpiler;

fn main() {
    println!("\x1b[1;36m=================================================\x1b[0m");
    println!("\x1b[1;32m   CUX Native Compiler & Transpiler \x1b[0m");
    println!("\x1b[1;36m=================================================\x1b[0m\n");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] != "build" {
        println!("\x1b[1;33mUsage:\x1b[0m");
        println!("  cux build <file.cux | file.cu | file.c> [output_name]");
        return;
    }

    let input_path = &args[2];
    let default_out = input_path.split('.').next().unwrap_or("out").to_string();
    let output_name = if args.len() >= 4 { &args[3] } else { &default_out };

    let path = Path::new(input_path);
    if !path.exists() {
        println!("\x1b[1;31mError: File not found '{}'\x1b[0m", input_path);
        return;
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let is_cux = ext == "cux";

    println!("\x1b[1;34m[*] Analyzing file: {}\x1b[0m", input_path);

    let mut compile_target = input_path.clone();
    let generated_file = format!("{}.generated.c", default_out);

    if is_cux {
        println!("\x1b[1;34m[*] Transpiling .cux to C/CUDA...\x1b[0m");
        let content = fs::read_to_string(input_path).expect("Failed to read file");
        let transpiled = transpiler::transpile(&content);
        fs::write(&generated_file, transpiled).expect("Failed to write generated file");
        compile_target = generated_file.clone();
    }

    println!("\x1b[1;34m[*] Compiling native library...\x1b[0m");

    // Try to compile. Let's prefer `gcc` or `clang` for .c, and `nvcc` for .cu
    let target_ext = Path::new(&compile_target).extension().and_then(|e| e.to_str()).unwrap_or("");
    let is_cuda = target_ext == "cu";

    let compiler = if is_cuda { "nvcc" } else { "gcc" };
    
    // For Windows, output is .dll
    #[cfg(target_os = "windows")]
    let out_file = format!("{}.dll", output_name);
    #[cfg(not(target_os = "windows"))]
    let out_file = format!("{}.so", output_name);

    let mut cmd = Command::new(compiler);
    if compiler == "nvcc" {
        cmd.arg("--shared").arg("-o").arg(&out_file).arg(&compile_target);
    } else {
        cmd.arg("-shared").arg("-fPIC").arg("-o").arg(&out_file).arg(&compile_target);
    }

    let status = cmd.status();

    match status {
        Ok(s) if s.success() => {
            println!("\n\x1b[1;32m[+] Successfully compiled to {}\x1b[0m", out_file);
        }
        Ok(s) => {
            println!("\n\x1b[1;31m[-] Compilation failed with exit code {}\x1b[0m", s.code().unwrap_or(-1));
        }
        Err(e) => {
            println!("\n\x1b[1;31m[-] Failed to execute compiler '{}': {}\x1b[0m", compiler, e);
            if compiler == "gcc" {
                println!("\x1b[1;33m[!] Make sure 'gcc' (MinGW or Linux GCC) is in your PATH.\x1b[0m");
            } else if compiler == "nvcc" {
                println!("\x1b[1;33m[!] Make sure NVIDIA CUDA Toolkit is installed and 'nvcc' is in your PATH.\x1b[0m");
            }
        }
    }
}
