import { useEffect, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { BoincRpcStatus, BoincTask } from "./types/boinc";
import { Skeleton } from "./components/ui/skeleton";

function App() {
  const [activeTab, setActiveTab] = useState<"dashboard" | "debug">("dashboard");
  const [debugUnlocked, setDebugUnlocked] = useState(false);

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [boincTasks, setBoincTasks] = useState<BoincTask[] | null>(null);
  const [boincError, setBoincError] = useState<string | null>(null);
  const [boincLoading, setBoincLoading] = useState(false);
  const [isFocused, setIsFocused] = useState(() => window.document.hasFocus());

  const [rpcStatus, setRpcStatus] = useState<BoincRpcStatus | null>(null);
  const [rpcLoading, setRpcLoading] = useState(false);
  const [rpcCheckedAt, setRpcCheckedAt] = useState<number | null>(null);

  const [lastRefreshAt, setLastRefreshAt] = useState<number | null>(null);

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
      setLastRefreshAt(Date.now());
    } catch (e) {
      setBoincError(String(e));
    } finally {
      setBoincLoading(false);
      inFlightRef.current = false;
    }
  }

  async function loadRpcStatus() {
    setRpcLoading(true);
    try {
      const status = await invoke<BoincRpcStatus>("get_boinc_rpc_status");
      setRpcStatus(status);
      setRpcCheckedAt(Date.now());
    } catch (e) {
      setRpcStatus({ connection: "Connection Refused", error: String(e) });
      setRpcCheckedAt(Date.now());
    } finally {
      setRpcLoading(false);
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
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "d") {
        setDebugUnlocked((prev) => !prev);
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  useEffect(() => {
    if (!debugUnlocked && activeTab === "debug") {
      setActiveTab("dashboard");
    }
  }, [debugUnlocked, activeTab]);

  useEffect(() => {
    if (activeTab === "debug" && debugUnlocked) {
      void loadRpcStatus();
    }
  }, [activeTab, debugUnlocked]);

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

      <div className="row" style={{ marginTop: 12, gap: 8, alignItems: "center" }}>
        <button
          type="button"
          onClick={() => setActiveTab("dashboard")}
          disabled={activeTab === "dashboard"}
        >
          Dashboard
        </button>
        {debugUnlocked ? (
          <button type="button" onClick={() => setActiveTab("debug")} disabled={activeTab === "debug"}>
            Debug
          </button>
        ) : null}
        {debugUnlocked ? <span style={{ opacity: 0.7 }}>(Ctrl+Shift+D toggles)</span> : null}
      </div>

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

      {activeTab === "dashboard" ? (
        <>
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

        <div className="row" style={{ marginTop: 24, gap: 12, alignItems: "center" }}>
          <button type="button" onClick={loadBoincTasks} disabled={boincLoading}>
            {boincLoading ? "Refreshing..." : "Refresh now"}
          </button>
          <span style={{ opacity: 0.8 }}>
            Polling: {isFocused ? "on (2s)" : "off (window not focused)"}
          </span>
          <span style={{ opacity: 0.8 }}>
            Last refresh: {lastRefreshAt ? new Date(lastRefreshAt).toLocaleTimeString() : "—"}
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
        </>
      ) : (
        <div style={{ width: "100%", marginTop: 24, textAlign: "left", maxWidth: 720 }}>
          <h2 style={{ marginBottom: 8 }}>RPC Debug</h2>
          <div className="row" style={{ justifyContent: "flex-start", gap: 12, alignItems: "center" }}>
            <button type="button" onClick={loadRpcStatus} disabled={rpcLoading}>
              {rpcLoading ? "Checking..." : "Check connection"}
            </button>
            <button type="button" onClick={loadBoincTasks} disabled={boincLoading}>
              {boincLoading ? "Refreshing..." : "Refresh tasks"}
            </button>
            <span style={{ opacity: 0.8 }}>
              Last check: {rpcCheckedAt ? new Date(rpcCheckedAt).toLocaleTimeString() : "—"}
            </span>
          </div>

          <div style={{ marginTop: 12, display: "grid", gap: 8 }}>
            <div>
              <strong>Connected:</strong>{" "}
              {rpcStatus?.connection === "Connected" ? "Yes" : "No"}
            </div>
            <div>
              <strong>Authorized:</strong>{" "}
              {rpcStatus?.authorized === true
                ? "Yes"
                : rpcStatus?.authorized === false
                  ? "No"
                  : "—"}
            </div>
            <div>
              <strong>Status:</strong>{" "}
              {rpcStatus?.connection ?? "—"}
            </div>
            <div>
              <strong>Error:</strong>{" "}
              {rpcStatus?.error ?? "—"}
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
