#!/usr/bin/env python3
"""
╔══════════════════════════════════════════════════════════════════╗
║  Nux Bootstrap — nux_oleg/bootstrap/boot.py                     ║
║  TEMPORARY. Used ONLY to compile lib/ovm/ovm.nux on first build. ║
║  Once ovm.nxb exists, this file is never used again.            ║
║                                                                  ║
║  Self-hosting plan:                                              ║
║    Step 1: boot.py  parses + runs ovm.nux  (you are here)       ║
║    Step 2: ovm.nux  compiles itself → ovm.nxb                   ║
║    Step 3: ovm.nxb  runs all future .nux/.nxb files             ║
║    Step 4: boot.py  is archived, never needed again              ║
║                                                                  ║
║  The real OVM is:  lib/ovm/ovm.nux  (written in Nux)            ║
╚══════════════════════════════════════════════════════════════════╝
Usage (bootstrap only):
    python3 nux_oleg/bootstrap/boot.py <file.nux|file.nxb>
"""
import sys, os, re, subprocess, tempfile, gzip, zlib, textwrap
try:
    import xml.etree.ElementTree as ET
except ImportError:
    ET = None

# ─── Bonfort.toml reader ─────────────────────────────────────────────────────
def load_toml(path):
    """Minimal TOML key=value parser (no deps required)."""
    cfg = {}
    section = 'root'
    if not os.path.exists(path): return cfg
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'): continue
            if line.startswith('['):
                section = line.strip('[]').strip(); cfg.setdefault(section, {}); continue
            if '=' in line:
                k, _, v = line.partition('=')
                k = k.strip(); v = v.strip().strip('"').strip("'")
                if section == 'root': cfg[k] = v
                else: cfg.setdefault(section, {})[k] = v
    return cfg

# ─── imports.xml reader ──────────────────────────────────────────────────────
def load_imports_xml(xml_path, src_file_path=None):
    """
    Returns: {
      'nux_libs':   [{'pkg': str, 'as': str, 'auto': bool}],
      'lag_bindings': [{'lang':str,'version':str,'alias':str,'from_imports':[str]}],
      'packages':   [{'name':str,'version':str,'as':str}],
      'file_override': None | same structure for a specific file,
      'settings': dict
    }
    """
    result = {'nux_libs': [], 'lag_bindings': [], 'packages': [], 'settings': {}}
    if not os.path.exists(xml_path) or ET is None:
        return result
    try:
        tree = ET.parse(xml_path)
        root = tree.getroot()
    except Exception as e:
        print(f"[OVM] WARNING: could not parse {xml_path}: {e}"); return result

    # <nux-libs>
    for lib in root.findall('./nux-libs/import'):
        result['nux_libs'].append({
            'pkg':  lib.get('pkg',''),
            'as':   lib.get('as',''),
            'auto': lib.get('auto','false').lower() == 'true'
        })

    # <lag-bindings>
    for lang_el in root.findall('./lag-bindings/lang'):
        froms = [fi.get('module','') for fi in lang_el.findall('from-import')]
        result['lag_bindings'].append({
            'lang':    lang_el.get('name',''),
            'version': lang_el.get('version',''),
            'alias':   lang_el.get('alias',''),
            'from_imports': froms
        })

    # <packages>
    for pkg in root.findall('./packages/package'):
        result['packages'].append({
            'name':    pkg.get('name',''),
            'version': pkg.get('version',''),
            'as':      pkg.get('as','')
        })

    # <settings>
    for s in root.findall('./settings/compiler/*'):
        result['settings'][s.tag] = s.text
    for s in root.findall('./settings/ovm/*'):
        result['settings']['ovm_'+s.tag] = s.text

    # <file-overrides> — apply only if src_file_path matches
    if src_file_path:
        fname = os.path.basename(src_file_path)
        frel  = os.path.relpath(src_file_path)
        for fo in root.findall('./file-overrides/file'):
            fp = fo.get('path','')
            if fp == fname or fp == frel:
                inherit = fo.get('inherit','true').lower() == 'true'
                override = {'nux_libs':[], 'lag_bindings':[], 'packages':[], 'settings':{}}
                for lib in fo.findall('import'):
                    override['nux_libs'].append({'pkg':lib.get('pkg',''),'as':lib.get('as',''),'auto':False})
                for lang_el in fo.findall('lang'):
                    froms = [fi.get('module','') for fi in lang_el.findall('from-import')]
                    override['lag_bindings'].append({'lang':lang_el.get('name',''),'version':lang_el.get('version',''),'alias':lang_el.get('alias',''),'from_imports':froms})
                if not inherit:
                    return override  # file-specific only, ignore global
                # Merge: file overrides extend global
                result['nux_libs']    += override['nux_libs']
                result['lag_bindings']+= override['lag_bindings']
    return result

