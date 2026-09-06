import re
import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

parts = content.split("fn parse_primary")
if len(parts) == 2:
    expr_part = parts[1]
    
    for op in ["sec_login", "syscall", "peek_ptr", "arr_new"]:
        expr_part = re.sub(
            rf'if part1 == "{op}".*?return Ok\(\(\)\);',
            lambda m: m.group(0).replace("return Ok(());", "return Ok(Type::Int);"),
            expr_part,
            flags=re.DOTALL
        )
        
    content = parts[0] + "fn parse_primary" + expr_part

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

print("Fixed the 4 parse_primary returns!")
