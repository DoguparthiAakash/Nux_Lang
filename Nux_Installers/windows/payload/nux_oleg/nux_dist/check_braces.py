with open('compiler/lexer.nux') as f:
    lines = f.read().split('\n')
depth = 0
for i, line in enumerate(lines):
    # ignore string literals when counting, though we don't have many with braces
    # simple count:
    open_b = line.count('{')
    close_b = line.count('}')
    depth += open_b - close_b
    if depth <= 0 and i > 200:
        print(f'{i+1:03d} {depth:02d} {line}')
