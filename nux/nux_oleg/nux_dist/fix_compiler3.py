import re
import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

# The file contains methods `fn parse_statement_impl` and `fn parse_primary`
# Let's split by `fn parse_primary`
parts = content.split("fn parse_primary")
if len(parts) == 2:
    stmt_part = parts[0]
    expr_part = parts[1]
    
    # In stmt_part, replace Ok(Type::Int) with Ok(()) for quantum operations
    for op in ["q_alloc", "q_h", "q_x", "q_z", "q_cx"]:
        # Find the block for this op and replace its return
        stmt_part = re.sub(
            rf'if part1 == "{op}".*?return Ok\(.*?\);',
            lambda m: m.group(0).replace("return Ok(Type::Int);", "return Ok(());").replace("return Ok(Type::Void);", "return Ok(());"),
            stmt_part,
            flags=re.DOTALL
        )
        
    # In expr_part, replace Ok(()) with Ok(Type::Int) for quantum operations
    for op in ["q_alloc", "q_h", "q_x", "q_z", "q_cx"]:
        expr_part = re.sub(
            rf'if part1 == "{op}".*?return Ok\(.*?\);',
            lambda m: m.group(0).replace("return Ok(());", "return Ok(Type::Int);").replace("return Ok(Type::Void);", "return Ok(Type::Int);"),
            expr_part,
            flags=re.DOTALL
        )
        
    content = stmt_part + "fn parse_primary" + expr_part

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

print("Fixed return types robustly!")
