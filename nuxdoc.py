#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys, io
if sys.stdout.encoding and sys.stdout.encoding.lower() != 'utf-8':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
"""
NuxDoc - Nux Language Documentation Generator
Parses .nux source files and produces beautiful HTML documentation.
Usage: python nuxdoc.py [file_or_dir] [--out <output_dir>]
"""

import sys
import os
import re
import json
import shutil
import argparse
from pathlib import Path
from datetime import datetime
from dataclasses import dataclass, field
from typing import Optional

__version__ = "0.1.0"

# ─── Data Model ────────────────────────────────────────────────────────────────

@dataclass
class DocItem:
    kind: str            # fn / class / const / let / enum / trait
    name: str
    signature: str
    doc_comment: str
    line: int
    params: list[str] = field(default_factory=list)
    returns: Optional[str] = None

@dataclass
class ModuleDoc:
    name: str
    path: str
    doc_comment: str
    items: list[DocItem] = field(default_factory=list)


# ─── Parser ────────────────────────────────────────────────────────────────────

def parse_file(path: str) -> ModuleDoc:
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    name = Path(path).stem
    items: list[DocItem] = []
    module_doc = ""
    pending_comment = ""
    module_doc_done = False
    i = 0

    while i < len(lines):
        stripped = lines[i].rstrip()

        # Accumulate doc comments
        if stripped.lstrip().startswith("///"):
            comment_line = stripped.lstrip()[3:].lstrip()
            pending_comment += comment_line + "\n"
            i += 1
            continue

        # Module-level block comment at the top
        if stripped.lstrip().startswith("//!") and not module_doc_done:
            module_doc += stripped.lstrip()[3:].lstrip() + "\n"
            i += 1
            continue

        # Detect fn
        fn_match = re.match(r'\s*(?:pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)', stripped)
        if fn_match:
            module_doc_done = True
            fn_name = fn_match.group(1)
            params_raw = fn_match.group(2)
            params = [p.strip() for p in params_raw.split(",") if p.strip()]
            items.append(DocItem(
                kind="fn", name=fn_name,
                signature=f"fn {fn_name}({params_raw})",
                doc_comment=pending_comment.strip(),
                line=i + 1, params=params))
            pending_comment = ""
            i += 1
            continue

        # Detect class
        cls_match = re.match(r'\s*(?:pub\s+)?class\s+(\w+)', stripped)
        if cls_match:
            module_doc_done = True
            cls_name = cls_match.group(1)
            items.append(DocItem(
                kind="class", name=cls_name,
                signature=f"class {cls_name}",
                doc_comment=pending_comment.strip(), line=i + 1))
            pending_comment = ""
            i += 1
            continue

        # Detect enum
        enum_match = re.match(r'\s*(?:pub\s+)?enum\s+(\w+)', stripped)
        if enum_match:
            module_doc_done = True
            e_name = enum_match.group(1)
            items.append(DocItem(
                kind="enum", name=e_name,
                signature=f"enum {e_name}",
                doc_comment=pending_comment.strip(), line=i + 1))
            pending_comment = ""
            i += 1
            continue

        # Detect const / let at module level
        const_match = re.match(r'\s*(const|let)\s+(\w+)\s*=\s*(.+)', stripped)
        if const_match:
            module_doc_done = True
            kind = const_match.group(1)
            c_name = const_match.group(2)
            c_val = const_match.group(3).rstrip(";")
            items.append(DocItem(
                kind=kind, name=c_name,
                signature=f"{kind} {c_name} = {c_val}",
                doc_comment=pending_comment.strip(), line=i + 1))
            pending_comment = ""

        # Detect trait
        trait_match = re.match(r'\s*(?:pub\s+)?trait\s+(\w+)', stripped)
        if trait_match:
            module_doc_done = True
            t_name = trait_match.group(1)
            items.append(DocItem(
                kind="trait", name=t_name,
                signature=f"trait {t_name}",
                doc_comment=pending_comment.strip(), line=i + 1))
            pending_comment = ""

        if stripped and not stripped.lstrip().startswith("//"):
            if pending_comment and not any(stripped.startswith(k) for k in ["fn ", "class ", "enum ", "trait ", "const ", "let "]):
                pending_comment = ""

        i += 1

    return ModuleDoc(name=name, path=path,
                     doc_comment=module_doc.strip(), items=items)


# ─── HTML Generator ────────────────────────────────────────────────────────────

KIND_COLORS = {
    "fn":    ("#7E5FFF", "⚡"),
    "class": ("#58A6FF", "🧩"),
    "enum":  ("#F85149", "🔴"),
    "trait": ("#39D0D8", "🔷"),
    "const": ("#FFA657", "🔒"),
    "let":   ("#3FB950", "📌"),
}

