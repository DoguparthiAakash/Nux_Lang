import os
import re

std_dir = 'E:/nux/Nux_Lang/nux/nux_oleg/nux_dist/lib/std'

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # fn -> func
    content = re.sub(r'\bfn\b', 'func', content)
    # let -> var
    content = re.sub(r'\blet\b', 'var', content)

    with open(filepath, 'w') as f:
        f.write(content)

for filename in os.listdir(std_dir):
    if filename.endswith('.nux'):
        process_file(os.path.join(std_dir, filename))

print("Done fixing stdlib keywords.")
