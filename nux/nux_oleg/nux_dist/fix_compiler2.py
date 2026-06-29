import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

# First, find the two occurrences of "q_alloc".
parts = content.split('if part1 == "q_alloc" {')
if len(parts) == 3:
    # parts[0] is before the first
    # parts[1] is the first one (in parse_statement_impl). It should use Ok(())
    parts[1] = parts[1].replace('return Ok(Type::Int);', 'return Ok(());')
    
    # parts[2] is the second one (in parse_primary). It should use Ok(Type::Int)
    parts[2] = parts[2].replace('return Ok(());', 'return Ok(Type::Int);')
    
    content = 'if part1 == "q_alloc" {'.join(parts)

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

print("Fixed return types in compiler.rs!")
