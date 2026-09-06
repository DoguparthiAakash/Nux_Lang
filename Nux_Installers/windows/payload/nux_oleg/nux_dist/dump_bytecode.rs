use std::fs;

fn main() {
    let bytecode = fs::read("target/debug/test_all.nuxc").unwrap_or_else(|_| fs::read("test_all.nuxc").unwrap());
    println!("Bytecode len: {}", bytecode.len());
    
    print!("1170: ");
    for i in 1170..1190 {
        print!("{:02X} ", bytecode[i]);
    }
    println!();
    
    print!("2969: ");
    for i in 2969..2989 {
        print!("{:02X} ", bytecode[i]);
    }
    println!();
}
