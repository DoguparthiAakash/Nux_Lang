
fn main() {
    let source = std::fs::read_to_string("test_3d_dial.nux").unwrap();
    let asm = nux::high_level::compile_to_asm_source(&source).unwrap();
    std::fs::write("real_asm.txt", asm).unwrap();
}

