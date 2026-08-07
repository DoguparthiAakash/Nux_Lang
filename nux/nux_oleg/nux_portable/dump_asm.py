import sys

def patch():
    with open('src/main.rs', 'r') as f:
        content = f.read()

    old = "let bytecode = compiler::compile_to_bytecode(&asm_str)"
    new = "println!(\"RAW ASM:\\n{}\\n\", asm_str);\nlet bytecode = compiler::compile_to_bytecode(&asm_str)"
    
    content = content.replace(old, new)
    
    with open('src/main.rs', 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch()
