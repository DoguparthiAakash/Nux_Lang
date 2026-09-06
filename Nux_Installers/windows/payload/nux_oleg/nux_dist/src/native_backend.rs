use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn compile_native(input_file: &str, source: &str, output_path: &str) -> Result<(), String> {
    println!("Native compilation starting for: {}", input_file);
    
    // 1. Strip @[no_std] and @[entry]
    let re_attr = Regex::new(r"@\[.*?\]").unwrap();
    let mut content = re_attr.replace_all(source, "").to_string();
    
    // 2. Comments: # to //
    let re_comment = Regex::new(r"#(.*)").unwrap();
    content = re_comment.replace_all(&content, "//$1").to_string();

    // 3. Imports
    let re_import = Regex::new(r"import\s+([a-zA-Z0-9_:]+);").unwrap();
    content = re_import.replace_all(&content, "").to_string();

    // 3b. Casts (strip 'as Type' for C compatibility)
    content = content.replace(" as u8", "");
    content = content.replace(" as u16", "");
    content = content.replace(" as u32", "");
    content = content.replace(" as *u8", "");
    content = content.replace(" as *u16", "");
    content = content.replace(" as *u32", "");


    // 4. Struct setup
    let re_class = Regex::new(r"class\s+([a-zA-Z0-9_]+)\s*\{").unwrap();
    content = re_class.replace_all(&content, "struct $1 {\n").to_string();
    
    // 5. Assembly
    content = content.replace("asm(", "__asm__(");


    let lines: Vec<&str> = content.split('\n').collect();
    let mut out_lines = Vec::new();
    let mut in_class = None;
    
    for line in lines {
        let mut l = line.to_string();
        
        // Function calls like shell::start_shell() -> start_shell()
        let re_func_call = Regex::new(r"([a-zA-Z0-9_]+)::([a-zA-Z0-9_]+)").unwrap();
        l = re_func_call.replace_all(&l, "$2").to_string();
        
        let re_struct = Regex::new(r"^struct\s+([a-zA-Z0-9_]+)\s*\{").unwrap();
        if let Some(caps) = re_struct.captures(&l) {
            let class_name = caps[1].to_string();
            in_class = Some(class_name.clone());
            out_lines.push(format!("typedef struct {} {};", class_name, class_name));
            out_lines.push(l);
            continue;
        }
            
        if in_class.is_some() && l.trim() == "}" {
            out_lines.push("};".to_string());
            in_class = None;
            continue;
        }
            
        if in_class.is_some() && l.contains("func ") {
            out_lines.push(format!("// {}", l));
            continue;
        }

        // Variables: var name: type = val; -> type name = val;
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u8\s*=").unwrap().replace_all(&l, "uint8_t* $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u16\s*=").unwrap().replace_all(&l, "uint16_t* $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u32\s*=").unwrap().replace_all(&l, "uint32_t* $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*?fs::File\s*=").unwrap().replace_all(&l, "File* $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*File\s*=").unwrap().replace_all(&l, "File* $1 =").to_string();
        
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u8\s*=").unwrap().replace_all(&l, "uint8_t $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u16\s*=").unwrap().replace_all(&l, "uint16_t $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u32\s*=").unwrap().replace_all(&l, "uint32_t $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*bool\s*=").unwrap().replace_all(&l, "bool $1 =").to_string();
        
        // var name: type;
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u8").unwrap().replace_all(&l, "uint8_t* $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u16").unwrap().replace_all(&l, "uint16_t* $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*u32").unwrap().replace_all(&l, "uint32_t* $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*?fs::File").unwrap().replace_all(&l, "File* $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*\*File").unwrap().replace_all(&l, "File* $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*fs::Directory").unwrap().replace_all(&l, "Directory $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*Directory").unwrap().replace_all(&l, "Directory $1").to_string();
        
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u8").unwrap().replace_all(&l, "uint8_t $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u16").unwrap().replace_all(&l, "uint16_t $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*u32").unwrap().replace_all(&l, "uint32_t $1").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*:\s*bool").unwrap().replace_all(&l, "bool $1").to_string();
        
        // Untyped vars
        l = Regex::new(r"var\s+root\s*=\s*get_root\(\);").unwrap().replace_all(&l, "Directory root = get_root();").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*=").unwrap().replace_all(&l, "uint32_t $1 =").to_string();
        l = Regex::new(r"var\s+([a-zA-Z0-9_]+)\s*;").unwrap().replace_all(&l, "uint32_t $1;").to_string();
        
        // Functions: func name(args) -> type
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*bool").unwrap().replace_all(&l, "bool $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*u8").unwrap().replace_all(&l, "uint8_t $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*u16").unwrap().replace_all(&l, "uint16_t $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*u32").unwrap().replace_all(&l, "uint32_t $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*\*u8").unwrap().replace_all(&l, "uint8_t* $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)\s*->\s*Directory").unwrap().replace_all(&l, "Directory $1($2)").to_string();
        l = Regex::new(r"func\s+([a-zA-Z0-9_]+)\s*\((.*?)\)").unwrap().replace_all(&l, "void $1($2)").to_string();
        
        // Function arguments translation
        let re_args_match = Regex::new(r"void\s+[a-zA-Z0-9_]+\s*\(|bool\s+[a-zA-Z0-9_]+\s*\(|uint\d+_t\*?\s+[a-zA-Z0-9_]+\s*\(").unwrap();
        if re_args_match.is_match(&l) {
            let re_args = Regex::new(r"\((.*?)\)").unwrap();
            if let Some(caps) = re_args.captures(&l) {
                let args_str = caps[1].to_string();
                let mut new_args = Vec::new();
                for arg in args_str.split(',') {
                    let arg = arg.trim();
                    if arg.is_empty() { continue; }
                    let parts: Vec<&str> = arg.split(':').collect();
                    if parts.len() == 2 {
                        let name = parts[0].trim();
                        let typ = parts[1].trim();
                        let t = match typ {
                            "u8" => "uint8_t",
                            "*u8" => "uint8_t*",
                            "u16" => "uint16_t",
                            "*u16" => "uint16_t*",
                            "u32" => "uint32_t",
                            "*u32" => "uint32_t*",
                            "bool" => "bool",
                            "*File" => "File*",
                            _ => typ,
                        };
                        new_args.push(format!("{} {}", t, name));
                    }
                }
                l = l.replace(&format!("({})", args_str), &format!("({})", new_args.join(", ")));
            }
        }
        
        l = l.replace("root.files + (i * 12)", "(uint8_t*)root.files + (i * 12)");
        
        // Casts: as *u8 -> (uint8_t*)
        l = Regex::new(r"([a-zA-Z0-9_]+)\s+as\s+\*u8").unwrap().replace_all(&l, "((uint8_t*)$1)").to_string();
        l = Regex::new(r"\s+as\s+\*u8").unwrap().replace_all(&l, " /*as*/").to_string();
        l = Regex::new(r"([a-zA-Z0-9_]+)\s+as\s+u32").unwrap().replace_all(&l, "((uint32_t)$1)").to_string();
        
        if l.contains("root_dir.init(") {
            l = "root_dir.name = \"/\"; root_dir.files = root_files; root_dir.file_count = root_file_count;".to_string();
        }
        
        out_lines.push(l);
    }
    
    let mut full_c = out_lines.join("\n");
    
    // 6. Expand @[builtin] function bodies for std library calls
    full_c = full_c.replace(
        "void outb(uint16_t port, uint8_t val);",
        "static inline void outb(uint16_t port, uint8_t val) { __asm__ volatile (\"outb %0, %1\" : : \"a\"(val), \"Nd\"(port)); }"
    );
    full_c = full_c.replace(
        "uint8_t inb(uint16_t port);",
        "static inline uint8_t inb(uint16_t port) { uint8_t ret; __asm__ volatile (\"inb %1, %0\" : \"=a\"(ret) : \"Nd\"(port)); return ret; }"
    );
    full_c = full_c.replace(
        "void outw(uint16_t port, uint16_t val);",
        "static inline void outw(uint16_t port, uint16_t val) { __asm__ volatile (\"outw %0, %1\" : : \"a\"(val), \"Nd\"(port)); }"
    );
    full_c = full_c.replace(
        "uint16_t inw(uint16_t port);",
        "static inline uint16_t inw(uint16_t port) { uint16_t ret; __asm__ volatile (\"inw %1, %0\" : \"=a\"(ret) : \"Nd\"(port)); return ret; }"
    );
    full_c = full_c.replace(
        "void cli();",
        "static inline void cli() { __asm__ volatile (\"cli\"); }"
    );
    full_c = full_c.replace(
        "void sti();",
        "static inline void sti() { __asm__ volatile (\"sti\"); }"
    );
    full_c = full_c.replace(
        "void hlt();",
        "static inline void hlt() { __asm__ volatile (\"hlt\"); }"
    );
    full_c = full_c.replace(
        "uint8_t* alloc(uint32_t size);",
        "static uint8_t* __heap_ptr = (uint8_t*)0x00400000; \
static inline uint8_t* alloc(uint32_t size) { \
    uint32_t aligned = (size + 7) & ~7u; \
    uint8_t* p = __heap_ptr; __heap_ptr += aligned; return p; }"
    );
    full_c = full_c.replace(
        "void free(uint8_t* ptr);",
        "static inline void free(uint8_t* ptr) { (void)ptr; }"
    );
    full_c = full_c.replace(
        "void copy(uint8_t* dst, uint8_t* src, uint32_t n);",
        "static inline void copy(uint8_t* dst, uint8_t* src, uint32_t n) { \
    for(uint32_t _i=0;_i<n;_i++) dst[_i]=src[_i]; }"
    );
    full_c = full_c.replace(
        "void set(uint8_t* dst, uint8_t val, uint32_t n);",
        "static inline void set(uint8_t* dst, uint8_t val, uint32_t n) { \
    for(uint32_t _i=0;_i<n;_i++) dst[_i]=val; }"
    );
    full_c = full_c.replace(
        "uint32_t cmp(uint8_t* a, uint8_t* b, uint32_t n);",
        "static inline uint32_t cmp(uint8_t* a, uint8_t* b, uint32_t n) { \
    for(uint32_t _i=0;_i<n;_i++) { if(a[_i]!=b[_i]) return (uint32_t)((int)a[_i]-(int)b[_i]); } return 0; }"
    );
    let c_file_path = format!("{}.c", output_path);
    
    let mut f = File::create(&c_file_path).map_err(|e| format!("Failed to create temp C file: {}", e))?;
    f.write_all(b"#include <stdint.h>\n#include <stdbool.h>\n").unwrap();
    f.write_all(b"static inline uint8_t __inb(uint16_t port) { uint8_t ret; __asm__ volatile ( \"inb %1, %0\" : \"=a\"(ret) : \"Nd\"(port) ); return ret; }\n").unwrap();
    f.write_all(b"static inline void __outb(uint16_t port, uint8_t val) { __asm__ volatile ( \"outb %0, %1\" : : \"a\"(val), \"Nd\"(port) ); }\n").unwrap();
    
    if full_c.contains("struct File") {
        f.write_all(b"typedef struct File File;\n").unwrap();
    }
    if full_c.contains("struct Directory") {
        f.write_all(b"typedef struct Directory Directory;\n").unwrap();
    }
    f.write_all(full_c.as_bytes()).unwrap();
    
    // Call GCC directly (nux runs natively on Linux/WSL, so no wsl wrapping needed)
    let status = Command::new("gcc")
        .args(&[
            "-m32",
            "-ffreestanding",
            "-fno-pie",
            "-fno-stack-protector",
            "-Wno-int-conversion",
            "-Wno-implicit-function-declaration",
            "-Wno-incompatible-pointer-types",
            "-c", &c_file_path,
            "-o", &format!("{}.o", output_path),
        ])
        .status()
        .map_err(|e| format!("Failed to execute GCC: {}", e))?;

    if !status.success() {
        return Err("GCC compilation failed — check errors above.".to_string());
    }
    
    // Clean up temporary C file — leave zero intermediate artefacts
    // let _ = std::fs::remove_file(&c_file_path);
    
    Ok(())
}
