use std::fs;
use std::process::Command;
use std::path::Path;
use crate::platform::Platform;
use std::env;

/// Desktop JIT Execution Engine
/// This converts the parsed Nux Bytecode directly into highly optimized C Code,
/// compiles it via GCC at runtime into a native executable/library, and executes it.
pub fn execute_jit(bytecode: &[u8], _platform: Option<&mut dyn Platform>) {
    eprintln!("\x1b[1;36m   JIT Compiler\x1b[0m Generating native machine code...");

    // Find where the program starts (skip headers)
    let mut start_idx = 0;
    if bytecode.len() >= 64 && bytecode[0] == b'A' && bytecode[1] == b'N' && bytecode[2] == b'U' && bytecode[3] == b'X' {
        start_idx = 64;
    }

    let mut c_code = String::new();
    c_code.push_str("#include <stdio.h>\n");
    c_code.push_str("#include <stdint.h>\n");
    c_code.push_str("#include <stdlib.h>\n\n");
    c_code.push_str("int32_t stack[4096];\n");
    c_code.push_str("int32_t vars[4096];\n");
    c_code.push_str("int sp = -1;\n\n");
    c_code.push_str("#define PUSH(x) stack[++sp] = (x)\n");
    c_code.push_str("#define POP() stack[sp--]\n\n");
    c_code.push_str("int main() {\n");
    c_code.push_str("    int32_t a, b;\n");

    let key = b"NUX_SECURE_KEY_123";
    
    let mut pc = start_idx;
    while pc < bytecode.len() {
        // We emit a label for EVERY bytecode offset so JMP/JE can target them seamlessly!
        c_code.push_str(&format!("    L_{:04X}:\n", pc));

        // Read and decrypt opcode
        let mut opcode = bytecode[pc];
        if pc >= 64 {
            opcode ^= key[(pc - 64) % 18];
        }
        pc += 1;
        
        let mut read_byte = || -> u8 {
            if pc >= bytecode.len() { return 0; }
            let mut val = bytecode[pc];
            if pc >= 64 {
                val ^= key[(pc - 64) % 18];
            }
            pc += 1;
            val
        };

        if opcode >= 0xA0 && opcode <= 0xAF {
            c_code.push_str(&format!("        PUSH({});\n", opcode - 0xA0));
            continue;
        }

        match opcode {
            0xB0 => {
                let v = read_byte() as i8;
                c_code.push_str(&format!("        PUSH({});\n", v));
            }
            0xB1 => {
                let mut val: i32 = 0;
                val |= read_byte() as i32;
                val |= (read_byte() as i32) << 8;
                if val & 0x8000 != 0 { val |= -65536; } // sign extend
                c_code.push_str(&format!("        PUSH({});\n", val));
            }
            0xB2 => {
                let mut val: i32 = 0;
                val |= read_byte() as i32;
                val |= (read_byte() as i32) << 8;
                val |= (read_byte() as i32) << 16;
                val |= (read_byte() as i32) << 24;
                c_code.push_str(&format!("        PUSH({});\n", val));
            }
            0x01 => {
                let mut val: i32 = 0;
                val |= read_byte() as i32;
                val |= (read_byte() as i32) << 8;
                val |= (read_byte() as i32) << 16;
                val |= (read_byte() as i32) << 24;
                for _ in 0..4 { read_byte(); }
                c_code.push_str(&format!("        PUSH({});\n", val));
            }
            0x02 => {
                c_code.push_str("        if (sp >= 0) sp--;\n");
            }
            0x10 => c_code.push_str("        b = POP(); a = POP(); PUSH(a + b);\n"),
            0x11 => c_code.push_str("        b = POP(); a = POP(); PUSH(a - b);\n"),
            0x12 => c_code.push_str("        b = POP(); a = POP(); PUSH(a * b);\n"),
            0x13 => c_code.push_str("        b = POP(); a = POP(); PUSH(b == 0 ? 0 : a / b);\n"),
            0x14 => c_code.push_str("        b = POP(); a = POP(); PUSH(b == 0 ? 0 : a % b);\n"),
            0x90 => c_code.push_str("        b = POP(); a = POP(); PUSH(a == b ? 1 : 0);\n"),
            0x91 => c_code.push_str("        b = POP(); a = POP(); PUSH(a != b ? 1 : 0);\n"),
            0x92 => c_code.push_str("        b = POP(); a = POP(); PUSH(a < b ? 1 : 0);\n"),
            0x93 => c_code.push_str("        b = POP(); a = POP(); PUSH(a > b ? 1 : 0);\n"),
            0x94 => c_code.push_str("        b = POP(); a = POP(); PUSH(a <= b ? 1 : 0);\n"),
            0x95 => c_code.push_str("        b = POP(); a = POP(); PUSH(a >= b ? 1 : 0);\n"),
            0x18 => c_code.push_str("        b = POP(); a = POP(); PUSH((a && b) ? 1 : 0);\n"),
            0x19 => c_code.push_str("        b = POP(); a = POP(); PUSH((a || b) ? 1 : 0);\n"),
            0x60 => {
                let mut addr: i32 = 0;
                addr |= read_byte() as i32;
                addr |= (read_byte() as i32) << 8;
                addr |= (read_byte() as i32) << 16;
                addr |= (read_byte() as i32) << 24;
                for _ in 0..4 { read_byte(); }
                c_code.push_str(&format!("        goto L_{:04X};\n", addr));
            }
            0x61 => {
                let mut addr: i32 = 0;
                addr |= read_byte() as i32;
                addr |= (read_byte() as i32) << 8;
                addr |= (read_byte() as i32) << 16;
                addr |= (read_byte() as i32) << 24;
                for _ in 0..4 { read_byte(); }
                c_code.push_str(&format!("        b = POP(); a = POP(); if (a == b) goto L_{:04X};\n", addr));
            }
            0x51 => {
                c_code.push_str("        putchar((uint8_t)POP());\n");
            }
            0x40 => {
                c_code.push_str("        a = POP(); PUSH(vars[a / 8]);\n");
            }
            0x41 => {
                c_code.push_str("        a = POP(); b = POP(); vars[a / 8] = b;\n");
            }
            _ => {
                c_code.push_str(&format!("        /* Unknown Opcode: 0x{:02X} */\n", opcode));
            }
        }
    }
    
    // Add end label for jump bounds
    c_code.push_str(&format!("    L_{:04X}:\n", pc));
    c_code.push_str("    return 0;\n}\n");

    let temp_dir = env::temp_dir();
    let c_file_path = temp_dir.join("nux_jit.c");
    let exe_path = temp_dir.join(if cfg!(windows) { "nux_jit.exe" } else { "nux_jit" });

    fs::write(&c_file_path, c_code).expect("Failed to write JIT C file");

    eprintln!("\x1b[1;36m   JIT Compiler\x1b[0m Invoking native GCC compiler (-O3)...");

    let status = Command::new("gcc")
        .arg("-O3")
        .arg(&c_file_path)
        .arg("-o")
        .arg(&exe_path)
        .status();

    match status {
        Ok(s) if s.success() => {
            eprintln!("\x1b[1;32m    JIT Executing\x1b[0m {}\n", exe_path.display());
            let _ = Command::new(&exe_path).status();
        },
        _ => {
            eprintln!("\x1b[1;31merror\x1b[0m: JIT compilation failed. Make sure 'gcc' is installed and in your PATH.");
            eprintln!("Falling back to standard bytecode interpreter...");
            let mut machine = crate::vm::NuxVm::new(bytecode.to_vec());
            machine.run(None);
        }
    }
}
