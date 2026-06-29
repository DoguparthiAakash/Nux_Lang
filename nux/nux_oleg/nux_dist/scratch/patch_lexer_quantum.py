import re

with open('src/lexer.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# Add to Token enum
code = re.sub(
    r'(NetListen, NetAccept, NetRead, NetWrite, NetClose, NetListenTls,)',
    r'\1\n    QAlloc, QH, QX, QZ, QCx, QMeasure,',
    code
)

# Add to lex_identifier match
code = re.sub(
    r'("fs_read" => Token::FsRead,)',
    r'"q_alloc" => Token::QAlloc,\n            "q_h" => Token::QH,\n            "q_x" => Token::QX,\n            "q_z" => Token::QZ,\n            "q_cx" => Token::QCx,\n            "q_measure" => Token::QMeasure,\n            \1',
    code
)

with open('src/lexer.rs', 'w', encoding='utf-8') as f:
    f.write(code)

print("Lexer patched.")
