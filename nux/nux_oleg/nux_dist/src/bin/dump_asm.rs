use std::fs;
use nux::compiler::compile_to_asm_source;

fn main() {
    let source = fs::read_to_string("pure_nux_donut.nux").unwrap();
    let asm = compile_to_asm_source(&source).unwrap();
    fs::write("pure_nux_donut.asm", asm).unwrap();
}
