with open('src/compiler.rs', 'r') as f:
    for i, l in enumerate(f):
        if l.strip().startswith('fn ') or l.strip().startswith('pub fn '):
            print(f"{i}: {l.strip()}")