CSS = """
:root {
  --bg: #0E1117;
  --bg2: #161B22;
  --bg3: #21262D;
  --border: #30363D;
  --accent: #7E5FFF;
  --accent2: #A78BFA;
  --green: #3FB950;
  --red: #F85149;
  --yellow: #D29922;
  --blue: #58A6FF;
  --cyan: #39D0D8;
  --orange: #FFA657;
  --fg: #E6EDF3;
  --fg2: #8B949E;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body { background: var(--bg); color: var(--fg); font-family: 'Segoe UI', system-ui, sans-serif; }
a { color: var(--accent2); text-decoration: none; }
a:hover { text-decoration: underline; }

header {
  background: var(--bg2);
  border-bottom: 1px solid var(--border);
  padding: 18px 40px;
  display: flex;
  align-items: center;
  gap: 20px;
}
header h1 { font-size: 1.5rem; color: var(--fg); }
header .badge {
  background: var(--accent);
  color: #fff;
  border-radius: 20px;
  padding: 2px 12px;
  font-size: .78rem;
  font-weight: 700;
  letter-spacing: .05em;
}

.layout { display: flex; min-height: calc(100vh - 68px); }

nav {
  width: 240px; min-width: 200px;
  background: var(--bg2);
  border-right: 1px solid var(--border);
  padding: 24px 0;
  position: sticky; top: 0; height: 100vh; overflow-y: auto;
}
nav .nav-section { padding: 8px 20px; font-size: .7rem; color: var(--fg2);
  text-transform: uppercase; letter-spacing: .1em; font-weight: 700; margin-top: 12px; }
nav a { display: block; padding: 6px 20px; color: var(--fg2); font-size: .88rem; }
nav a:hover { color: var(--fg); background: var(--bg3); text-decoration: none; }
nav .kind-icon { margin-right: 6px; }

main { flex: 1; padding: 40px 60px; max-width: 960px; }

.module-title { font-size: 2rem; font-weight: 700; margin-bottom: 8px; }
.module-path { color: var(--fg2); font-size: .85rem; font-family: Consolas, monospace; margin-bottom: 20px; }
.module-doc { color: var(--fg); line-height: 1.7; margin-bottom: 36px;
  background: var(--bg2); border-left: 3px solid var(--accent);
  padding: 16px 20px; border-radius: 0 8px 8px 0; }

.item {
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: 10px;
  margin-bottom: 24px;
  overflow: hidden;
}
.item-header {
  display: flex; align-items: center; gap: 12px;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg3);
}
.item-kind {
  font-size: .7rem; font-weight: 800; padding: 3px 10px;
  border-radius: 4px; text-transform: uppercase; letter-spacing: .06em;
}
.item-name { font-family: Consolas, monospace; font-size: 1.05rem; font-weight: 700; }
.item-line { margin-left: auto; color: var(--fg2); font-size: .8rem; }
.item-body { padding: 16px 20px; }
.item-sig {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 12px 16px;
  font-family: Consolas, monospace;
  font-size: .9rem;
  margin-bottom: 14px;
  color: var(--accent2);
}
.item-doc { color: var(--fg); line-height: 1.7; font-size: .93rem; }
.item-doc code { background: var(--bg); padding: 1px 6px; border-radius: 4px;
  font-family: Consolas, monospace; color: var(--cyan); }

.params table { border-collapse: collapse; width: 100%; margin-top: 10px; }
.params th { text-align: left; color: var(--fg2); font-size: .8rem; font-weight: 600;
  border-bottom: 1px solid var(--border); padding: 4px 8px; }
.params td { padding: 5px 8px; border-bottom: 1px solid var(--bg3);
  font-family: Consolas, monospace; font-size: .88rem; }

footer {
  text-align: center; color: var(--fg2); font-size: .8rem;
  padding: 24px; border-top: 1px solid var(--border); margin-top: 40px;
}
"""

def render_module_html(mod: ModuleDoc, all_modules: list[str]) -> str:
    def kind_badge(kind: str) -> str:
        color, _ = KIND_COLORS.get(kind, ("#8B949E", "•"))
        return f'<span class="item-kind" style="background:{color}22;color:{color}">{kind}</span>'

    def nav_links():
        out = '<nav>'
        out += '<div class="nav-section">Modules</div>'
        for m in sorted(all_modules):
            out += f'<a href="{m}.html">📦 {m}</a>'
        out += f'<div class="nav-section">Items</div>'
        for item in mod.items:
            _, icon = KIND_COLORS.get(item.kind, ("#8B949E", "•"))
            out += (f'<a href="#{item.name}">'
                    f'<span class="kind-icon">{icon}</span>{item.name}</a>')
        out += '</nav>'
        return out

    def render_item(item: DocItem) -> str:
        color, icon = KIND_COLORS.get(item.kind, ("#8B949E", "•"))
        params_html = ""
        if item.params:
            rows = "".join(f"<tr><td>{p}</td><td>any</td></tr>" for p in item.params)
            params_html = (f'<div class="params"><table>'
                           f'<tr><th>Parameter</th><th>Type</th></tr>{rows}</table></div>')
        doc_html = item.doc_comment.replace("\n", "<br>") if item.doc_comment else \
                   '<em style="color:var(--fg2)">No documentation.</em>'
        return (
            f'<div class="item" id="{item.name}">'
            f'  <div class="item-header">'
            f'    {kind_badge(item.kind)}'
            f'    <span class="item-name">{icon} {item.name}</span>'
            f'    <span class="item-line">line {item.line}</span>'
            f'  </div>'
            f'  <div class="item-body">'
            f'    <div class="item-sig">{item.signature}</div>'
            f'    <div class="item-doc">{doc_html}</div>'
            f'    {params_html}'
            f'  </div>'
            f'</div>'
        )

    items_html = "\n".join(render_item(i) for i in mod.items)
    module_doc_html = (mod.doc_comment.replace("\n", "<br>")
                       if mod.doc_comment else "")

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{mod.name} — Nux Docs</title>
  <style>{CSS}</style>
