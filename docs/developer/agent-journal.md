# Agent Handoff Journal

This journal is a shared index for Codex, Claude Code, Gemini CLI, and local
agents. It points future agents to the Git history they should inspect before
continuing related work.

Canonical rules for maintaining this file live in
`docs/agent-rules/agent-handoff.md`.

Allowed `agent` values:

- `codex`
- `claude-code`
- `gemini-cli`
- `local-agent`

## 2026-07-05 - codex - landing refresh in progress

- agent: `codex`
- branch: `agent/codex/bootstrap/repolens-cli`
- PR: not opened yet
- status: local staged work, pending maintainer commit/push
- base commit before this landing work: `b0cd276`

Commits:

- pending - proposed headline: `feat(web): refresh landing page from Figma mockup`

Read:

```bash
git show --stat b0cd276
git diff --cached --stat
git diff --cached -- web/app/page.tsx web/app/layout.tsx web/app/globals.css web/lib/landing.ts web/components/landing
```

Notes:

- Rebuilt the home page as composed Next.js App Router sections instead of a
  single pasted JSX block.
- Added landing domain components under `web/components/landing/`, centralized
  landing content in `web/lib/landing.ts`, and kept styling token-driven through
  `web/app/globals.css`.
- Implemented the hero, metrics, features, multi-harness/report preview, install
  callout, header anchors, and the new SVG app icon/favicon.
- Removed the old `web/app/favicon.ico`; `web/app/icon.svg` is now the favicon
  declared in `web/app/layout.tsx`.
- Figma Make direct source extraction was limited by the connector, so final
  implementation used the user's screenshots plus pasted multi-harness excerpt.
- Validated with `scripts/agent-preflight.sh`, `cd web && npm run lint`,
  `cd web && npx tsc --noEmit`, `cd web && npm run test`, `cd web && npm run build`,
  and `git diff --check`.

## 2026-07-05 - codex - PR #12

- agent: `codex`
- branch: `agent/codex/bootstrap/repolens-cli`
- PR: https://github.com/stephanemalho/agentready/pull/12
- status: merged
- merged into `main` as `2a3ecd7b6b28f1d952eae3d11f9e6feb466f55d6`

Commits:

- `6bbf58a762cc867cb82353127e21f1f966bb999c` - fix(web): clarify missing API_URL scan setup
- `8957ff40c7c6ac1c36c2ab38700eb65fa1f70caa` - fix(web): sync package lock for Linux CI
- `338a9a165c53a0f77287657f368108c9ed960871` - fix(ci): reuse cached cargo tools

Read:

```bash
git show --stat 2a3ecd7b6b28f1d952eae3d11f9e6feb466f55d6
git show 6bbf58a762cc867cb82353127e21f1f966bb999c
git show 8957ff40c7c6ac1c36c2ab38700eb65fa1f70caa
git show 338a9a165c53a0f77287657f368108c9ed960871
```

Notes:

- Local dev failure was caused by `next dev` being started without `API_URL`.
- Web CI failure was caused by a macOS-generated lockfile missing Linux optional dependencies.
- Rust CI failure was caused by cached `cargo-audit` / `cargo-tarpaulin` binaries being reinstalled.
- GitHub checks were green before merge.

## 2026-07-05 - claude-code - PR #11

- agent: `claude-code`
- branch: `agent/claude/bootstrap/repolens-cli`
- PR: https://github.com/stephanemalho/agentready/pull/11
- status: merged
- merged into `main` as `f1ac91b42172040ca0c5dd31813aa979a562a12e`

Commits:

- `4fce93e9174423bc0695b526d9a8495edf90ac42` - feat(server): add axum HTTP API wrapping the readiness engine
- `ddb3928a88f893070d72034623d3714566d6b9e5` - feat(server): persist scans in Postgres with history endpoints
- `fe732cdf0b773649f0d4236876a3ffed71630c74` - feat(web): add Next.js SaaS front-end

Read:

```bash
git show --stat f1ac91b42172040ca0c5dd31813aa979a562a12e
git show 4fce93e9174423bc0695b526d9a8495edf90ac42
git show ddb3928a88f893070d72034623d3714566d6b9e5
git show fe732cdf0b773649f0d4236876a3ffed71630c74
```

Notes:

- Added the Phase 4 local SaaS shape: Rust API, Postgres-backed scan history,
  and Next.js web client.
- Read `docs/developer/running-the-project.md` before local full-stack testing.

## 2026-07-04 - gemini-cli - PR #10

- agent: `gemini-cli`
- branch: `agent/gemini/bootstrap/repolens-cli`
- PR: https://github.com/stephanemalho/agentready/pull/10
- status: merged
- merged into `main` as `e232f00cd496737282ff6a22b21acd03ea1ac472`

Commits:

- `f2d3c1b086b1abc66bdca255112a25de7bb65f92` - feat(ci): Add code coverage with cargo-tarpaulin

Read:

```bash
git show --stat e232f00cd496737282ff6a22b21acd03ea1ac472
git show f2d3c1b086b1abc66bdca255112a25de7bb65f92
```

Notes:

- Introduced Rust CI code coverage with `cargo-tarpaulin`.
- Later Rust CI cache behavior was adjusted in PR #12.
