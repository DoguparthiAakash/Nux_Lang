import sys

def patch_compiler():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()
        
    find_str = r'''             if let Err(e) = self.parse_statement_or_expr(out) {
                 println!("DEBUG parse_block error: {:?}", e);
                 self.errors.push(e);
                 self.synchronize();
             }'''
             
    replace_str = r'''             if let Err(e) = self.parse_statement_or_expr(out) {
                 eprintln!("DEBUG parse_block error: {:?}", e);
                 self.errors.push(e);
                 self.synchronize();
             }'''
             
    if find_str in content:
        content = content.replace(find_str, replace_str)
        print("Patched debug print in parse_block to use eprintln")
    else:
        print("Failed to patch debug print")

    with open(path, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch_compiler()