</head>
<body>
<header>
  <h1>📦 Nux Docs</h1>
  <span class="badge">v{__version__}</span>
  <span style="color:var(--fg2);margin-left:auto;font-size:.85rem">
    Generated {datetime.now().strftime("%Y-%m-%d %H:%M")}
  </span>
</header>
<div class="layout">
  {nav_links()}
  <main>
    <div class="module-title">{mod.name}</div>
    <div class="module-path">{mod.path}</div>
    {"<div class='module-doc'>" + module_doc_html + "</div>" if module_doc_html else ""}
    {items_html if mod.items else '<p style="color:var(--fg2)">No public items found.</p>'}
  </main>
</div>
<footer>Nux Documentation Generator v{__version__} · Built with NuxDoc</footer>
</body>
</html>"""


def render_index_html(modules: list[ModuleDoc]) -> str:
    rows = ""
    for mod in sorted(modules, key=lambda m: m.name):
        fns    = sum(1 for i in mod.items if i.kind == "fn")
        cls    = sum(1 for i in mod.items if i.kind == "class")
        others = len(mod.items) - fns - cls
        rows += (f'<tr><td><a href="{mod.name}.html">📦 {mod.name}</a></td>'
                 f'<td style="color:var(--fg2)">{mod.path}</td>'
                 f'<td>{fns} fn</td><td>{cls} class</td><td>{others}</td></tr>')

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Nux Documentation</title>
  <style>{CSS}
    table {{ border-collapse: collapse; width: 100%; }}
    th {{ text-align:left; color:var(--fg2); font-size:.8rem; border-bottom:1px solid var(--border);
          padding:8px 12px; text-transform:uppercase; letter-spacing:.08em; }}
    td {{ padding:10px 12px; border-bottom:1px solid var(--bg3); }}
    tr:hover td {{ background:var(--bg3); }}
  </style>
</head>
<body>
<header>
  <h1>📦 Nux Standard Library Docs</h1>
  <span class="badge">v{__version__}</span>
</header>
<main style="max-width:900px;margin:40px auto;padding:0 24px">
  <h2 style="margin-bottom:24px">All Modules ({len(modules)})</h2>
  <table>
    <tr><th>Module</th><th>File</th><th>Functions</th><th>Classes</th><th>Other</th></tr>
    {rows}
  </table>
</main>
<footer>Nux Documentation Generator v{__version__} · Built with NuxDoc</footer>
</body>
</html>"""


# ─── CLI ───────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="NuxDoc — Nux Language Documentation Generator")
    parser.add_argument("source", nargs="?", default="lib",
                        help="Source file or directory (default: lib/)")
    parser.add_argument("--out", "-o", default="docs",
                        help="Output directory (default: docs/)")
    parser.add_argument("--version", action="version", version=f"NuxDoc {__version__}")
    args = parser.parse_args()

    source = Path(args.source)
    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)

    nux_files: list[Path] = []
    if source.is_dir():
        nux_files = list(source.glob("**/*.nux"))
    elif source.is_file():
        nux_files = [source]
    else:
        print(f"ERROR: '{source}' not found.", file=sys.stderr)
        sys.exit(1)

    if not nux_files:
        print("No .nux files found.", file=sys.stderr)
        sys.exit(1)

    print(f"NuxDoc {__version__}  —  Generating docs for {len(nux_files)} file(s)…")

    modules: list[ModuleDoc] = []
    for f in sorted(nux_files):
        try:
            mod = parse_file(str(f))
            modules.append(mod)
            print(f"  OK  {f}  ({len(mod.items)} items)")
        except Exception as e:
            print(f"  ERR {f}  ERROR: {e}")

    all_names = [m.name for m in modules]

    # Write module pages
    for mod in modules:
        html = render_module_html(mod, all_names)
        out_file = out_dir / f"{mod.name}.html"
        out_file.write_text(html, encoding="utf-8")

    # Write index
    index_html = render_index_html(modules)
    (out_dir / "index.html").write_text(index_html, encoding="utf-8")

    print(f"\nDone! {len(modules)} pages written to '{out_dir}/'")
    print(f"Open: {out_dir / 'index.html'}")


if __name__ == "__main__":
    main()
