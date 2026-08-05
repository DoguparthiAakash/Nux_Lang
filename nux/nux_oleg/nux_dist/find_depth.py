import sys

lines = open('compiler/lexer.nux', 'r').readlines()
b = 0
start_line = 629
found_start = False

for i in range(start_line - 1, len(lines)):
    l = lines[i]
    if '{' in l or '}' in l:
        b += l.count('{')
        b -= l.count('}')
        if b == 0:
            print(f"Brace depth hit 0 at line {i+1}: {l.strip()}")
            break
