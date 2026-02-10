import { useEffect, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { BoincTask } from "./types/boinc";
import { Skeleton } from "./components/ui/skeleton";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [boincTasks, setBoincTasks] = useState<BoincTask[] | null>(null);
  const [boincError, setBoincError] = useState<string | null>(null);
  const [boincLoading, setBoincLoading] = useState(false);
  const [isFocused, setIsFocused] = useState(() => window.document.hasFocus());

  const pollTimerRef = useRef<number | null>(null);
  const inFlightRef = useRef(false);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function loadBoincTasks() {
    if (inFlightRef.current) return;
    inFlightRef.current = true;
    setBoincError(null);
    setBoincLoading(true);
    try {
      const tasks = await invoke<BoincTask[]>("get_boinc_tasks", {
        active_only: false,
      });
      setBoincTasks(tasks);
    } catch (e) {
      setBoincError(String(e));
    } finally {
      setBoincLoading(false);
      inFlightRef.current = false;
    }
  }

  useEffect(() => {
    const onFocus = () => setIsFocused(true);
    const onBlur = () => setIsFocused(false);
    const onVisibilityChange = () => {
      setIsFocused(!window.document.hidden && window.document.hasFocus());
    };

    window.addEventListener("focus", onFocus);
    window.addEventListener("blur", onBlur);
    window.document.addEventListener("visibilitychange", onVisibilityChange);
    return () => {
      window.removeEventListener("focus", onFocus);
      window.removeEventListener("blur", onBlur);
      window.document.removeEventListener("visibilitychange", onVisibilityChange);
    };
  }, []);

  useEffect(() => {
    if (!isFocused) {
      if (pollTimerRef.current != null) {
        window.clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }
      return;
    }

    void loadBoincTasks();
    pollTimerRef.current = window.setInterval(() => {
      void loadBoincTasks();
    }, 2000);

    return () => {
      if (pollTimerRef.current != null) {
        window.clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }
    };
  }, [isFocused]);

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <div className="row" style={{ marginTop: 24 }}>
        <button type="button" onClick={loadBoincTasks} disabled={boincLoading}>
          {boincLoading ? "Refreshing..." : "Refresh now"}
        </button>
        <span style={{ opacity: 0.8 }}>
          Polling: {isFocused ? "on (2s)" : "off (window not focused)"}
        </span>
      </div>
      {boincError ? <p style={{ color: "#ff4d4f" }}>{boincError}</p> : null}
      {boincTasks === null ? (
        <div style={{ width: "100%", marginTop: 12 }}>
          <div style={{ maxWidth: 720, margin: "0 auto" }}>
            <Skeleton className="h-4 w-2/3" />
            <div className="h-3" />
            <Skeleton className="h-4 w-full" />
            <div className="h-3" />
            <Skeleton className="h-4 w-5/6" />
          </div>
        </div>
      ) : (
        <div style={{ width: "100%", marginTop: 12, textAlign: "left" }}>
          {boincLoading ? (
            <div style={{ marginBottom: 12 }}>
              <Skeleton className="h-3 w-24" />
            </div>
          ) : null}
          <pre style={{ whiteSpace: "pre-wrap" }}>{JSON.stringify(boincTasks, null, 2)}</pre>
        </div>
      )}
    </main>
  );
}

export default App;
