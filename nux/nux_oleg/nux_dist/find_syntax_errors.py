import sys

lines = open('nux_errors.txt', 'r', encoding='utf-16le').readlines()
for i, l in enumerate(lines):
    if 'Unexpected' in l or 'Expected' in l or 'Unknown' in l:
        print(f"L{i}: {l.strip()}")
