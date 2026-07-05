# Web Client Testing Rules

## Commands

```bash
cd web
npm run test        # vitest run
npm run lint
npx tsc --noEmit
npm run build
```

## Policy

- `lib/` (contract parsing, scoring, env) is pure TypeScript and aims for full coverage: every exported function has unit tests, including error paths (invalid API payloads must fail Zod parsing).
- Domain components with logic (status mapping, conditional rendering) get Testing Library tests; purely presentational wrappers do not need one.
- No network in tests: API responses are fixtures built from the documented contract (`data-contract.md`). Keep one canonical fixture module reused by all tests (DRY).
- shadcn/ui primitives are not tested (upstream responsibility).
- Browser E2E (Playwright) arrives with the deployment lot; do not add it ad hoc.

## Conventions

- Test files live next to the code: `score.test.ts` beside `score.ts`, `FindingRow.test.tsx` beside `FindingRow.tsx`.
- Follow the same do-not-weaken rule as the Rust side: never delete or loosen a test to make a change pass (`docs/agent-rules/testing.template.md`).
