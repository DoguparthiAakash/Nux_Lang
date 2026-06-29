import sys
import re

def patch():
    with open('src/compiler.rs', 'r') as f:
        content = f.read()
    
    # We want to replace 'if self.current_token == Token::Eq {' inside Token::Dot branch with 'self.advance();\n                        if self.current_token == Token::Eq {'
    
    pattern = r'(let member = match &self\.current_token \{ Token::Identifier\(s\) => s\.clone\(\), _ => return self\.error\(\"Expected member name\"\.to_string\(\)\) \};\s+)if self\.current_token == Token::Eq \{'
    replacement = r'\1self.advance();\n                        if self.current_token == Token::Eq {'
    new_content = re.sub(pattern, replacement, content)
    
    if new_content == content:
        print('NO CHANGES MADE. Regex failed.')
    else:
        with open('src/compiler.rs', 'w') as f:
            f.write(new_content)
        print('Patch applied successfully.')

patch()