def apply_xml_config(cfg, verbose=True):
    """Pre-populate LAG bindings and imports from XML config."""
    for b in cfg.get('lag_bindings', []):
        if not b['lang'] or not b['alias']: continue
        rt = find_runtime(b['lang'])
        if rt:
            bindings[b['alias']] = {
                'lang': b['lang'], 'version': b['version'],
                'runtime': rt, 'imports': b['from_imports'][:]
            }
            if verbose: print(f"[XML] Auto-bind {b['alias']} → {b['lang']} {b['version']} ({rt})")
        else:
            if verbose: print(f"[XML] WARNING: {b['lang']} runtime not found. Run: bonfort lang add {b['lang']}")
    if verbose and cfg.get('nux_libs'):
        auto = [l['pkg'] for l in cfg['nux_libs'] if l.get('auto')]
        if auto: print(f"[XML] Auto-imports: {', '.join(auto)}")

# ─── NXB decompression ───────────────────────────────────────────────────────
def load_source(path):
    if path.endswith('.nxb'):
        with open(path, 'rb') as f: data = f.read()
        magic4 = data[:4]
        # Zstd magic: FD 2F B5 28
        if magic4 == b'\xfd\x2f\xb5\x28' or magic4[:4] == bytes([0x28,0xb5,0x2f,0xfd]):
            import subprocess as sp
            r = sp.run(['zstd','-d','--stdout',path], capture_output=True)
            return r.stdout.decode()
        # Gzip magic: 1F 8B
        if data[:2] == b'\x1f\x8b':
            return gzip.decompress(data).decode()
        # zlib
        try: return zlib.decompress(data).decode()
        except Exception: return data.decode()
    with open(path) as f:
        return f.read()


# ─── LAG state ───────────────────────────────────────────────────────────────
bindings   = {}   # alias -> {lang, version, runtime, imports}
var_blocks = {}   # varname -> {code, alias, success, output}

LANG_RUNTIMES = {
    'python': ['python3','python'],
    'rust':   ['rustc'],
    'c':      ['gcc','clang','cc'],
    'c++':    ['g++','clang++'],
    'java':   ['java'],
    'zig':    ['zig'],
    'go':     ['go'],
    'js':     ['node'],
    'lua':    ['lua5.4','lua'],
    'ruby':   ['ruby'],
}
EXT = {'python':'.py','rust':'.rs','c':'.c','c++':'.cpp',
       'java':'.java','zig':'.zig','go':'.go','js':'.js','lua':'.lua','ruby':'.rb'}

def find_runtime(lang):
    for name in LANG_RUNTIMES.get(lang, []):
        path = subprocess.run(['which',name], capture_output=True, text=True).stdout.strip()
        if path: return path
    return None

