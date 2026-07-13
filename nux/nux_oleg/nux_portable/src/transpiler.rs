use std::fs;
use std::process::Command;

#[derive(PartialEq)]
pub enum TranspileTarget {
    C,
    Asm, // Native ASM (using GCC as backend for now)
    MicroVM, // C Micro-interpreter with embedded bytecode
}

#[derive(PartialEq)]
pub enum TranspileProfile {
    Standard, // Linux/PC (stdio, main)
    Embedded, // Bare metal (no stdio, no main, macros)
    Extreme,  // Native C via Symbolic Stack Tracking (No VM overhead)
    Nano,     // Bare metal (< 1KB RAM, int32_t, 32-element arrays)
    Legacy,   // MS-DOS/UNIX (ANSI C89, long, no stdint.h)
}

pub struct TranspilerConfig {
    pub target: TranspileTarget,
    pub profile: TranspileProfile,
}

pub fn transpile_and_compile(asm: &str, output_path: &str, config: &TranspilerConfig) -> Result<(), String> {
    let c_code = transpile_to_c(asm, &config.profile);
    let c_file = format!("{}.c", output_path);
    
    fs::write(&c_file, c_code).map_err(|e| e.to_string())?;
    
    match config.profile {
        TranspileProfile::Standard | TranspileProfile::Extreme => {
            let status = Command::new("gcc")
                .arg(&c_file)
                .arg("-o")
                .arg(output_path)
                .arg("-O3") // Extreme mode thrives on -O3
                .arg("-ffast-math") // Extra speed for Extreme
                .status()
                .map_err(|e| format!("Failed to run gcc: {}", e))?;
                
            if !status.success() {
                return Err("Compilation failed".to_string());
            }
        },
        TranspileProfile::Embedded => {
             println!("⚠️  Note: Embedded profile selected. 'build-native' only produced the C source: {}", c_file);
             println!("   You must compile this file with your target toolchain (e.g. avr-gcc, xtensa-gcc).");
        },
        TranspileProfile::Nano => {
             println!("⚠️  Note: Nano profile selected. Memory footprint is < 1KB.");
             println!("   Output C source: {}. Compile with your micro-controller toolchain.", c_file);
        },
        TranspileProfile::Legacy => {
             println!("⚠️  Note: Legacy profile selected (ANSI C89).");
             println!("   Output C source: {}. Move to MS-DOS and compile with Turbo C or Watcom.", c_file);
        }
    }
    
    Ok(())
}

