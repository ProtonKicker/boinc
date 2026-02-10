## 1. Product Overview
A cross-platform Tauri 2.0 desktop/mobile app that renders a clean “Hello World” screen.
Built with React (TypeScript) + Vite and styled with Tailwind + shadcn/ui using a Zinc dark theme.

## 2. Core Features

### 2.1 Feature Module
Our app requirements consist of the following main pages:
1. **Hello World**: app shell, centered greeting content, Zinc dark theme styling.

### 2.2 Page Details
| Page Name | Module Name | Feature description |
|-----------|-------------|---------------------|
| Hello World | App Shell | Render a single-window app surface that hosts the React UI inside Tauri for Windows/macOS/Android. |
| Hello World | Hello World Content | Display a clear primary headline “Hello World” with a short supporting line (subtitle) in a centered layout. |
| Hello World | Zinc Dark Theme | Apply Zinc dark theme styling consistently across background, text, and card elements using Tailwind + shadcn/ui tokens. |

## 3. Core Process
User Flow:
1. You launch the app.
2. You see the Hello World screen in Zinc dark theme.

```mermaid
graph TD
  A["App Launch"] --> B["Hello