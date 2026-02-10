## 1.Architecture design
```mermaid
graph TD
  A["User"] --> B["Tauri 2.0 Application Shell"]
  B --> C["WebView (Desktop/Mobile)"]
  C --> D["React UI (TypeScript)"]

  subgraph "Host Layer"
    B
  end

  subgraph "Frontend Layer"
    C
    D
  end
```

## 2.Technology Description
- Frontend: React@18 + TypeScript + vite + tailwindcss + shadcn/ui
- Backend: Tauri@2 (Rust host; no separate server)

## 3.Route definitions
| Route | Purpose |
|-------|---------|
| / | Single-screen UI that renders the Hello World view in Zinc dark theme |

## 6.Data model(if applicable)
Not applicable (no persisted data required for a Hello World screen).
