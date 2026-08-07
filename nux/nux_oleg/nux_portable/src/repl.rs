use std::io::{self, Write};
use crate::high_level;
use crate::compiler;
use crate::vm;
use crate::platform;
use crate::lexer::Span;

pub fn start() {
    println!("\x1b[1;36mNux REPL v0.5.0\x1b[0m");
    println!("Type 'exit' to quit.");
    
    let mut buffer = String::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        
        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        
        // Append line to cumulative buffer
        buffer.push_str(&line);
        
        // Compile the buffer
        let result = if buffer.contains("print(") || buffer.contains(";") || buffer.contains("func ") || buffer.contains("fn ") || buffer.contains("var ") {
            high_level::compile_high_level(&buffer)
        } else {
            compiler::compile(&buffer).map_err(|e| vec![high_level::CompileError { message: e, span: Span { line: 0, col: 0 } }])
        };
        
        match result {
            Ok(bytes) => {
                // Execute
                let mut machine = vm::NuxVm::new(bytes);
                let mut platform_instance: Box<dyn platform::Platform> = if cfg!(feature = "gui") {
                    Box::new(platform::desktop::DesktopPlatform::new())
                } else {
                    Box::new(platform::headless::HeadlessPlatform::new())
                };
                
                // For a true REPL we might want to capture stdout to avoid repeating previous prints,
                // but as a V0 hybrid, we run the accumulated buffer.
                machine.run(Some(platform_instance.as_mut()));
            },
            Err(errors) => {
                // If it's a compilation error, it might be an incomplete statement (e.g. missing `}`).
                // But if it's a real syntax error, we should probably remove the last line from the buffer.
                // For now, we will print the error and remove the line to prevent poisoning the buffer.
                for e in &errors {
                    println!("\x1b[1;31merror\x1b[0m: {}", e.message);
                }
                
                // Remove the last added line
                for _ in 0..line.len() {
                    buffer.pop();
                }
            }
        }
    }
}
