import React, { useEffect, useMemo, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

type Example = {
    id: string;
    name: string;
    description: string;
    code: string;
    output: string;
    accent: string;
};

const examples: Example[] = [
    {
        id: "hello",
        name: "Hello World",
        description: "Your first Nux program",
        accent: "orange",
        code: `func main() {
  print("Hello, Nux!")
  print("Welcome to the playground.")
}`,
        output: `Hello, Nux!
Welcome to the playground.`,
    },
    {
        id: "control-flow",
        name: "Control Flow",
        description: "Loops and decisions",
        accent: "blue",
        code: `func main() {
  let scores = [12, 7, 18, 4]

  for score in scores {
    if score > 10 {
      print("pass: " + score)
    }
  }
}`,
        output: `pass: 12
pass: 18`,
    },
    {
        id: "pointers",
        name: "Memory Pointers",
        description: "A peek under the hood",
        accent: "green",
        code: `func main() {
  let value = 42
  let pointer = &value

  print("value: " + value)
  print("address: " + pointer)
}`,
        output: `value: 42
address: 0x00007FF8`,
    },
];

const modes = ["VM", "Native", "LLVM"] as const;
type Mode = (typeof modes)[number];

function App() {
    const [selectedId, setSelectedId] = useState("hello");
    const [code, setCode] = useState(examples[0].code);
    const [mode, setMode] = useState<Mode>("VM");
    const [status, setStatus] = useState("Ready to run");
    const [isRunning, setIsRunning] = useState(false);
    const [output, setOutput] = useState("Run your code to see the browser preview here.");
    const [hasRun, setHasRun] = useState(false);
    const [duration, setDuration] = useState("--");
    const runTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

    const selectedExample = useMemo(
        () => examples.find((example) => example.id === selectedId) ?? examples[0],
        [selectedId],
    );

    const reset = () => {
        setCode(selectedExample.code);
        setStatus("Ready to run");
        setOutput("Run your code to see the browser preview here.");
        setHasRun(false);
        setDuration("--");
    };

    const handleExampleChange = (id: string) => {
        const nextExample = examples.find((example) => example.id === id) ?? examples[0];
        setSelectedId(nextExample.id);
        setCode(nextExample.code);
        setStatus("Ready to run");
        setOutput("Run your code to see the browser preview here.");
        setHasRun(false);
        setDuration("--");
    };

    const handleRun = () => {
        if (isRunning) return;
        setIsRunning(true);
        setStatus("Running browser preview...");
        setOutput("Preparing preview...");
        setDuration("--");
        runTimer.current = setTimeout(() => {
            setOutput(selectedExample.output);
            setStatus("Run complete");
            setDuration(`${(0.18 + code.length / 5000).toFixed(2)}s`);
            setIsRunning(false);
            setHasRun(true);
        }, 550);
    };

    useEffect(() => () => {
        if (runTimer.current) clearTimeout(runTimer.current);
    }, []);

    useEffect(() => {
        const onKeyDown = (event: KeyboardEvent) => {
            if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
                event.preventDefault();
                handleRun();
            }
        };
        window.addEventListener("keydown", onKeyDown);
        return () => window.removeEventListener("keydown", onKeyDown);
    });

    const lineNumbers = code.split("\n").map((_, index) => index + 1);

    return (
        <div className="app-shell">
            <header className="topbar">
                <div className="brand-lockup"><span className="brand-mark">N</span><span>Nux</span><span className="brand-divider" /><span className="product-label">Playground</span></div>
                <div className="topbar-actions"><a href="https://github.com" target="_blank" rel="noreferrer">Docs <span aria-hidden="true">↗</span></a><span className="status-dot" /> <span className="topbar-status">Browser preview</span><button className="icon-button" aria-label="Toggle theme">◐</button></div>
            </header>

            <div className="app-grid">
                <aside className="sidebar">
                    <div className="side-heading"><span>Examples</span><span className="count">03</span></div>
                    <nav className="example-list" aria-label="Nux examples">
                        {examples.map((example) => <button key={example.id} className={`example-item ${selectedId === example.id ? "active" : ""}`} onClick={() => handleExampleChange(example.id)}><span className={`example-icon ${example.accent}`}>{example.id === "hello" ? "01" : example.id === "control-flow" ? "02" : "03"}</span><span><strong>{example.name}</strong><small>{example.description}</small></span><span className="chevron">›</span></button>)}
                    </nav>
                    <div className="sidebar-footer"><div className="runtime-label"><span className="mini-pulse" /> Nux runtime</div><strong>v1.0.0</strong><small>WASM-compatible preview</small></div>
                </aside>

                <main className="workspace">
                    <div className="workspace-head"><div><div className="breadcrumb"><span>Workspace</span><span>/</span><strong>{selectedExample.name}</strong></div><h1>Nux Playground <span className="beta-tag">BETA</span></h1><p>Write, run, and learn Nux in your browser.</p></div><div className="shortcut"><kbd>⌘</kbd><kbd>↵</kbd><span>to run</span></div></div>

                    <section className="playground-card">
                        <div className="editor-toolbar"><div className="toolbar-group"><label htmlFor="example-select">Example</label><select id="example-select" value={selectedId} onChange={(event) => handleExampleChange(event.target.value)}>{examples.map((example) => <option value={example.id} key={example.id}>{example.name}</option>)}</select></div><div className="toolbar-group mode-group"><span className="toolbar-label">Execution mode</span><div className="segmented" role="group" aria-label="Execution mode">{modes.map((item) => <button key={item} className={mode === item ? "selected" : ""} onClick={() => setMode(item)}>{item}</button>)}</div></div><div className="toolbar-actions"><button className="reset-button" onClick={reset}>↺ <span>Reset</span></button><button className="run-button" onClick={handleRun} disabled={isRunning}><span aria-hidden="true">▶</span> {isRunning ? "Running" : "Run"}</button></div></div>
                        <div className="panes">
                            <section className="code-pane" aria-label="Nux code editor"><div className="pane-header"><span className="file-tab"><span className="file-dot" /> main.nux</span><span className="language-note">Nux</span></div><div className="editor-body"><div className="line-numbers" aria-hidden="true">{lineNumbers.map((line) => <span key={line}>{line}</span>)}</div><textarea aria-label="Nux source code" spellCheck={false} value={code} onChange={(event) => setCode(event.target.value)} /></div><div className="pane-footer"><span>UTF-8</span><span>Spaces: 2</span><span className="cursor-state">Ln 1, Col 1</span></div></section>
                            <section className="output-pane" aria-label="Browser preview output"><div className="pane-header output-header"><span><span className={`output-status ${hasRun ? "success" : ""}`} /> Output</span><span className="preview-label">Browser preview</span></div><div className={`output-body ${hasRun ? "has-output" : ""}`}><div className="output-meta"><span className="status-chip"><span className="chip-dot" /> {status}</span>{hasRun && <span>{mode} · {duration}</span>}</div><pre>{output}</pre>{!hasRun && <div className="output-placeholder"><span>⌁</span><p>Your program's output<br />will appear here.</p></div>}</div><div className="pane-footer output-footer"><span className="output-note">Preview only · no device connected</span><button onClick={() => { setOutput("Output cleared."); setHasRun(false); setStatus("Ready to run"); setDuration("--"); }}>Clear output</button></div></section>
                        </div>
                    </section>
                    <div className="info-strip"><div><span className="info-icon">i</span><span><strong>Nux 1.0</strong> language preview</span></div><div className="toolchain"><span className="toolchain-check">✓</span> {mode} mode active <span className="strip-divider" /> <span>Backend integration coming soon</span></div></div>
                    <footer className="workspace-footer"><span>Made for curious minds.</span><span>© 2024 Nux Language</span></footer>
                </main>
            </div>
        </div>
    );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
