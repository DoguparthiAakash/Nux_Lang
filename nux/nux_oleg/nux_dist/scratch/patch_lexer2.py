with open('src/lexer.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Add to Token enum
code = code.replace('Import,', 'Import, From, Use, InlineLang(String),')

# Add to keyword matching
code = code.replace('"import" => Token::Import,', '"import" => Token::Import,\n                            "from" => Token::From,\n                            "use" => Token::Use,')

with open('src/lexer.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print('Lexer patched.')
