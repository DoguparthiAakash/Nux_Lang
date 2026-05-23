
use std::fs;
use std::process::Command;

pub enum TranspileTarget {
    C,
    Asm, // Native ASM (using GCC as backend for now)
}

pub enum TranspileProfile {
    Standard, // Linux/PC (stdio, main)
    Embedded, // Bare metal (no stdio, no main, macros)
}

pub struct TranspilerConfig {
    pub target: TranspileTarget,
    pub profile: TranspileProfile,
}

pub fn transpile_and_compile(asm: &str, output_path: &str, config: &TranspilerConfig) -> Result<(), String> {
    let c_code = transpile_to_c(asm, &config.profile);
    let c_file = format!("{}.c", output_path);
    
    fs::write(&c_file, c_code).map_err(|e| e.to_string())?;
    
    // Compile using gcc
    // If embedded, we probably CAN'T compile with host gcc to a binary easily without a linker script/entry point.
    // So 'build-native' usually implies Standard profile.
    
    match config.profile {
        TranspileProfile::Standard => {
            let status = Command::new("gcc")
                .arg(&c_file)
                .arg("-o")
                .arg(output_path)
                .arg("-O3")
                .status()
                .map_err(|e| format!("Failed to run gcc: {}", e))?;
                
            if !status.success() {
                return Err("Compilation failed".to_string());
            }
        },
        TranspileProfile::Embedded => {
             println!("⚠️  Note: Embedded profile selected. 'build-native' only produced the C source: {}", c_file);
             println!("   You must compile this file with your target toolchain (e.g. avr-gcc, xtensa-gcc).");
        }
    }
    
    Ok(())
}

pub fn transpile_to_c(asm: &str, profile: &TranspileProfile) -> String {
    let mut code = String::new();
    
    match profile {
        TranspileProfile::Standard => {
            code.push_str(r#"
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define NUX_STACK_SIZE 1024
int64_t stack[NUX_STACK_SIZE];
int sp = -1;
int64_t vars[1024];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

// Standard IO Mappings
#define NUX_PRINT_VAL(x) printf("%ld", (x))
#define NUX_PRINT_CHAR(x) printf("%c", (char)(x))
#define NUX_INPUT() getchar()
#define NUX_EXIT() return 0

int main() {
"#);
        },
        TranspileProfile::Embedded => {
            code.push_str(r#"
/* Nux Embedded C Output */
#include <stdint.h>

/* USER MUST DEFINE THESE MACROS IN THEIR PROJECT */
/*
#define NUX_PRINT_VAL(x)  Serial.print(x)
#define NUX_PRINT_CHAR(x) Serial.write((char)x)
#define NUX_INPUT()       0
#define NUX_EXIT()        while(1){}
*/

#ifndef NUX_STACK_SIZE
#define NUX_STACK_SIZE 256
#endif

int64_t stack[NUX_STACK_SIZE];
int sp = -1;
int64_t vars[256]; 

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

/* Entry point to call from setup() or main() */
int nux_entry() {
"#);
        }
    }
    
    for line in asm.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }
        
        if line.ends_with(':') {
             let label = &line[..line.len()-1];
             code.push_str(&format!("{}:\n", label));
             continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = parts[0];
        
        match mnemonic {
            "PUSH" => code.push_str(&format!("    PUSH({});\n", parts[1])),
            "ADD" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a + b); }\n"),
            "SUB" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a - b); }\n"),
            "MUL" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a * b); }\n"),
            "DIV" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a / b); }\n"),
            "EQ" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a == b ? 1 : 0); }\n"),
            "NEQ" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a != b ? 1 : 0); }\n"),
            "LT" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a < b ? 1 : 0); }\n"),
            "GT" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a > b ? 1 : 0); }\n"),
            "LTE" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a <= b ? 1 : 0); }\n"),
            "GTE" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a >= b ? 1 : 0); }\n"),
            "AND" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a && b ? 1 : 0); }\n"),
            "OR" => code.push_str("    { int64_t b = POP(); int64_t a = POP(); PUSH(a || b ? 1 : 0); }\n"),
            
            "JMP" => code.push_str(&format!("    goto {};\n", parts[1])),
            "JE" => code.push_str(&format!("    {{ int64_t b = POP(); int64_t a = POP(); if (a == b) goto {}; }}\n", parts[1])),
            
            "PRINT_VAL" => code.push_str("    NUX_PRINT_VAL(POP());\n"),
            "PRINT_CHAR" => code.push_str("    NUX_PRINT_CHAR(POP());\n"),
            "PEEK" => code.push_str("    { int64_t addr = POP(); PUSH(vars[addr / 8]); }\n"),
            "POKE" => code.push_str("    { int64_t addr = POP(); int64_t val = POP(); vars[addr / 8] = val; }\n"),
            "INPUT" => code.push_str("    PUSH(NUX_INPUT());\n"),
            "EXIT" => code.push_str("    NUX_EXIT();\n"),
            "RET" => code.push_str("    return 0;\n"), // For simple functions
            "CALL" => code.push_str(&format!("    // Call {} not fully implemented in C transpiler yet\n", parts[1])), 
            _ => code.push_str(&format!("    // Unknown: {}\n", line)),
        }
    }
    
    code.push_str("    return 0;\n}\n");
    code
}
