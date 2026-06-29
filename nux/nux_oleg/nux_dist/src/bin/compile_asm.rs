use nux::compile_to_asm;
use std::fs;

fn main() {
    let source = fs::read_to_string("test_lumina.nux").unwrap();
    let asm = compile_to_asm(&source).unwrap();
    fs::write("test_lumina.asm", asm).unwrap();
}