def run_foreign(lang, code, imports):
    rt = find_runtime(lang)
    if not rt:
        return False, f"Runtime not found: {lang}\nRun: bonfort lang add {lang}"
    # Only prepend imports not already in code
    extra = [m for m in imports if f'import {m}' not in code and m not in code]
    prefix = '\n'.join(f'import {m}' for m in extra) + '\n' if extra else ''
    full_code = prefix + code

    if lang in ('python','js','lua','ruby'):
        # Interpreted: write temp file, run
        ext = EXT.get(lang, '.txt')
        with tempfile.NamedTemporaryFile(suffix=ext, delete=False, mode='w') as f:
            f.write(full_code); fname = f.name
        r = subprocess.run([rt, fname], capture_output=True, text=True)
        os.unlink(fname)
        out = r.stdout.strip()
        err = r.stderr.strip() if r.returncode != 0 else ''
        return r.returncode == 0, out or err
    elif lang in ('rust','c','c++','zig','go'):
        # Compiled: write source, compile, run
        ext = EXT[lang]
        with tempfile.NamedTemporaryFile(suffix=ext, delete=False, mode='w') as f:
            f.write(full_code); src = f.name
        bin_path = src + '.out'
        if lang == 'rust':
            cr = subprocess.run([rt, src, '-o', bin_path], capture_output=True, text=True)
        elif lang in ('c','c++'):
            cr = subprocess.run([rt, src, '-o', bin_path], capture_output=True, text=True)
        elif lang == 'zig':
            cr = subprocess.run([rt, 'run', src], capture_output=True, text=True)
            os.unlink(src)
            return cr.returncode == 0, cr.stdout.strip() or cr.stderr.strip()
        elif lang == 'go':
            cr = subprocess.run([rt, 'run', src], capture_output=True, text=True)
            os.unlink(src)
            return cr.returncode == 0, cr.stdout.strip() or cr.stderr.strip()
        else:
            cr = subprocess.run([rt, src], capture_output=True, text=True)
        if cr.returncode != 0:
            os.unlink(src)
            return False, cr.stderr.strip()
        r = subprocess.run([bin_path], capture_output=True, text=True)
        os.unlink(src)
        if os.path.exists(bin_path): os.unlink(bin_path)
        return r.returncode == 0, r.stdout.strip() or r.stderr.strip()
    return False, f"Unsupported lang: {lang}"

# ─── Parser ──────────────────────────────────────────────────────────────────
def execute(source):
    lines = source.splitlines()
    i = 0
    nux_vars = {}   # name -> value

    while i < len(lines):
        line = lines[i].strip()

        # Skip comments and blank lines
        if not line or line.startswith('#'):
            i += 1; continue

        # import nux.lag.python.3.13.7;  (semicolon required or optional)
        m = re.match(r'import nux\.lag\.(\w+)\.([\d.]+)\s*;?', line)
        if m:
            lang, ver = m.group(1), m.group(2)
            print(f"[OVM] LAG import: {lang} {ver}")
            i += 1; continue

        # from python import os;  (semicolon required or optional)
        m = re.match(r'from (\w+) import (.+?)\s*;?$', line)
        if m:
            lang, mod = m.group(1), m.group(2).strip()
            for alias, b in bindings.items():
                if b['lang'] == lang:
                    b['imports'].append(mod)
            i += 1; continue

        # nux.lag python.3.13.7 = v1;  (semicolon required or optional)
        m = re.match(r'nux\.lag\s+(\w+)\.([\d.]+)\s*=\s*(\w+)\s*;?', line)
        if m:
            lang, ver, alias = m.group(1), m.group(2), m.group(3)
            bindings[alias] = {'lang': lang, 'version': ver,
                               'runtime': find_runtime(lang), 'imports': []}
            rt = bindings[alias]['runtime']
            print(f"[OVM] Bound {alias} → {lang} {ver} ({rt or 'NOT FOUND'})")
            i += 1; continue

        # var = new var;  or  var1 = new var;
        m = re.match(r'(\w+)\s*=\s*new\s+var', line)
        if m:
            vname = m.group(1)
            var_blocks[vname] = {'code':'','alias':'','success':False,'output':''}
            i += 1; continue

        # varname.alias { <foreign code> }
        m = re.match(r'(\w+)\.(\w+)\s*\{', line)
        if m:
            vname, alias = m.group(1), m.group(2)
            # Collect body until matching }
            depth = 1; code_lines = []
            i += 1
            while i < len(lines) and depth > 0:
                l = lines[i]
                depth += l.count('{') - l.count('}')
                if depth > 0: code_lines.append(l)
                i += 1
            code = textwrap.dedent('\n'.join(code_lines))
            if alias in bindings:
                b = bindings[alias]
                ok, out = run_foreign(b['lang'], code, b['imports'])
                if vname in var_blocks:
                    var_blocks[vname] = {'code':code,'alias':alias,'success':ok,'output':out}
                print(f"[OVM] {vname}.{alias} → {'OK' if ok else 'FAIL'}")
                if out: print(out)
            else:
                print(f"[OVM] ERROR: unknown binding '{alias}'")
            continue

        # if(var == true) { ... } else { ... }
        m = re.match(r'if\s*\(\s*(\w+)\s*==\s*true\s*\)\s*\{', line)
        if m:
            vname = m.group(1)
            # Collect then block
            depth = 1; then_lines = []
            i += 1
            while i < len(lines) and depth > 0:
                l = lines[i].strip()
                depth += l.count('{') - l.count('}')
                if depth > 0: then_lines.append(l)
                i += 1
            # Check for else
            else_lines = []
            if i < len(lines) and lines[i].strip().startswith('else'):
                i += 1
                depth = 1
                while i < len(lines) and depth > 0:
                    l = lines[i].strip()
                    depth += l.count('{') - l.count('}')
                    if depth > 0: else_lines.append(l)
                    i += 1
            cond = var_blocks.get(vname, {}).get('success', False)
            branch = then_lines if cond else else_lines
            for bl in branch:
                m2 = re.match(r'print(?:ln)?\((\w+)\)', bl)
                if m2:
                    bv = m2.group(1)
                    info = var_blocks.get(bv, {})
                    print(f"[NUX] {bv}: success={info.get('success')}, output={info.get('output','')}")
            continue

        # println("...")
        m = re.match(r'println\("(.+)"\)', line)
        if m: print(m.group(1)); i += 1; continue

        # print("...")
        m = re.match(r'print\("(.+)"\)', line)
        if m: print(m.group(1), end=''); i += 1; continue

        i += 1

