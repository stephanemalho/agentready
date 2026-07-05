# AgentReady Web Client — Agent Entrypoint

Canonical project rules live at the repository root: read `../AGENTS.md` first, then the web client rules in `../docs/client/README.md` (architecture, style, clean-code, data-contract, testing) before changing anything here.

## Next.js Version Warning

This project uses Next.js 16, which has breaking changes over older versions — APIs, conventions, and file structure may differ from your training data. Read the relevant guide in `node_modules/next/dist/docs/` before writing any code. Heed deprecation notices.

## Quick Facts

- TypeScript strict, App Router, Server Components by default.
- Styling only through the design tokens in `app/globals.css`.
- All API access goes through `lib/api.ts` (server-only); the browser never calls the Rust API.
- Validation: `npm run lint && npx tsc --noEmit && npm run test && npm run build`.
