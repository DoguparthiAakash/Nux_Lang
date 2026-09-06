import os

path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path, "r", encoding="utf-8") as f:
    content = f.read()

parts = content.split("fn parse_primary")
if len(parts) == 2:
    stmt_part = parts[0]
    expr_part = parts[1]
    
    # stmt_part is parse_statement_impl and everything before.
    # We mistakenly replaced `return Ok(());` with `return Ok(Type::Int);` everywhere in fix_quantum.py!
    # So we change it back.
    stmt_part = stmt_part.replace("return Ok(Type::Int);", "return Ok(());")
    
    # In expr_part, we mistakenly did the same, but expr_part SHOULD return Ok(Type::XXX), but wait, what if expr_part had some Ok(())?
    # expr_part didn't have Ok(()) except the ones I injected. So expr_part having Ok(Type::Int) is actually correct!
    
    content = stmt_part + "fn parse_primary" + expr_part

with open(path, "w", encoding="utf-8") as f:
    f.write(content)

print("Fixed the global replace disaster!")
