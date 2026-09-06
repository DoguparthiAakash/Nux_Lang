use std::fs;
use std::collections::HashMap;

fn main() {
    let assembly = fs::read_to_string("test_all.asm").unwrap();
    let mut labels = HashMap::new();
    let mut ops = Vec::new();

    for line in assembly.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }
        
        if line.ends_with(':') {
            let label = line[..line.len()-1].to_string();
            labels.insert(label, ops.len());
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "PUSH" => { ops.push(0x10); ops.extend_from_slice(&[0u8; 8]); },
            "POP" => { ops.push(0x11); },
            "ADD" => { ops.push(0x20); },
            "SUB" => { ops.push(0x21); },
            "MUL" => { ops.push(0x22); },
            "DIV" => { ops.push(0x23); },
            "MOD" => { ops.push(0x24); },
            "LT" => { ops.push(0x30); },
            "GT" => { ops.push(0x31); },
            "EQ" => { ops.push(0x32); },
            "PEEK" => { ops.push(0x40); },
            "POKE" => { ops.push(0x41); },
            "JMP" => { ops.push(0x50); ops.extend_from_slice(&[0u8; 8]); },
            "JE" => { ops.push(0x51); ops.extend_from_slice(&[0u8; 8]); },
            "CALL" => { ops.push(0x70); ops.extend_from_slice(&[0u8; 8]); ops.extend_from_slice(&[0u8; 8]); },
            "RET" => { ops.push(0x71); },
            "PRINT_VAL" => { ops.push(0x80); },
            "OP_SPAWN" => { ops.push(0x90); ops.extend_from_slice(&[0u8; 8]); ops.extend_from_slice(&[0u8; 8]); },
            "OP_JOIN" => { ops.push(0x91); },
            "SET_LOCAL" => { ops.push(0x45); ops.extend_from_slice(&[0u8; 8]); },
            "OP_GET_LOCAL" => { ops.push(0x46); ops.extend_from_slice(&[0u8; 8]); },
            "EXIT" => { ops.push(0xFF); },
            _ => {},
        }
    }
    
    println!("alloc: {:?}", labels.get("alloc"));
    println!("fib_iter: {:?}", labels.get("fib_iter"));
}
