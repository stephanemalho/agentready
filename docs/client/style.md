# Web Client Style Rules

## Single Source Of Style

All design tokens live in ONE place: the `@theme` block of `web/app/globals.css` (Tailwind CSS v4 CSS-first configuration). Colors, radii, spacing, and typography are defined there as CSS variables and nowhere else.

Rebranding the product = editing that single block.

## Rules

- Never hardcode a color, radius, shadow, or font in a component (`bg-[#3b82f6]` is forbidden). Use token-backed utilities (`bg-primary`, `text-muted-foreground`, `rounded-lg`).
- shadcn/ui components consume the same tokens; customize the theme, not the generated component code, unless behavior must change.
- Semantic status colors are tokens too: `--color-status-pass`, `--color-status-warn`, `--color-status-fail`, and severity tokens (`info`, `low`, `medium`, `high`). Components map statuses to these tokens through shared helpers, never inline.
- Dark mode comes from the token layer (CSS variables switched on `.dark`), not from per-component conditionals.
- Icons: lucide-react only, sized with Tailwind utilities.

## Component Conventions

- shadcn/ui primitives live in `web/components/ui/` as generated; treat them as the project's component library.
- Domain components compose primitives; they never re-implement buttons, cards, badges, or tables.
- Class merging goes through the `cn()` helper (`clsx` + `tailwind-merge`) — never manual string concatenation.
