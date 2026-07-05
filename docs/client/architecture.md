# Web Client Architecture

## Stack

Next.js 16 (App Router, Turbopack, React 19 + React Compiler), TypeScript strict, Tailwind CSS v4, shadcn/ui, Zod. No state manager: Server Components and Server Actions cover the current scope.

## Folder Responsibilities

```txt
web/
  app/                 # routes ONLY: pages, layouts, loading/error boundaries, server actions
  components/ui/       # shadcn/ui primitives (generated; customize via tokens, not forks)
  components/          # domain components (pure, typed props, no data fetching)
  lib/
    api.ts             # THE single API client (server-only fetch to the Rust API)
    contract.ts        # TS types + Zod schemas of the engine JSON contract
    score.ts           # pure scoring helpers (per-harness score)
    env.ts             # validated environment access (API_URL)
  app/globals.css      # design tokens (see style.md)
```

- `app/` files stay thin: compose components and call `lib/`. No business logic in routes.
- Domain components receive data via props and never fetch. Fetching happens in Server Components (pages) or Server Actions, always through `lib/api.ts`.
- `lib/` is framework-light: pure TypeScript, no React imports (testable in isolation).

## Rendering Rules

- Server Components by default. Add `"use client"` only for real interactivity (form submission state, toggles). Keep client components leaf-sized.
- Every page that fetches gets a `loading.tsx` (the Render free tier can be slow; the UI must show progress, never freeze).
- Error states use `error.tsx` boundaries with actionable messages.

## Data Flow (BFF Pattern)

```txt
Browser -> Next.js server (RSC / Server Action) -> Rust API (Render) -> GitHub/Postgres
```

- The browser NEVER calls the Rust API directly. `API_URL` is a server-only env var (no `NEXT_PUBLIC_` prefix).
- All responses are validated with Zod at the boundary (`lib/contract.ts`) before use.
- Scan submission is a Server Action that calls `POST /api/scans` then redirects to `/scans/{id}`.

## Deployment Shape (Lot D)

Vercel project with Root Directory = `web/`; `API_URL` points to the Render service. Builds are skipped when nothing under `web/` changed (Ignored Build Step).
