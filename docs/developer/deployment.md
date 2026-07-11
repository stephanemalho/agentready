# Deployment (Production)

Live since 2026-07-05. Total hosting cost: 0 EUR.

| Piece | Platform | URL / Identifier |
|---|---|---|
| Backend API | Render free tier (Frankfurt) | https://agentready-zkx0.onrender.com |
| Database | Neon free Postgres | project `agentready` |
| Frontend | Vercel hobby | https://agentready-psi.vercel.app |

## How It Deploys

Push to `main` → Render rebuilds the root `Dockerfile` (Rust build, 10-20 min) and Vercel redeploys the front (~2 min). No manual steps.

## Configuration Reference

### Render (service `agentready`)

- Language: Docker (root `Dockerfile`), branch `main`, instance type Free.
- Health Check Path: `/health`.
- Environment variables:
  - `DATABASE_URL`: the Neon connection string (pooling disabled; the server has its own pool and runs migrations at startup).
  - `GITHUB_TOKEN`: fine-grained PAT, scope **Public repositories (read-only)**, no extra permissions. Required in practice: Render's shared egress IPs exhaust GitHub's anonymous 60 req/h limit immediately; the token raises it to 5000 req/h. It cannot read private repositories (that is roadmap Phase 5, via a GitHub App). **Renew before expiry** and update it under service → Environment.

### Neon

- Free Postgres 16+, region Frankfurt. The database suspends after ~5 min idle; the first query after that adds ~1 s.
- Migrations are applied automatically by the server at startup (`server/migrations/`).

### Vercel (project `agentready`)

- Root Directory: `web/` (crucial setting), framework Next.js.
- Environment variable: `API_URL` = the Render URL, no trailing slash. Server-only — the browser never calls the Rust API (BFF).

## Free-Tier Behavior (By Design)

There is NO 24/7 keep-alive. The Render service spins down after ~15 min without traffic; the web client wakes it on page visit and shows the state in the header dot (see `docs/client/architecture.md`, "Backend Wake-On-Visit"). First scan after a quiet period takes ~1 min; the scan form stays disabled with an explanation until the engine is up.

Other known limits: Render free = 750 h/month (fine for one service); Neon free storage quota is generous for scan history.

## Smoke Test After A Deploy

```bash
curl https://agentready-zkx0.onrender.com/health        # -> ok (may take ~1 min if asleep)
curl -X POST https://agentready-zkx0.onrender.com/api/scans \
  -H "Content-Type: application/json" \
  -d '{"target":"github:stephanemalho/agentready"}'      # -> JSON report with scan_id
```

Then open the Vercel URL: the header dot must turn green and a scan of a public repo must render a report page.

## Troubleshooting

- Header dot stays red: check Render service Logs (usually a bad `DATABASE_URL`).
- "rate limit exceeded" on scans: `GITHUB_TOKEN` missing or expired on Render.
- "404: the repository does not exist or is private": expected for private repos — the SaaS only scans public repositories until Phase 5.
- Vercel build fails on `npm ci`: regenerate the lockfile (`cd web && rm -rf node_modules package-lock.json && npm install`) — npm on macOS sometimes drops Linux/WASM optional entries.
