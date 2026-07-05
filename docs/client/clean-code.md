# Web Client Clean Code Rules

## TypeScript

- `strict: true`, no exceptions. `any` is forbidden unless accompanied by a comment explaining why no type is possible.
- Exported functions and components declare explicit return/prop types.
- Types flow from `lib/contract.ts` (the engine contract); never redeclare a shape that already exists there.

## DRY

- One API client (`lib/api.ts`). No `fetch` calls anywhere else.
- One contract module (`lib/contract.ts`). No duplicated interfaces.
- One scoring implementation (`lib/score.ts`). UI never re-computes scores inline.
- One status/severity → token mapping helper, shared by all components.

## Separation Of Concerns

- Routes (`app/`) compose; components render; `lib/` computes and fetches. A file doing two of these is a smell.
- Domain components are pure: props in, JSX out. No fetching, no env access, no side effects.
- Server Actions live next to the route that uses them (`app/**/actions.ts`) and delegate to `lib/api.ts` immediately.

## Size And Naming

- A component file stays under ~120 lines; past that, extract.
- Components: `PascalCase` files named after the component (`ScoreGauge.tsx`). Lib modules: `camelCase` functions, `kebab-case` files only for multi-word non-component modules.
- Props interfaces are named `<Component>Props` and declared in the component file.

## Imports

- Use the `@/` alias; no `../../..` chains.
- Order: react/next, external packages, `@/lib`, `@/components`, relative. ESLint enforces this.