# ─── Main ────────────────────────────────────────────────────────────────────
if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: ovm.py <file.nux|file.nxb>"); sys.exit(1)
    path = sys.argv[1]
    if not os.path.exists(path):
        print(f"ovm: file not found: {path}"); sys.exit(1)
    try:
        src = load_source(path)
    except Exception as e:
        print(f"ovm: failed to load {path}: {e}"); sys.exit(1)

    # ── 1. Load Bonfort.toml (if present in cwd or file's dir) ──────────────
    file_dir = os.path.dirname(os.path.abspath(path))
    for search_dir in [os.getcwd(), file_dir]:
        toml_path = os.path.join(search_dir, 'Bonfort.toml')
        if not os.path.exists(toml_path):
            toml_path = os.path.join(search_dir, 'bonfort.toml')
        if os.path.exists(toml_path):
            toml = load_toml(toml_path)
            xml_ref = toml.get('package', {}).get('imports') if isinstance(toml.get('package'), dict) else toml.get('imports', 'imports.xml')
            print(f"[OVM] Bonfort.toml: {toml_path}")
            break
    else:
        xml_ref = 'imports.xml'

    # ── 2. Check for @imports("file") override at top of source ─────────────
    for raw_line in src.splitlines()[:10]:
        m = re.match(r'@imports\("([^"]+)"\)\s*;?', raw_line.strip())
        if m:
            xml_ref = m.group(1)
            print(f"[OVM] File-level @imports override: {xml_ref}")
            break

    # ── 3. Resolve and load imports.xml ─────────────────────────────────────
    for search_dir in [os.getcwd(), file_dir]:
        xml_path = os.path.join(search_dir, xml_ref)
        if os.path.exists(xml_path):
            cfg = load_imports_xml(xml_path, path)
            apply_xml_config(cfg)
            break
    else:
        if xml_ref != 'imports.xml':
            print(f"[OVM] WARNING: imports file not found: {xml_ref}")

    # ── 4. Execute source ────────────────────────────────────────────────────
    execute(src)
