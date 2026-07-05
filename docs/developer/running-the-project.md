# Running AgentReady (Daily Workflow)

Day-to-day steps to run the backend (Rust API) and the frontend (Next.js) locally, plus the production picture.

## Prerequisites (one-time)

- Rust toolchain (`rustup`), Node.js >= 20, Docker Desktop.
- First-time database container:

```bash
docker run -d --name agentready-pg \
  -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=agentready_test \
  -p 5432:5432 postgres:16
```

## Daily: Local Development

### 1. Start the database (optional but recommended)

Open Docker Desktop (menu bar whale → wait for "Engine running"), then:

```bash
docker start agentready-pg
```

Skip this step if you only need stateless scans (no history).

### 2. Start the backend (repository root)

```bash
# With scan history:
DATABASE_URL=postgres://postgres:postgres@localhost:5432/agentready_test \
  cargo run -p agentready-server

# Or stateless (no database; history endpoints answer 503):
cargo run -p agentready-server
```

Listens on `http://localhost:8080` (override with `PORT`). Sanity check: `curl localhost:8080/health` → `ok`. Optional: `GITHUB_TOKEN=...` raises the GitHub API rate limit.

### 3. Start the frontend

One-time setup: create `web/.env.local` (git-ignored) so `API_URL` is always set:

```bash
echo "API_URL=http://localhost:8080" > web/.env.local
```

Then the daily command is just:

```bash
cd web
npm run dev
```

Open `http://localhost:3000`, paste `github:owner/repo`, scan. `API_URL` is server-only and required.

If the UI says the scan service is not configured, `next dev` was started
without `API_URL` (missing `.env.local`). Create the file above, then stop the
dev server (Ctrl+C) and run `npm run dev` again — Next.js reads environment
variables only at startup.

### 4. End of day

```bash
# Ctrl+C on both dev servers, then:
docker stop agentready-pg
```

The container keeps its data; `docker rm -f agentready-pg` wipes it (migrations re-run automatically on next connect).

## Daily: CLI Only

No server needed for the CLI:

```bash
cargo run -p agentready -- harness .
cargo run -p agentready -- harness github:owner/repo --format json
```

## Validation Before Any Commit

```bash
cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings \
  && cargo test --workspace && cargo build --release --workspace
cd web && npm run lint && npx tsc --noEmit && npm run test && npm run build
```

Database tests run only when `TEST_DATABASE_URL` is set (see `docs/agent-rules/testing.template.md`).

## Production

> **Not deployed yet.** This is the target architecture (roadmap Phase 4, lot D); the accounts and configuration below still have to be created. Update this section once the deployment is live.

| Piece | Platform | Configuration |
|---|---|---|
| Backend API | Render (free tier) | Builds the root `Dockerfile`; env: `DATABASE_URL` (Neon), optional `GITHUB_TOKEN`; a GitHub Actions keep-alive will ping `/health` every 10 min to prevent spin-down |
| Database | Neon (free Postgres) | Connection string goes into Render's `DATABASE_URL`; migrations run automatically at server startup |
| Frontend | Vercel (hobby tier) | Root Directory = `web/`; env: `API_URL` = the Render service URL; Ignored Build Step so only `web/**` changes trigger deploys |

Planned production flow: push to `main` → Render rebuilds the API image, Vercel redeploys the front (if `web/` changed). No manual deploy steps.
