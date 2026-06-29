import os

compiler_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(compiler_path, "r", encoding="utf-8") as f:
    content = f.read()

# We need to remove the block that returns `()` from `parse_expression`
bad_block = """                            Token::NetListen | Token::NetAccept | Token::NetRead | Token::NetWrite | Token::NetClose | Token::NetListenTls => {
                    self.parse_expression(out)?;
                    out.push_str("POP\\n");
                    if self.current_token == Token::SemiColon { self.advance(); }
                },
"""

# There are two occurrences of this bad block if `replace` replaced all `Token::ImgGet => {`
# The one in `parse_statement` is correct! It needs to return `()`.
# The one in `parse_expression` is wrong! It causes the error.

# How to identify the one in parse_statement vs parse_expression?
# The one in parse_statement is before `Token::ImgGet => {` which does:
# `Token::ImgGet => { self.advance(); if self.current_token != Token::LParen { return self.error("Expected (".to_string()); }` and ends with `out.push_str("OP_IMG_GET\\nPOP\\n");`

# The one in parse_expression is before `Token::ImgGet => {` which does:
# `Token::ImgGet => { self.advance(); if self.current_token!=Token::LParen{return self.error("(".to_string());} ... Ok(Type::Int) },`

# So we just find the specific occurrence and remove it.

content_lines = content.splitlines()
out_lines = []

skip = False
for i, line in enumerate(content_lines):
    if "Token::NetListen | Token::NetAccept | Token::NetRead | Token::NetWrite | Token::NetClose | Token::NetListenTls => {" in line:
        # Check if the next few lines look like the bad block, AND it's inside parse_expression.
        # How to know if it's inside parse_expression? Look ahead a few lines for `Ok(Type::Int)` in ImgGet
        is_parse_expr = False
        for j in range(i, min(i+10, len(content_lines))):
            if "Token::ImgGet => {" in content_lines[j] and "Ok(Type::Int)" in content_lines[j]:
                is_parse_expr = True
                break
        
        if is_parse_expr:
            # We are at the bad block, skip the next 4 lines
            skip = True
            lines_to_skip = 5
            continue

    if skip:
        lines_to_skip -= 1
        if lines_to_skip == 0:
            skip = False
        continue
    
    out_lines.append(line)

with open(compiler_path, "w", encoding="utf-8") as f:
    f.write("\n".join(out_lines) + "\n")

print("Compiler duplicates fixed")
