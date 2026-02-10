# Page Design Spec — Hello World (Zinc Dark)

## Global (applies to all pages)

### Layout
- Use a hybrid Flexbox + max-width container approach.
- Default (desktop-first): center content both vertically and horizontally with generous spacing.
- Responsive: allow vertical stacking and safe padding on small screens (Android) without changing the visual hierarchy.

### Meta Information
- Title: "Hello World"
- Description: "A minimal Tauri app using React + Vite and shadcn/ui."
- Open Graph:
  - og:title: "Hello World"
  - og:description: "A minimal cross-platform Tauri 2.0 app."
  - og:type: "website" (WebView context)

### Global Styles (Zinc dark theme)
- Background: Zinc near-black (e.g., Tailwind `zinc-950`).
- Surface/Card: darker Zinc elevated surface (e.g., `zinc-900`) with subtle border (e.g., `zinc-800`).
- Text:
  - Primary: near-white (e.g., `zinc-50`).
  - Secondary: muted (e.g., `zinc-400`).
- Typography scale:
  - H1: 32–40px (desktop), 28–32px (mobile)
  - Body: 14–16px
- Buttons/Interactive states (if present as part of shadcn baseline styles):
  - Focus ring visible on dark background (Zinc-compatible, high contrast).
  - Hover: subtle brightness increase on surfaces; avoid strong color accents.

## Page: Hello World

### Page Structure
- Single-page, centered card layout.
- Composition: background → centered container → card → text stack.

### Sections & Components
1. App Background
   - Full-viewport background color using the Zinc dark token.
   - Safe padding (e.g., 24px desktop, 16px mobile).

2. Centered Container
   - Max width ~560–640px.
   - Flex column with vertical spacing; center aligned.

3. Hello World Card (shadcn/ui “Card”)
   - Card container with subtle border and slight shadow.
   - Internal padding ~24–32px.

4. Greeting Content
   - Headline: “Hello World” (H1).
   - Subtitle: one short sentence describing the stack (React + Vite + Tailwind + shadcn/ui) in muted text.

### Interaction & Motion
- No required interactions.
- If transitions are enabled globally, keep them subtle (150–200ms) for hover/focus only.
