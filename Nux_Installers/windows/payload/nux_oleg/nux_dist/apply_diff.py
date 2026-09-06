lines = open('diff.txt', encoding='utf-8').read().splitlines()
out = []
started = False
for l in lines:
    if started:
        if l.startswith('+'):
            out.append(l[1:])
        elif l.startswith(' '):
            out.append(l[1:])
        # ignore '-' lines
    elif l.startswith('+++'):
        started = True

with open('compiler/lexer.nux', 'w', encoding='utf-8') as f:
    f.write('\n'.join(out) + '\n')
