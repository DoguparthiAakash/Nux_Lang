use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

const RUNTIME: &str = r#"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int64_t nux_mem_alloc(int64_t bytes) { return bytes > 0 ? (int64_t)(uintptr_t)malloc((size_t)bytes) : 0; }
static int64_t nux_mem_free(int64_t ptr) { free((void*)(uintptr_t)ptr); return 1; }
static int64_t nux_mem_read8(int64_t ptr) { return ptr ? *(uint8_t*)(uintptr_t)ptr : 0; }
static int64_t nux_mem_write8(int64_t ptr, int64_t value) { if (!ptr) return 0; *(uint8_t*)(uintptr_t)ptr = (uint8_t)value; return 1; }
static int64_t nux_mem_read64(int64_t ptr) { return ptr ? *(int64_t*)(uintptr_t)ptr : 0; }
static int64_t nux_mem_write64(int64_t ptr, int64_t value) { if (!ptr) return 0; *(int64_t*)(uintptr_t)ptr = value; return 1; }
static int64_t nux_mem_copy(int64_t dst, int64_t src, int64_t bytes) { if (!dst || !src || bytes < 0) return 0; memcpy((void*)(uintptr_t)dst, (void*)(uintptr_t)src, (size_t)bytes); return bytes; }
static int64_t nux_mem_set(int64_t ptr, int64_t value, int64_t bytes) { if (!ptr || bytes < 0) return 0; memset((void*)(uintptr_t)ptr, (int)value, (size_t)bytes); return bytes; }
static int64_t nux_mem_size(void) { return 0; }
"#;

pub fn compile_to_object(source_path: &Path, output_path: &Path) -> Result<(), String> {
    let compiler = std::env::var("NUX_CC").unwrap_or_else(|_| "cc".to_string());
    compile_to_object_with(source_path, output_path, &compiler)
}

pub fn compile_to_llvm_object(source_path: &Path, output_path: &Path) -> Result<(), String> {
    let compiler = std::env::var("NUX_CLANG").unwrap_or_else(|_| "clang".to_string());
    compile_to_object_with(source_path, output_path, &compiler)
}

fn compile_to_object_with(source_path: &Path, output_path: &Path, compiler: &str) -> Result<(), String> {
    let source = fs::read_to_string(source_path).map_err(|error| format!("unable to read source: {}", error))?;
    let generated = transpile(&source)?;
    let generated_path = output_path.with_extension("nux.native.c");
    fs::write(&generated_path, generated).map_err(|error| format!("unable to write native source: {}", error))?;

    let mut command = Command::new(compiler);
    if Path::new(compiler).file_stem().and_then(|name| name.to_str()) == Some("zig") {
        command.arg("cc");
    }
    let result = command
        .args(["-c"])
        .arg(&generated_path)
        .arg("-o")
        .arg(output_path)
        .status();
    let _ = fs::remove_file(&generated_path);

    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("native compiler exited with {}", status.code().unwrap_or(-1))),
        Err(error) => Err(format!("failed to execute '{}': {}", compiler, error)),
    }
}

fn transpile(source: &str) -> Result<String, String> {
    let mut output = String::from(RUNTIME);
    let mut function_depth = 0i32;

    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("import ") {
            continue;
        }
        if line.starts_with("func ") {
            output.push_str(&translate_function(line)?);
            function_depth += 1;
            continue;
        }
        if line == "}" {
            if function_depth > 0 {
                output.push_str("}\n");
                function_depth -= 1;
            } else {
                output.push_str("}\n");
            }
            continue;
        }
        output.push_str(&translate_line(line));
        output.push('\n');
    }

    if function_depth != 0 {
        return Err("native backend found an unbalanced function block".to_string());
    }
    Ok(output)
}

fn translate_function(line: &str) -> Result<String, String> {
    let open = line.find('(').ok_or_else(|| "function is missing '('".to_string())?;
    let close = line[open..].find(')').map(|offset| open + offset).ok_or_else(|| "function is missing ')'".to_string())?;
    let name = line[5..open].trim();
    let arguments = &line[open + 1..close];
    let mut result = String::new();
    if name == "main" {
        result.push_str("int main(void) {\n");
    } else {
        result.push_str("int64_t ");
        result.push_str(name);
        result.push('(');
        for (index, argument) in arguments.split(',').filter(|argument| !argument.trim().is_empty()).enumerate() {
            if index > 0 { result.push_str(", "); }
            let argument_name = argument.split(':').next().unwrap_or(argument).trim();
            result.push_str("int64_t ");
            result.push_str(argument_name);
        }
        result.push_str(") {\n");
    }
    Ok(result)
}

fn translate_line(line: &str) -> String {
    let mut value = line.to_string();
    if let Some(comment) = value.find('#') { value.truncate(comment); }
    value = value.replace("true", "1").replace("false", "0");
    value = value.replace("mem_alloc", "nux_mem_alloc");
    value = value.replace("mem_free", "nux_mem_free");
    value = value.replace("mem_read8", "nux_mem_read8");
    value = value.replace("mem_write8", "nux_mem_write8");
    value = value.replace("mem_read64", "nux_mem_read64");
    value = value.replace("mem_write64", "nux_mem_write64");
    value = value.replace("mem_copy", "nux_mem_copy");
    value = value.replace("mem_set", "nux_mem_set");
    value = value.replace("mem_size", "nux_mem_size");
    value = value.replace("for (var ", "for (int64_t ");

    if value.starts_with("for (") && value.ends_with(") {") {
        if let Some(in_position) = value.find(" in rangeOf(") {
            let name = value[5..in_position].trim();
            let limit_start = in_position + " in rangeOf(".len();
            let limit = value[limit_start..value.len() - 4].trim();
            return format!("for (int64_t {0} = 0; {0} < {1}; {0}++) {{", name, limit);
        }
    }

    if value.starts_with("var ") || value.starts_with("const ") {
        let keyword_end = value.find(' ').unwrap_or(0) + 1;
        value.replace_range(..keyword_end, "int64_t ");
        if let Some(colon) = value.find(':') {
            if let Some(equal) = value[colon..].find('=') {
                value.replace_range(colon..colon + equal, " ");
            } else if value.ends_with(';') {
                value.replace_range(colon..value.len() - 1, " ");
            }
        }
    }

    if value.starts_with("println(") && value.ends_with(");") {
        let expression = &value[8..value.len() - 2];
        if expression.trim_start().starts_with('"') {
            return format!("printf(\"%s\\n\", {});", expression);
        }
        return format!("printf(\"%lld\\n\", (long long)({}));", expression);
    }
    if value.starts_with("print(") && value.ends_with(");") {
        let expression = &value[6..value.len() - 2];
        if expression.trim_start().starts_with('"') {
            return format!("printf(\"%s\", {});", expression);
        }
        return format!("printf(\"%lld\", (long long)({}));", expression);
    }
    value
}

#[allow(dead_code)]
fn _io_error(error: io::Error) -> String { error.to_string() }
