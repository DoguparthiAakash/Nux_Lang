import os
def load_imports(path, loaded=None):
    if loaded is None: loaded = set()
    if path in loaded: return ''
    loaded.add(path)
    res = []
    try:
        with open(path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        for line in lines:
            if line.startswith('import '):
                p = line.split('\"')[1]
                res.append(load_imports(p, loaded))
            else:
                res.append(line)
        return ''.join(res)
    except Exception as e:
        return ''
out = load_imports('test_3d_dial.nux')
with open('debug_out.nux', 'w', encoding='utf-8') as f: f.write(out)
for i, line in enumerate(out.split('\n'), 1):
    if 150 <= i <= 170:
        print(f'{i:03d} | {line}')
