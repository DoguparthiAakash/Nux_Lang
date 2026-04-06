import React, { useState } from "react";
import ReactDOM from "react-dom/client";

function App() {
    const [code, setCode] = useState(`# Nux Embedded Example\n\nimport "embedded/gpio";\n\nfunc main() {\n  # Blink\n}\n`);
    const [status, setStatus] = useState("Ready");

    const handleRun = async () => {
        setStatus("Compiling...");
        // Mock compilation delay
        setTimeout(() => {
            setStatus("Uploading to ESP32...");
            setTimeout(() => {
                setStatus("Running!");
            }, 1000);
        }, 1000);
    };

    return (
        <div style={{ padding: 20, fontFamily: "sans-serif" }}>
            <h1>Nux IDE</h1>
            <div style={{ display: "flex", gap: 10, marginBottom: 10 }}>
                <button onClick={handleRun} style={{ padding: "10px 20px", background: "#4CAF50", color: "white", border: "none", cursor: "pointer" }}>
                    Run on Device
                </button>
                <span>Status: {status}</span>
            </div>
            <textarea
                value={code}
                onChange={(e) => setCode(e.target.value)}
                style={{ width: "100%", height: "400px", fontFamily: "monospace", padding: 10 }}
            />
        </div>
    );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
