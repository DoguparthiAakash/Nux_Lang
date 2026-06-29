import re
with open(src/compiler.rs, r) as f:
    content = f.read()
content = re.sub(
    rlet typ = if i == 0 && !class_prefix\.is_empty\(\) \{
\s+Type::Int
\s+\} else \{
\s+Type::Int
\s+\};,
    rlet typ = if i == 0 && !class_prefix.is_empty() { Type::Class(class_prefix.to_string()) } else { Type::Int };,
    content
)
with open(src/compiler.rs, w) as f:
    f.write(content)
