import { useEffect, useRef } from "react";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { type UnlistenFn } from "@tauri-apps/api/event";
import "./App.css";
import "xterm/css/xterm.css";

function App() {
  const termContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!termContainerRef.current) {
      return;
    }

    let isCleanup = false;
    let unlisten: UnlistenFn | undefined;

    async function setupTerminal() {
      // create and attach terminal
      const term = new Terminal({
        cursorBlink: true,
        fontSize: 14,
        fontFamily: "monospace",
        theme: { background: "#1e1e1e" },
        convertEol: true,
      });
      const fitAddon = new FitAddon();
      term.loadAddon(fitAddon);
      term.open(termContainerRef.current!);
      fitAddon.fit();

      // start wsl
      await invoke("start_wsl", { distro: "Arch" });

      if (isCleanup) return;

      // listener for output from wsl
      unlisten = await listen<string>("wsl-output", (event) => {
        term.write(event.payload);
      });

      // handler for input from the user
      const dataListener = term.onData((data) => {
        invoke("write_to_wsl", { data });
      });

      return () => {
        isCleanup = true;
        unlisten?.();
        dataListener.dispose();
        term.dispose();
      };
    }

    const cleanupPromise = setupTerminal();

    return () => {
      cleanupPromise.then((cleanup) => {
        cleanup?.();
      });
    };
  }, []);

  return (
    <div className="app">
      {/* Experimental Notice Banner */}
      <div
        style={{
          backgroundColor: "#ff6b35",
          color: "white",
          padding: "8px 16px",
          textAlign: "center",
          fontSize: "14px",
          fontWeight: "bold",
        }}
      >
        ⚠️ EXPERIMENTAL - This feature is under development and may be unstable
      </div>

      <div className="container">
        <div className="home">
          <h1>Subsys Launcher</h1>
          <p>Arch Linux Terminal</p>
          <div id="terminal" ref={termContainerRef}></div>
        </div>
        helo
      </div>
    </div>
  );
}

export default App;
