# Page Design — BOINC Manager Bento Dashboard (Desktop-first)

## Global Design System
### Layout
- Desktop-first shell: **two-column layout** with a fixed glassmorphism sidebar (left) and a scrollable main content area (right).
- Main content uses **CSS Grid** for bento layout (12-column grid; cards span 3–12 columns depending on importance).
- Spacing scale: 4/8/12/16/24/32px; card gap 16px; section padding 24px.
- Responsive behavior (down to tablet): sidebar collapses to icon-only rail; bento grid collapses to 6 columns; drawers become full-width sheets.

### Meta Information (applies to all pages)
- Title template: `BOINC Manager — {Page}`
- Description: "Monitor and control your BOINC client with a modern dashboard."
- Open Graph: title, description, app icon image.

### Global Styles (Tailwind + shadcn/ui)
- Theme: dark-first, with light mode support.
- Background: `bg-gradient-to-br` (near-black → deep slate) with subtle noise overlay.
- Glass surfaces: `bg-white/10 dark:bg-white/5`, `backdrop-blur-xl`, `border border-white/10`.
- Typography: Inter (or system), base 14–16px; headings 20/24/32.
- Accent color: cyan/teal for active states; destructive: red.
- Buttons (shadcn):
  - Primary: solid accent, hover increases brightness.
  - Secondary: glass outline.
  - Destructive: red, confirm dialogs.
- Icons: Lucide (16–20px), consistent stroke width.
- Motion: Framer Motion for tab/page transitions (fade + slight y-translate), respects “reduce motion”.

### Shared Components
- **AppShell**: Sidebar + top bar + main outlet.
- **GlassSidebar**:
  - Brand header (app name + connection chip).
  - Nav items with Lucide icons (Dashboard, Tasks & Projects, Settings).
  - Footer: connection state indicator + refresh button.
- **TopBar**:
  - Current page title.
  - Global actions: refresh, run/suspend toggle (when connected).
- **BentoCard** (shadcn Card): title, value, subtext, optional sparkline/mini-meter.
- **List + Toolbar**: search, filter chips, sort menu.
- **Details Drawer** (shadcn Sheet): shows task/project details and actions.
- **Toasts**: success/error feedback for RPC actions.

---

## Page 1 — Dashboard (/)
### Layout
- Main area uses **CSS Grid bento**: 12 columns, 3–4 rows depending on viewport height.
- Cards animate in on mount; switching routes uses Framer Motion `AnimatePresence`.

### Meta
- Title: `BOINC Manager — Dashboard`
- OG: Dashboard preview (optional)

### Page Structure
1. **TopBar**
2. **Connection Banner (conditional)**
3. **Bento Grid**
4. **Activity Feed**

### Sections & Components
- Connection Banner (only when disconnected/error):
  - Text: status + last error.
  - CTA button: “Open Settings”.
- Bento Grid Cards (examples of card types; all read from overview/state):
  - Overall State Card: Running/Suspended + toggle button.
  - Active Tasks Card: count + link to Tasks.
  - CPU Usage Card (if available): progress bar / meter.
  - GPU Usage Card (if available): progress bar / meter.
  - Credit Card (if available): total credit + RAC.
  - Network Card: on/off + toggle.
- Activity Feed:
  - Vertical list with severity dot + timestamp.
  - “View more” links to Tasks & Projects page section.

### Interaction Notes
- Quick actions require a connected state; otherwise disabled with tooltip.
- Route transition: 180–240ms easeOut (fade + y: 6px).

---

## Page 2 — Tasks & Projects (/tasks)
### Layout
- Two stacked panels (desktop): Tasks panel first, Projects panel second.
- Each panel is a shadcn Card with its own toolbar; lists scroll internally to keep toolbars visible.

### Meta
- Title: `BOINC Manager — Tasks & Projects`

### Page Structure
1. **TopBar**
2. **Segmented Tabs** (Framer Motion underline + content transition)
3. **Panel Content**
4. **Details Drawer** (Sheet)

### Sections & Components
- Segmented Tabs:
  - Tabs: “Tasks”, “Projects” (content swaps with Framer Motion).
- Tasks Tab:
  - Toolbar: search, status filter (Running/Ready/Suspended/Error/Completed), sort (Progress/ETA/Name).
  - Table/List rows: name, project, status pill, progress bar, ETA.
  - Row actions: suspend/resume, abort (kebab menu).
- Projects Tab:
  - Toolbar: search, status filter.
  - Rows: project name, status pill, last contact, resource share.
  - Row actions: update, suspend/resume, no-new-tasks toggle.
- Details Drawer:
  - Header: item name + status.
  - Key fields: IDs, progress, timings.
  - Primary actions: context dependent; destructive action requires confirm dialog.

### Interaction Notes
- Optimistic UI is limited: show pending state while command executes; always reconcile with refresh.
- Empty/error states: clear copy + retry.

---

## Page 3 — Settings (/settings)
### Layout
- Single-column settings form with grouped cards; max width ~720px for readability.

### Meta
- Title: `BOINC Manager — Settings`

### Page Structure
1. **TopBar**
2. **Connection Card**
3. **Security Card**
4. **Appearance Card**

### Sections & Components
- Connection Card:
  - Inputs: host, port.
  - Password input: masked with reveal toggle.
  - Buttons: Test Connection, Save, Disconnect.
  - Inline validation + error message region.
- Security Card:
  - Toggle: “Store password in secure storage”.
  - Button: “Forget saved password”.
- Appearance Card:
  - Theme toggle (light/dark/system if supported).
  - Reduce motion toggle (disables most Framer Motion transitions).
  - Sidebar mode toggle (full/compact).

### Interaction Notes
- Test Connection shows spinner and a toast result.
- Saving persists non-secret prefs locally; secrets go to OS secure storage.
