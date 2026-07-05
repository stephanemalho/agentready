# AgentReady Web Client Rules

Canonical rules for all work under `web/` (the Next.js front-end of the SaaS, roadmap Phase 4+).

Every agent must read the relevant theme file below before implementing front-end work. These rules have the same authority as `docs/agent-rules/` for the Rust side.

## Themes

| Theme | File | Read before |
|---|---|---|
| Folder structure, RSC vs client, data flow | `architecture.md` | Any new page, component, or data access |
| Design tokens, Tailwind/shadcn conventions | `style.md` | Any styling or UI change |
| DRY, separation of concerns, TypeScript rules | `clean-code.md` | Any code change |
| Engine JSON contract, Zod schemas, score algorithm | `data-contract.md` | Anything touching API data |
| Test policy and commands | `testing.md` | Any behavior change |

## Non-Negotiables

- TypeScript `strict` everywhere; no `any` without a written justification comment.
- Server Components by default; `"use client"` only for real interactivity.
- The browser never calls the Rust API directly (BFF pattern; see `architecture.md`).
- All styling goes through the design tokens in `web/app/globals.css` (see `style.md`).
- The engine JSON contract is the single source of truth for types (see `data-contract.md`).

## Validation Commands

```bash
cd web
npm run lint
npx tsc --noEmit
npm run test
npm run build
```
