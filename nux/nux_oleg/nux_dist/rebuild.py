import os
os.system('git checkout compiler/lexer.nux')
lines = open('diff.txt', encoding='utf-8').read().splitlines()
out = open('compiler/lexer.nux', 'w', encoding='utf-8')
started = False
for l in lines:
    if started:
        if l.startswith('+'):
            out.write(l[1:] + '\n')
    elif l.startswith('+++'):
        started = True
out.close()

# Remove the last 48 lines (the tests)
lines = open('compiler/lexer.nux', encoding='utf-8').readlines()
open('compiler/lexer.nux', 'w', encoding='utf-8').writelines(lines[:-48])

# Append the missing lines
with open('compiler/lexer.nux', 'a', encoding='utf-8') as f:
    f.write('    if (kind == TK_AMPERSAND) { print("AMPERSAND"); return 0; }\n')
    f.write('    if (kind == TK_PIPE) { print("PIPE"); return 0; }\n')
    f.write('    if (kind == TK_TILDE) { print("TILDE"); return 0; }\n')
    f.write('    print("?");\n')
    f.write('    print(kind);\n')
    f.write('    return 0;\n')
    f.write('}\n')
