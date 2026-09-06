import os
import sys

visited = set()

def process_imports(path):
    path = os.path.abspath(path)
    if path in visited:
        return ""
    visited.add(path)
    
    try:
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {path}: {e}")
        return ""
        
    out = ""
    for line in content.splitlines():
        trimmed = line.strip()
        if trimmed.startswith("@hardware"):
            continue
        if trimmed.startswith("import ") and trimmed.endswith(";"):
            raw_import = trimmed.split('"')[1]
            import_path_str = raw_import if raw_import.endswith(".nux") or raw_import.endswith(".nuxel") or raw_import.endswith(".nuxg") else raw_import.replace(".", "/")
            
            exts = [import_path_str] if (import_path_str.endswith(".nux") or import_path_str.endswith(".nuxel") or import_path_str.endswith(".nuxg")) else [f"{import_path_str}.nuxel", f"{import_path_str}.nuxg", f"{import_path_str}.nux"]
            
            resolved_path = None
            parent = os.path.dirname(path)
            cwd = os.getcwd()
            
            for ext in exts:
                # 1. relative to current file
                p1 = os.path.join(parent, ext)
                if os.path.exists(p1): resolved_path = p1; break
                # 2. relative to lib
                p2 = os.path.join(parent, "lib", ext)
                if os.path.exists(p2): resolved_path = p2; break
                # 3a. cwd
                p3a = os.path.join(cwd, ext)
                if os.path.exists(p3a): resolved_path = p3a; break
                # 3b. cwd/lib
                p3b = os.path.join(cwd, "lib", ext)
                if os.path.exists(p3b): resolved_path = p3b; break
                
            if resolved_path:
                out += process_imports(resolved_path)
            else:
                print(f"Import not found: {import_path_str} from {path}")
            continue
        out += line + "\n"
    return out

res = process_imports(sys.argv[1])
with open("test_import_out.txt", "w", encoding='utf-8') as f:
    f.write(res)
