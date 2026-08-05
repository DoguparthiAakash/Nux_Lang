with open('compiler/lexer.nux') as f:
    lines = f.read().split('\n')
depth = 0
for i, line in enumerate(lines):
    open_b = line.count('{')
    close_b = line.count('}')
    depth += open_b - close_b
    print(f'{i+1:03d} {depth:02d} {line[-60:]}')
