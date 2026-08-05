import sys

lines = open('compiler/lexer.nux', 'r').readlines()
b = 0
for i, l in enumerate(lines[628:800]):
    if '{' in l or '}' in l:
        b += l.count('{')
        b -= l.count('}')
        print(f"L{i+629}: b={b} {l.strip()}")
