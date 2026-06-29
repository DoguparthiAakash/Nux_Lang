pub fn transpile(input: &str) -> String {
    let mut out = String::new();
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stddef.h>\n\n");
    
    // Simple state machine for transpiling
    let mut in_export = false;
    let mut func_name = String::new();
    let mut args = Vec::new(); // (name, type)

    let lines = input.lines();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("export func ") {
            in_export = true;
            // Parse: export func name(arg1: type, arg2: type) -> type {
            let start = trimmed.find("export func ").unwrap() + 12;
            let paren_open = trimmed.find('(').unwrap_or(trimmed.len());
            func_name = trimmed[start..paren_open].trim().to_string();
            
            let paren_close = trimmed.find(')').unwrap_or(trimmed.len());
            let args_str = &trimmed[paren_open + 1..paren_close];
            args.clear();
            for arg in args_str.split(',') {
                if arg.trim().is_empty() { continue; }
                let parts: Vec<&str> = arg.split(':').collect();
                if parts.len() == 2 {
                    args.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
                } else {
                    args.push((arg.trim().to_string(), "int64_t".to_string())); // Default to int64_t
                }
            }
            
            out.push_str(&format!("int64_t {}_impl(", func_name));
            for (i, (arg_name, _arg_type)) in args.iter().enumerate() {
                if i > 0 { out.push_str(", "); }
                out.push_str(&format!("int64_t {}", arg_name));
            }
            out.push_str(") {\n");
        } else if in_export && trimmed == "}" {
            out.push_str("}\n\n");
            
            // Generate the Nux FFI wrapper
            out.push_str("#ifdef _WIN32\n__declspec(dllexport)\n#endif\n");
            out.push_str(&format!("int64_t {}(const int64_t* args, size_t num_args, const void* state) {{\n", func_name));
            out.push_str(&format!("    if (num_args < {}) return 0;\n", args.len()));
            for (i, (arg_name, _)) in args.iter().enumerate() {
                out.push_str(&format!("    int64_t {} = args[{}];\n", arg_name, i));
            }
            
            out.push_str(&format!("    return {}_impl(", func_name));
            for (i, (arg_name, _)) in args.iter().enumerate() {
                if i > 0 { out.push_str(", "); }
                out.push_str(arg_name);
            }
            out.push_str(");\n}\n\n");
            
            in_export = false;
            func_name.clear();
        } else {
            // Pass through other lines as standard C/CUDA
            out.push_str(line);
            out.push('\n');
        }
    }
    
    out
}
