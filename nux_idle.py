#!/usr/bin/env python3
"""
Nux IDLE - Interactive Development & Learning Environment
A rich GUI editor and REPL for the Nux programming language.
Inspired by Python IDLE, VSCode, and Atom.
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox, font
import subprocess
import threading
import os
import sys
import tempfile
import re
import json
from pathlib import Path

# ─── Color Palette ────────────────────────────────────────────────────────────
PALETTE = {
    "bg":        "#0E1117",
    "bg2":       "#161B22",
    "bg3":       "#21262D",
    "panel":     "#1C2128",
    "border":    "#30363D",
    "accent":    "#7E5FFF",
    "accent2":   "#A78BFA",
    "green":     "#3FB950",
    "red":       "#F85149",
    "yellow":    "#D29922",
    "blue":      "#58A6FF",
    "cyan":      "#39D0D8",
    "orange":    "#FFA657",
    "fg":        "#E6EDF3",
    "fg2":       "#8B949E",
    "fg3":       "#484F58",
    "selection": "#264F78",
    "highlight": "#1F2937",
}

# Nux syntax keywords
NUX_KEYWORDS  = ["fn", "let", "const", "if", "else", "while", "for", "return",
                  "import", "from", "use", "class", "trait", "impl", "match",
                  "break", "continue", "true", "false", "null", "in", "and",
                  "or", "not", "new", "spawn", "join", "try", "catch", "throw",
                  "unsafe", "extern", "enum", "type", "pub", "async", "await",
                  "print", "println", "input", "python", "c"]
NUX_TYPES     = ["int", "float", "str", "bool", "void", "any", "arr", "map"]
NUX_BUILTINS  = ["print", "println", "input", "len", "range", "type", "str", "int", "float"]


# ─── Syntax Highlighting Engine ────────────────────────────────────────────────
class NuxHighlighter:
    def __init__(self, text_widget):
        self.text = text_widget
        self._configure_tags()

    def _configure_tags(self):
        t = self.text
        t.tag_configure("keyword",  foreground=PALETTE["accent2"], font=("Consolas", 11, "bold"))
        t.tag_configure("type",     foreground=PALETTE["cyan"])
        t.tag_configure("string",   foreground=PALETTE["green"])
        t.tag_configure("comment",  foreground=PALETTE["fg3"], font=("Consolas", 11, "italic"))
        t.tag_configure("number",   foreground=PALETTE["orange"])
        t.tag_configure("builtin",  foreground=PALETTE["blue"])
        t.tag_configure("operator", foreground=PALETTE["red"])
        t.tag_configure("fn_name",  foreground=PALETTE["yellow"])
        t.tag_configure("bracket",  foreground=PALETTE["accent"])
        t.tag_configure("import_path", foreground=PALETTE["green"])

    def highlight(self, event=None):
        t = self.text
        for tag in ("keyword","type","string","comment","number","builtin",
                    "operator","fn_name","bracket","import_path"):
            t.tag_remove(tag, "1.0", "end")

        content = t.get("1.0", "end-1c")
        self._apply(content)

    def _apply(self, content):
        t = self.text

        patterns = [
            ("comment",    r"//[^\n]*"),
            ("string",     r'"(?:[^"\\]|\\.)*"'),
            ("number",     r"\b\d+\.?\d*\b"),
            ("keyword",    r"\b(" + "|".join(NUX_KEYWORDS) + r")\b"),
            ("type",       r"\b(" + "|".join(NUX_TYPES) + r")\b"),
            ("fn_name",    r"\bfn\s+(\w+)"),
            ("operator",   r"[+\-*/%=<>!&|^~]+"),
            ("bracket",    r"[(){}\[\]]"),
        ]

        for tag, pattern in patterns:
            for m in re.finditer(pattern, content, re.MULTILINE):
                # For fn_name, highlight only the captured group
                if tag == "fn_name" and m.lastindex:
                    start = m.start(1)
                    end   = m.end(1)
                else:
                    start = m.start()
                    end   = m.end()

                line_s = content[:start].count("\n") + 1
                col_s  = start - content[:start].rfind("\n") - 1
                line_e = content[:end].count("\n") + 1
                col_e  = end - content[:end].rfind("\n") - 1

                t.tag_add(tag, f"{line_s}.{col_s}", f"{line_e}.{col_e}")


# ─── Main Application ──────────────────────────────────────────────────────────
class NuxIDLE:
    def __init__(self, root: tk.Tk):
        self.root = root
        self.root.title("Nux IDLE  —  Interactive Development & Learning Environment")
        self.root.geometry("1300x820")
        self.root.configure(bg=PALETTE["bg"])

        self.current_file: str | None = None
        self.nux_exe = self._find_nux()
        self.history: list[str] = []
        self.history_idx = 0

        self._build_menubar()
        self._build_toolbar()
        self._build_body()
        self._build_statusbar()

        self.root.bind("<Control-r>", self.run_code)
        self.root.bind("<Control-s>", self.save_file)
        self.root.bind("<Control-o>", self.open_file)
        self.root.bind("<Control-n>", self.new_file)
        self.root.bind("<F5>",        self.run_code)

        # Show welcome
        self._print_output(WELCOME, tag="info")
        self._update_status()

    # ── Discovery ──────────────────────────────────────────────────────────────
    def _find_nux(self) -> str:
        candidates = [
            str(Path(__file__).parent / "nux/nux_oleg/nux_dist/target/debug/nux.exe"),
            str(Path(__file__).parent / "nux/nux_oleg/nux_dist/target/release/nux.exe"),
            "nux",
        ]
        for c in candidates:
            if Path(c).exists() or c == "nux":
                return c
        return "nux"

    # ── Menu ───────────────────────────────────────────────────────────────────
    def _build_menubar(self):
        mb = tk.Menu(self.root, bg=PALETTE["bg2"], fg=PALETTE["fg"],
                     activebackground=PALETTE["accent"], activeforeground=PALETTE["fg"],
                     relief="flat", bd=0)
        self.root.config(menu=mb)

        def add_menu(label, items):
            m = tk.Menu(mb, tearoff=0, bg=PALETTE["bg3"], fg=PALETTE["fg"],
                        activebackground=PALETTE["accent"], activeforeground=PALETTE["fg"])
            mb.add_cascade(label=label, menu=m)
            for item in items:
                if item == "---":
                    m.add_separator()
                else:
                    name, cmd, acc = item
                    m.add_command(label=name, command=cmd, accelerator=acc)

        add_menu("File", [
            ("New",         self.new_file,   "Ctrl+N"),
            ("Open...",     self.open_file,  "Ctrl+O"),
            ("Save",        self.save_file,  "Ctrl+S"),
            ("Save As...",  self.save_as,    ""),
            "---",
            ("Exit",        self.root.quit,  ""),
        ])
        add_menu("Run", [
            ("Run File",    self.run_code,   "F5 / Ctrl+R"),
            ("Clear Output",self.clear_output,""),
        ])
        add_menu("Help", [
            ("About",       self.show_about, ""),
        ])

    # ── Toolbar ────────────────────────────────────────────────────────────────
    def _build_toolbar(self):
        bar = tk.Frame(self.root, bg=PALETTE["bg2"], height=44, bd=0)
        bar.pack(fill="x", side="top")

        def btn(text, cmd, color=PALETTE["accent"]):
            b = tk.Button(bar, text=text, command=cmd, bg=color, fg=PALETTE["fg"],
                          relief="flat", bd=0, padx=14, pady=6,
                          font=("Segoe UI", 9, "bold"),
                          activebackground=PALETTE["accent2"],
                          activeforeground=PALETTE["fg"],
                          cursor="hand2")
            b.pack(side="left", padx=4, pady=6)
            return b

        btn("⏵  Run (F5)", self.run_code, PALETTE["green"])
        btn("⬛ Stop",      self.stop_run,  PALETTE["red"])
        btn("📁 Open",      self.open_file, PALETTE["bg3"])
        btn("💾 Save",      self.save_file, PALETTE["bg3"])
        btn("📄 New",       self.new_file,  PALETTE["bg3"])
        btn("🗑 Clear",     self.clear_output, PALETTE["bg3"])

        # NUX path label
        self._nux_label_var = tk.StringVar(value=f"  nux: {self.nux_exe}")
        lbl = tk.Label(bar, textvariable=self._nux_label_var,
                       bg=PALETTE["bg2"], fg=PALETTE["fg2"],
                       font=("Consolas", 8))
        lbl.pack(side="right", padx=12)

    # ── Body ───────────────────────────────────────────────────────────────────
    def _build_body(self):
        paned = tk.PanedWindow(self.root, orient=tk.HORIZONTAL,
                               bg=PALETTE["border"], sashwidth=4, sashrelief="flat")
        paned.pack(fill="both", expand=True)

        # Left: editor
        editor_frame = tk.Frame(paned, bg=PALETTE["bg"])
        paned.add(editor_frame, minsize=400)
        self._build_editor(editor_frame)

        # Right: REPL + output
        right_frame = tk.Frame(paned, bg=PALETTE["bg"])
        paned.add(right_frame, minsize=320)
        self._build_right(right_frame)

    def _build_editor(self, parent):
        header = tk.Frame(parent, bg=PALETTE["bg2"], height=28)
        header.pack(fill="x")
        tk.Label(header, text="  ✏  Editor", bg=PALETTE["bg2"], fg=PALETTE["fg2"],
                 font=("Segoe UI", 9)).pack(side="left", pady=4)

        self._file_lbl = tk.Label(header, text="  untitled.nux",
                                  bg=PALETTE["bg2"], fg=PALETTE["accent2"],
                                  font=("Segoe UI", 9, "bold"))
        self._file_lbl.pack(side="left")

        # Line numbers + editor
        edit_area = tk.Frame(parent, bg=PALETTE["bg"])
        edit_area.pack(fill="both", expand=True)

        self.line_numbers = tk.Text(edit_area, width=4, bg=PALETTE["bg2"],
                                    fg=PALETTE["fg3"], font=("Consolas", 11),
                                    state="disabled", relief="flat", bd=0,
                                    padx=4, selectbackground=PALETTE["bg2"])
        self.line_numbers.pack(side="left", fill="y")

        self.editor = tk.Text(edit_area, bg=PALETTE["bg"], fg=PALETTE["fg"],
                               insertbackground=PALETTE["accent"],
                               font=("Consolas", 11), relief="flat", bd=0,
                               padx=12, pady=8,
                               selectbackground=PALETTE["selection"],
                               undo=True, autoseparators=True, maxundo=-1,
                               tabs=("4c",))
        self.editor.pack(side="left", fill="both", expand=True)

        self.highlighter = NuxHighlighter(self.editor)
        self.editor.bind("<KeyRelease>", self._on_editor_change)
        self.editor.bind("<Tab>", self._handle_tab)

        # Starter template
        self.editor.insert("1.0", STARTER_CODE)
        self.highlighter.highlight()

    def _build_right(self, parent):
        # Output panel
        out_header = tk.Frame(parent, bg=PALETTE["bg2"], height=28)
        out_header.pack(fill="x")
        tk.Label(out_header, text="  ▶  Output", bg=PALETTE["bg2"], fg=PALETTE["fg2"],
                 font=("Segoe UI", 9)).pack(side="left", pady=4)

        self.output = tk.Text(parent, bg=PALETTE["bg"], fg=PALETTE["fg"],
                               font=("Consolas", 11), relief="flat", bd=0,
                               padx=12, pady=8, state="disabled",
                               selectbackground=PALETTE["selection"],
                               height=20)
        self.output.pack(fill="both", expand=True)

        self.output.tag_configure("info",    foreground=PALETTE["cyan"])
        self.output.tag_configure("error",   foreground=PALETTE["red"])
        self.output.tag_configure("success", foreground=PALETTE["green"])
        self.output.tag_configure("warn",    foreground=PALETTE["yellow"])
        self.output.tag_configure("plain",   foreground=PALETTE["fg"])

        # REPL panel
        repl_header = tk.Frame(parent, bg=PALETTE["bg2"], height=28)
        repl_header.pack(fill="x")
        tk.Label(repl_header, text="  ≫  REPL  (press Enter to run)", bg=PALETTE["bg2"],
                 fg=PALETTE["fg2"], font=("Segoe UI", 9)).pack(side="left", pady=4)

        repl_area = tk.Frame(parent, bg=PALETTE["bg3"])
        repl_area.pack(fill="x", side="bottom")

        tk.Label(repl_area, text="nux> ", bg=PALETTE["bg3"],
                 fg=PALETTE["accent2"], font=("Consolas", 11, "bold")).pack(side="left", padx=4)

        self.repl_entry = tk.Entry(repl_area, bg=PALETTE["bg3"], fg=PALETTE["fg"],
                                    insertbackground=PALETTE["accent"],
                                    font=("Consolas", 11), relief="flat", bd=0)
        self.repl_entry.pack(side="left", fill="x", expand=True, ipady=8)
        self.repl_entry.bind("<Return>", self.run_repl)
        self.repl_entry.bind("<Up>",     self._history_up)
        self.repl_entry.bind("<Down>",   self._history_down)

    def _build_statusbar(self):
        bar = tk.Frame(self.root, bg=PALETTE["bg2"], height=22)
        bar.pack(fill="x", side="bottom")

        self._status_var = tk.StringVar(value="Ready")
        tk.Label(bar, textvariable=self._status_var, bg=PALETTE["bg2"],
                 fg=PALETTE["fg2"], font=("Segoe UI", 8)).pack(side="left", padx=8)

        self._line_col_var = tk.StringVar(value="Ln 1, Col 1")
        tk.Label(bar, textvariable=self._line_col_var, bg=PALETTE["bg2"],
                 fg=PALETTE["fg2"], font=("Segoe UI", 8)).pack(side="right", padx=8)

        self.editor.bind("<KeyRelease>", self._update_status)
        self.editor.bind("<ButtonRelease>", self._update_status)

    # ── Helpers ────────────────────────────────────────────────────────────────
    def _on_editor_change(self, event=None):
        self.highlighter.highlight()
        self._update_line_numbers()
        self._update_status()

    def _update_line_numbers(self):
        self.line_numbers.config(state="normal")
        self.line_numbers.delete("1.0", "end")
        lines = self.editor.get("1.0", "end-1c").count("\n") + 1
        self.line_numbers.insert("1.0", "\n".join(str(i) for i in range(1, lines + 1)))
        self.line_numbers.config(state="disabled")

    def _update_status(self, event=None):
        try:
            pos = self.editor.index("insert")
            ln, col = pos.split(".")
            self._line_col_var.set(f"Ln {ln}, Col {int(col)+1}")
        except Exception:
            pass

    def _handle_tab(self, event):
        self.editor.insert("insert", "    ")
        return "break"

    def _print_output(self, text: str, tag: str = "plain"):
        self.output.config(state="normal")
        self.output.insert("end", text + "\n", tag)
        self.output.see("end")
        self.output.config(state="disabled")

    # ── File Operations ────────────────────────────────────────────────────────
    def new_file(self, event=None):
        self.editor.delete("1.0", "end")
        self.editor.insert("1.0", STARTER_CODE)
        self.current_file = None
        self._file_lbl.config(text="  untitled.nux")
        self.highlighter.highlight()

    def open_file(self, event=None):
        path = filedialog.askopenfilename(
            filetypes=[("Nux Files", "*.nux"), ("All Files", "*.*")])
        if path:
            with open(path, "r", encoding="utf-8") as f:
                self.editor.delete("1.0", "end")
                self.editor.insert("1.0", f.read())
            self.current_file = path
            self._file_lbl.config(text=f"  {Path(path).name}")
            self.highlighter.highlight()

    def save_file(self, event=None):
        if self.current_file:
            with open(self.current_file, "w", encoding="utf-8") as f:
                f.write(self.editor.get("1.0", "end-1c"))
            self._status_var.set(f"Saved: {self.current_file}")
        else:
            self.save_as()

    def save_as(self, event=None):
        path = filedialog.asksaveasfilename(
            defaultextension=".nux",
            filetypes=[("Nux Files", "*.nux"), ("All Files", "*.*")])
        if path:
            self.current_file = path
            self._file_lbl.config(text=f"  {Path(path).name}")
            self.save_file()

    # ── Execution ──────────────────────────────────────────────────────────────
    def run_code(self, event=None):
        code = self.editor.get("1.0", "end-1c")
        self._print_output("─" * 60, "info")
        self._print_output("▶ Running…", "info")
        self._status_var.set("Running…")
        threading.Thread(target=self._exec, args=(code,), daemon=True).start()

    def _exec(self, code: str):
        tmp = tempfile.NamedTemporaryFile(suffix=".nux", delete=False,
                                          mode="w", encoding="utf-8")
        tmp.write(code)
        tmp.close()
        try:
            result = subprocess.run(
                [self.nux_exe, tmp.name],
                capture_output=True, text=True, timeout=30)
            stdout = result.stdout.strip()
            stderr = result.stderr.strip()
            if stdout:
                self.root.after(0, lambda: self._print_output(stdout, "plain"))
            if stderr:
                self.root.after(0, lambda: self._print_output(stderr, "error"))
            rc = result.returncode
            tag = "success" if rc == 0 else "error"
            self.root.after(0, lambda: self._print_output(f"─ Exit code {rc} ─", tag))
            self.root.after(0, lambda: self._status_var.set(f"Finished (exit {rc})"))
        except subprocess.TimeoutExpired:
            self.root.after(0, lambda: self._print_output("Timed out after 30s", "error"))
        except FileNotFoundError:
            self.root.after(0, lambda: self._print_output(
                f"ERROR: nux executable not found at '{self.nux_exe}'.\n"
                "Build the project: cargo build --bin nux", "error"))
        finally:
            os.unlink(tmp.name)

    def stop_run(self):
        self._print_output("(Manual stop not yet implemented; wait for timeout)", "warn")

    # ── REPL ───────────────────────────────────────────────────────────────────
    def run_repl(self, event=None):
        code = self.repl_entry.get().strip()
        if not code:
            return
        self.history.append(code)
        self.history_idx = len(self.history)
        self._print_output(f"nux> {code}", "info")
        self.repl_entry.delete(0, "end")

        # Wrap in a fn main so simple expressions work
        wrapped = f"fn main() {{\n    {code}\n}}\nmain();\n"
        threading.Thread(target=self._exec, args=(wrapped,), daemon=True).start()

    def _history_up(self, event):
        if self.history_idx > 0:
            self.history_idx -= 1
            self.repl_entry.delete(0, "end")
            self.repl_entry.insert(0, self.history[self.history_idx])

    def _history_down(self, event):
        if self.history_idx < len(self.history) - 1:
            self.history_idx += 1
            self.repl_entry.delete(0, "end")
            self.repl_entry.insert(0, self.history[self.history_idx])
        else:
            self.history_idx = len(self.history)
            self.repl_entry.delete(0, "end")

    def clear_output(self, event=None):
        self.output.config(state="normal")
        self.output.delete("1.0", "end")
        self.output.config(state="disabled")

    def show_about(self):
        messagebox.showinfo("About Nux IDLE",
            "Nux IDLE\n"
            "Interactive Development & Learning Environment\n\n"
            "Version: 0.1.0\n"
            "The Nux Programming Language\n\n"
            "Shortcuts:\n"
            "  F5 / Ctrl+R — Run file\n"
            "  Ctrl+S      — Save\n"
            "  Ctrl+O      — Open\n"
            "  Ctrl+N      — New file\n"
            "  nux>  REPL  — Enter expression + Enter")


WELCOME = """\
╔══════════════════════════════════════════════════════════╗
║          Nux IDLE  v0.1.0  —  Welcome!                   ║
║  Write Nux code in the editor, press F5 to run.          ║
║  Use the REPL below for quick one-liners.                ║
╚══════════════════════════════════════════════════════════╝
"""

STARTER_CODE = """\
// Welcome to Nux!
// F5 or Ctrl+R to run this file.

import "math", "os";

fn greet(name) {
    print("Hello, ");
    print(name);
    println("!");
}

fn main() {
    greet("World");
    let x = 42;
    let y = x * 2;
    println(y);
}

main();
"""


if __name__ == "__main__":
    root = tk.Tk()
    app = NuxIDLE(root)
    root.mainloop()
