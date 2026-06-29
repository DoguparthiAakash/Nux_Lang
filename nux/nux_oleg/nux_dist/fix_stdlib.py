import os
import re

std_dir = 'E:/nux/Nux_Lang/nux/nux_oleg/nux_dist/lib/std'

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # var name: type; => var name;
    # var name: type = val; => var name = val;
    content = re.sub(r'var\s+([a-zA-Z0-9_]+)\s*:\s*[a-zA-Z0-9_]+\s*(;|=)', r'var \1 \2', content)

    # name: type; inside class (like size: int;) => var name;
    # Wait, we can just replace `\s+([a-zA-Z0-9_]+)\s*:\s*[a-zA-Z0-9_]+\s*;`
    content = re.sub(r'^(\s*)([a-zA-Z0-9_]+)\s*:\s*[a-zA-Z0-9_]+\s*;', r'\1var \2;', content, flags=re.MULTILINE)

    # Function arguments: func draw(gfx: Graphics, v1: Vector3)
    # We can match `identifier: type` and replace with `identifier`
    # But only inside parentheses!
    def repl_args(match):
        args = match.group(1)
        args_cleaned = re.sub(r'([a-zA-Z0-9_]+)\s*:\s*[a-zA-Z0-9_]+', r'\1', args)
        return f"({args_cleaned})"

    content = re.sub(r'\(([^)]*)\)', repl_args, content)

    # return types: -> int { => {
    content = re.sub(r'->\s*[a-zA-Z0-9_]+\s*\{', '{', content)

    with open(filepath, 'w') as f:
        f.write(content)

for filename in os.listdir(std_dir):
    if filename.endswith('.nux'):
        process_file(os.path.join(std_dir, filename))

print("Done fixing stdlib syntax.")
