import re

with open('src/compiler.rs', 'r') as f:
    text = f.read()

# First, extract the intrinsics block
match = re.search(r'Token::Identifier\(name\) => \{\s+let func_name = name\.clone\(\);\s+// Check for intrinsic functions that can be used as expressions\s+if func_name == "sec_login" \{(.*?)\s+// Fall through to default error', text, re.DOTALL)

if not match:
    print("Could not find intrinsics block")
    exit(1)

intrinsics_code = match.group(1)
intrinsics_code = 'if part1 == "sec_login" {' + intrinsics_code.replace('func_name', 'part1')

# Remove the second Identifier block entirely
text = re.sub(r'Token::Identifier\(name\) => \{\s+let func_name = name\.clone\(\);\s+// Check for intrinsic functions that can be used as expressions\s+if func_name == "sec_login" \{.*?\s+// Fall through to default error\s+return self\.error\(self\.format_unexpected_token\(&self\.current_token, "Unexpected token in expression:"\)\);\s+\},\s+', '', text, flags=re.DOTALL)

# Insert the intrinsics code into the first Identifier block
first_id_pattern = r'(Token::Identifier\(name\) => \{\s+let part1 = name\.clone\(\);\s+self\.advance\(\);\s+)'
replacement = r'\1' + '\n                // Intrinsics\n                ' + intrinsics_code.replace('\\', '\\\\') + ' else '

text = re.sub(first_id_pattern, replacement, text, count=1)

with open('src/compiler.rs', 'w') as f:
    f.write(text)

print("Refactor complete")
