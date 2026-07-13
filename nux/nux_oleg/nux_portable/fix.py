import os, re
path = "E:/nux/Nux_Lang/nux/nux_oleg/nux_portable/lib/std"
for file in os.listdir(path):
    if not file.endswith(".nuxel"): continue
    p = os.path.join(path, file)
    with open(p, "r") as f: content = f.read()
    content = re.sub(r"if\s+([^{]+?)\s*\{", r"if (\1) {", content)
    content = re.sub(r"while\s+([^{]+?)\s*\{", r"while (\1) {", content)
    with open(p, "w") as f: f.write(content)
print("Fixed syntax in stdlib")
