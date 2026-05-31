use std::fs;
use std::collections::HashMap;

fn main() {
    let assembly = fs::read_to_string("test_all.asm").unwrap_or_else(|_| String::from_utf8_lossy(&fs::read("test_all.asm").unwrap()).into_owned());
    let mut labels = HashMap::new();

    for line in assembly.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }
        
        if line.ends_with(':') {
            let label = line[..line.len()-1].to_string();
            labels.insert(label, 1);
        }
    }
    
    let mut keys: Vec<String> = labels.keys().cloned().collect();
    keys.sort();
    println!("Keys: {:?}", keys);
}
