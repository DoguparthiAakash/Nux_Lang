import sys
import re

def patch():
    with open('src/compiler.rs', 'r') as f:
        content = f.read()
    
    # We want to replace 'self.advance();\n                 if self.current_token == Token::Eq {'
    new_content = re.sub(r'self\.advance\(\);\s+if self\.current_token == Token::Eq \{', r'if self.current_token == Token::Eq {', content)
    
    if new_content == content:
        print('NO CHANGES MADE. Regex failed.')
    else:
        with open('src/compiler.rs', 'w') as f:
            f.write(new_content)
        print('Patch applied successfully.')

patch()
