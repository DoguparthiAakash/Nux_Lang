import re

with open('src/high_level.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Fix 1: OP_SWAP before POKE in obj.field = expr
code = re.sub(
    r'(out\.push_str\("POKE\\n"\);\s*\n\s*\} else if self\.current_token == Token::LParen \{)',
    r'out.push_str("OP_SWAP\\nPOKE\\n");\n                          \n                      } else if self.current_token == Token::LParen {',
    code
)

# Fix 2: cname_method in obj.method()
code = re.sub(
    r'(\/\/ Resolve "this"\/object again\s*\n\s*let \(loc, typ\) = if let Some\(r\) = self\.resolve_var\(&part1\) \{ r \} else \{ return self\.error\(format\!\("Undefined variable \'\{\}\'", part1\)\); \};\s*\n\s*)(.*?\n\s*match loc \{)',
    r'\1let cname = if let Type::Class(c) = typ { c.clone() } else { return self.error(format!("Cannot call method \'{}\' on variable \'{}\' of non-class type", member, part1)); };\n\2',
    code,
    flags=re.DOTALL
)

code = re.sub(
    r'(out\.push_str\(&format\!\("CALL \{\}_\{\} \{\}\\nPOP\\n", )part1, member',
    r'\1cname, member',
    code
)

with open('src/high_level.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print("Patched again.")
