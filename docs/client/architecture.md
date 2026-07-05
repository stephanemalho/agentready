# Web Client Architecture

## Stack

Next.js 16 (App Router, Turbopack, React 19 + React Compiler), TypeScript strict, Tailwind CSS v4, shadcn/ui, Zod. No state manager: Server Components and Server Actions cover the current scope.

## Folder Responsibilities

```txt
web/
  app/                 # routes ONLY: pages, layouts, loading/error boundaries, server actions
  app/api/health/      # BFF health proxy (the browser never calls the Rust API)
  components/ui/       # shadcn/ui primitives (generated, Base UI based; customize via tokens)
  components/          # domain components (pure, typed props, no data fetching)
  components/landing/  # landing page sections (server components; content from lib/landing.ts)
  lib/
    api.ts             # THE single API client (server-only fetch to the Rust API)
    contract.ts        # TS types + Zod schemas of the engine JSON contract
    score.ts           # pure scoring helpers (per-harness score)
    landing.ts         # landing page content (uses the contract vocabulary)
    env.ts             # validated environment access (API_URL)
  app/globals.css      # design tokens, light + dark (see style.md)
```

Shared enum→label/class mappings live in `web/components/display.ts`; landing and report components consume it — no local status/severity/harness maps.

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

## Backend Wake-On-Visit

The Rust API sleeps on the Render free tier; there is deliberately NO 24/7 keep-alive cron. Instead the client wakes it on demand:

- `app/api/health/route.ts` proxies `GET /health` server-side (BFF preserved) — every call also wakes the Render service.
- `components/BackendStatusProvider.tsx` (client context in the root layout) polls that route: fast (5 s) while the backend wakes, ~90 s of failures → `offline`, then a 10-minute heartbeat while the tab is **visible** only (`visibilitychange`), so an abandoned tab lets the backend sleep again.
- Consumers: `BackendStatusDot` (header, next to the version badge: checking / waking / online / offline) and `ScanForm` (input disabled with an explanation while the engine is waking or unreachable).

## Deployment Shape (Lot D)

Vercel project with Root Directory = `web/`; `API_URL` points to the Render service. Builds are skipped when nothing under `web/` changed (Ignored Build Step).
