# Web Client Style Rules

## Single Source Of Style

All design tokens live in ONE place: `web/app/globals.css` (Tailwind CSS v4 CSS-first configuration) — the `@theme inline` mapping plus the `:root` (light) and `.dark` (dark) value blocks. Colors, radii, spacing, and typography are defined there as CSS variables and nowhere else.

Rebranding the product = editing that single file.

## Theming

- Three modes: **light / dark / system**, switched by `next-themes` (class strategy on `<html>`, provider in `web/components/ThemeProvider.tsx`, selector in the header: `web/components/ThemeMenu.tsx`).
- Every token must be defined in BOTH `:root` and `.dark`, including the landing tokens (`--agent-grid-*`, `--agent-signal-*`, `--harness-*`) and the semantic status/severity tokens. A token with a single value is a bug waiting for the other theme.
- Never hardcode `dark:` styles to fake a theme; the token layer does the switching.

## Rules

- Never hardcode a color, radius, shadow, or font in a component (`bg-[#3b82f6]` is forbidden). Use token-backed utilities (`bg-primary`, `text-muted-foreground`, `rounded-lg`).
- shadcn/ui components consume the same tokens; customize the theme, not the generated component code, unless behavior must change.
- Semantic status colors are tokens too: `--color-status-pass`, `--color-status-warn`, `--color-status-fail`, and severity tokens (`info`, `low`, `medium`, `high`). Components map statuses to these tokens through shared helpers, never inline.
- Dark mode comes from the token layer (CSS variables switched on `.dark`), not from per-component conditionals.
- Icons: lucide-react only, sized with Tailwind utilities.

## Component Conventions

- shadcn/ui primitives live in `web/components/ui/` as generated; treat them as the project's component library. Since shadcn 4.13 they are built on **Base UI** (`@base-ui/react`) — not Radix — and compose via the `render` prop instead of `asChild`. The `shadcn` package stays in dependencies because `globals.css` imports `shadcn/tailwind.css`.
- Domain components compose primitives; they never re-implement buttons, cards, badges, or tables.
- Class merging goes through the `cn()` helper (`clsx` + `tailwind-merge`) — never manual string concatenation.
