import sys

lines = open('nux_errors.txt', 'r', encoding='utf-16le').readlines()
for l in lines[30:50]:
    sys.stdout.buffer.write(l.encode('utf-8'))