pub fn transpile_to_c(asm: &str, profile: &TranspileProfile) -> String {
    let mut code = String::new();
    
    let is_extreme = *profile == TranspileProfile::Extreme;
    
    match profile {
        TranspileProfile::Standard | TranspileProfile::Extreme => {
            code.push_str(r#"
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define NUX_INT int64_t

size_t nux_total_alloc = 0;
size_t nux_mem_limit = (size_t)-1;

void* nux_alloc(size_t size) {
    if (nux_mem_limit != (size_t)-1 && nux_total_alloc + size > nux_mem_limit) {
        fprintf(stderr, "Runtime Error: Out of Memory (Hit defined limit in C backend)\\n");
        exit(1);
    }
    void* p = malloc(size);
    if (p) nux_total_alloc += size;
    return p;
}

void nux_free(void* ptr) {
    free(ptr);
}
"#);
            if !is_extreme {
                code.push_str(r#"
#define NUX_STACK_SIZE 1024
NUX_INT stack[NUX_STACK_SIZE];
int sp = -1;
"#);
            }
            code.push_str(r#"
NUX_INT vars[1024];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

// Standard IO Mappings
#define NUX_PRINT_VAL(x) printf("%ld", (x))
#define NUX_PRINT_CHAR(x) printf("%c", (char)(x))
#define NUX_INPUT() getchar()
#define NUX_EXIT() return 0

int main() {
"#);
            if is_extreme {
                // In extreme mode, we pre-declare simulated registers
                code.push_str("    int64_t r[1024] = {0};\n");
            }
        },
        TranspileProfile::Embedded => {
            code.push_str(r#"
/* Nux Embedded C Output */
#include <stdint.h>

#define NUX_INT int64_t

#ifndef NUX_STACK_SIZE
#define NUX_STACK_SIZE 256
#endif

NUX_INT stack[NUX_STACK_SIZE];
int sp = -1;
NUX_INT vars[256]; 

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

/* Entry point to call from setup() or main() */
int nux_entry() {
"#);
        },
        TranspileProfile::Nano => {
            code.push_str(r#"
/* Nux Nano Profile Output */
#include <stdint.h>

#define NUX_INT int32_t
#define NUX_STACK_SIZE 32

NUX_INT stack[NUX_STACK_SIZE];
int sp = -1;
NUX_INT vars[32];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

/* User-defined macros for hardware I/O */
#ifndef NUX_PRINT_VAL
#define NUX_PRINT_VAL(x)
#endif
#ifndef NUX_PRINT_CHAR
#define NUX_PRINT_CHAR(x)
#endif
#define NUX_INPUT() 0
#define NUX_EXIT() return 0

int nux_entry() {
"#);
        },
        TranspileProfile::Legacy => {
            code.push_str(r#"
/* Nux Legacy Profile Output (ANSI C89) */
#include <stdio.h>
#include <stdlib.h>

#define NUX_INT long

long nux_total_alloc = 0;
long nux_mem_limit = -1;

void* nux_alloc(size_t size) {
    if (nux_mem_limit != -1 && nux_total_alloc + size > (size_t)nux_mem_limit) {
        fprintf(stderr, "Runtime Error: Out of Memory\n");
        exit(1);
    }
    void* p = malloc(size);
    if (p) nux_total_alloc += size;
    return p;
}

void nux_free(void* ptr) {
    free(ptr);
}

#define NUX_STACK_SIZE 1024
NUX_INT stack[NUX_STACK_SIZE];
int sp = -1;
NUX_INT vars[1024];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

#define NUX_PRINT_VAL(x) printf("%ld", (x))
#define NUX_PRINT_CHAR(x) printf("%c", (char)(x))
#define NUX_INPUT() getchar()
#define NUX_EXIT() return 0

int main() {
"#);
        }
    }
    
    let mut sim_sp = 0; // Simulated Stack Pointer for Extreme Mode
    
    for line in asm.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }
        
        if line.ends_with(':') {
             let label = &line[..line.len()-1];
             code.push_str(&format!("{}:\n", label));
             if is_extreme {
                 // Reset sim_sp at labels (heuristic for statement boundaries)
                 sim_sp = 0;
             }
             continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = parts[0];
        
        if is_extreme {
            // EXTREME MODE: Symbolic Execution to Registers
            match mnemonic {
                "PUSH" => { code.push_str(&format!("    r[{}] = {};\n", sim_sp, parts[1])); sim_sp += 1; },
                "ADD" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] + r[{}];\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "SUB" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] - r[{}];\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "MUL" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] * r[{}];\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "DIV" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] / r[{}];\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "EQ"  => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] == r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "NEQ" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] != r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "LT"  => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] < r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "GT"  => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] > r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "LTE" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] <= r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "GTE" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] >= r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "AND" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] && r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                "OR"  => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}] || r[{}] ? 1 : 0;\n", sim_sp-1, sim_sp-1, sim_sp)); },
                
                "JMP" => { code.push_str(&format!("    goto {};\n", parts[1])); },
                "JE"  => { sim_sp -= 2; code.push_str(&format!("    if (r[{}] == r[{}]) goto {};\n", sim_sp, sim_sp+1, parts[1])); },
                
                "PRINT_VAL" => { sim_sp -= 1; code.push_str(&format!("    NUX_PRINT_VAL(r[{}]);\n", sim_sp)); },
                "PRINT_CHAR" => { sim_sp -= 1; code.push_str(&format!("    NUX_PRINT_CHAR(r[{}]);\n", sim_sp)); },
                "PEEK" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = vars[r[{}] / 8];\n", sim_sp, sim_sp)); sim_sp += 1; },
                "POKE" => { sim_sp -= 2; code.push_str(&format!("    vars[r[{}] / 8] = r[{}];\n", sim_sp, sim_sp+1)); },
                "INPUT" => { code.push_str(&format!("    r[{}] = NUX_INPUT();\n", sim_sp)); sim_sp += 1; },
                "GET_LOCAL" => { code.push_str(&format!("    r[{}] = r[{}];\n", sim_sp, parts[1])); sim_sp += 1; },
                "SET_LOCAL" => { sim_sp -= 1; code.push_str(&format!("    r[{}] = r[{}];\n", parts[1], sim_sp)); },
                "POP" => { if sim_sp > 0 { sim_sp -= 1; } },
                "CALL" => { code.push_str(&format!("    goto {};\n", parts[1])); },
                "OP_VERIFY" | "VERIFY" => {
                    sim_sp -= 1;
                    code.push_str(&format!("    if (r[{}] == 0) {{ fprintf(stderr, \"Verification Failed!\\n\"); exit(1); }}\n", sim_sp));
                },
                "OP_ALLOC" => {
                    code.push_str(&format!("    r[{}] = (int64_t)malloc(r[{}]);\n", sim_sp - 1, sim_sp - 1));
                },
                "OP_FREE" => {
                    sim_sp -= 1;
                    code.push_str(&format!("    free((void*)r[{}]);\n", sim_sp));
                },
                "EXIT" => code.push_str("    NUX_EXIT();\n"),
                "RET" => code.push_str("    return 0;\n"),
                _ => code.push_str(&format!("    // Unknown (Extreme): {}\n", line)),
            }
        } else {
            // STANDARD / EMBEDDED MODE: VM Emulation via Macros
            match mnemonic {
                "PUSH" => code.push_str(&format!("    PUSH({});\n", parts[1])),
                "ADD" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }\n"),
                "SUB" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }\n"),
                "MUL" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }\n"),
                "DIV" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a / b); }\n"),
                "EQ" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a == b ? 1 : 0); }\n"),
                "NEQ" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a != b ? 1 : 0); }\n"),
                "LT" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }\n"),
                "GT" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a > b ? 1 : 0); }\n"),
                "LTE" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a <= b ? 1 : 0); }\n"),
                "GTE" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a >= b ? 1 : 0); }\n"),
                "AND" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a && b ? 1 : 0); }\n"),
                "OR" => code.push_str("    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a || b ? 1 : 0); }\n"),
                
                "JMP" => code.push_str(&format!("    goto {};\n", parts[1])),
                "JE" => code.push_str(&format!("    {{ NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto {}; }}\n", parts[1])),
                
                "PRINT_VAL" => code.push_str("    NUX_PRINT_VAL(POP());\n"),
                "PRINT_CHAR" => code.push_str("    NUX_PRINT_CHAR(POP());\n"),
                "PEEK" => code.push_str("    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }\n"),
                "POKE" => code.push_str("    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }\n"),
                "INPUT" => code.push_str("    PUSH(NUX_INPUT());\n"),
                "OP_VERIFY" | "VERIFY" => code.push_str("    { NUX_INT val = POP(); if (val == 0) { fprintf(stderr, \"Verification Failed!\\n\"); exit(1); } }\n"),
                "OP_ALLOC" => {
                     if *profile == TranspileProfile::Nano {
                         code.push_str("    { NUX_INT size = POP(); PUSH(0); /* Alloc unsupported in Nano */ }\n");
                     } else {
                         code.push_str("    { NUX_INT size = POP(); PUSH((NUX_INT)nux_alloc(size)); }\n");
                     }
                },
                "OP_FREE" => {
                     if *profile == TranspileProfile::Nano {
                         code.push_str("    { NUX_INT addr = POP(); /* Free unsupported in Nano */ }\n");
                     } else {
                         code.push_str("    { NUX_INT addr = POP(); nux_free((void*)addr); }\n");
                     }
                },
                "OP_LIMIT_MEM" => code.push_str("    { NUX_INT percent = POP(); /* memory limit */ }\n"),
                "EXIT" => code.push_str("    NUX_EXIT();\n"),
                "RET" => code.push_str("    return 0;\n"), 
                "CALL" => code.push_str(&format!("    /* Call {} not fully implemented in C transpiler yet */\n", parts[1])), 
                _ => code.push_str(&format!("    /* Unknown: {} */\n", line)),
            }
        }
    }
    
    code.push_str("    return 0;\n}\n");
    code
}
