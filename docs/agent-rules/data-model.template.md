# AgentReady Data Model Rules

## Data Ownership

| Entity | Owner | Source of truth |
|---|---|---|
| `RepoAnalysis` | AgentReady analyzer | Repository files (local or GitHub) |
| `DetectedStack` | AgentReady detectors | File marker evidence |
| `HealthChecks` | AgentReady analyzer | Repository files (local or GitHub) |
| Markdown report | AgentReady report renderer | `RepoAnalysis` |
| JSON report | AgentReady report renderer | `RepoAnalysis` |
| `repositories` table | `server/` (SaaS) | GitHub source metadata |
| `scans` table | `server/` (SaaS) | Engine reports (score, summary, full JSON) |
| `findings` table | `server/` (SaaS) | Harness checks of a scan |

## Database (SaaS Only)

- The CLI has no database. Only `server/` uses Postgres, and it stays fully functional without `DATABASE_URL` (stateless mode: scans work, history endpoints answer 503).
- Schema and migrations live in `server/migrations/` and run automatically at startup via sqlx.
- Tables: `repositories` (unique per provider/owner/name), `scans` (per-scan score, summary, commit SHA, full report as JSONB), `findings` (one row per harness check, for future per-rule queries).

## Migration Rules

- Migrations are append-only SQL files in `server/migrations/`; never edit an applied migration, add a new one.
- Keep schema changes additive and backward compatible whenever possible.

## API/Data Contracts

- Contract files: Rust serializable structs used for JSON output.
- Generated types: none.
- Compatibility policy: avoid unnecessary JSON field renames once released; additive fields are preferred.

## Test Data

- Fixtures location: temporary directories in tests; reusable fixtures may live under `tests/fixtures/`.
- Seed command: not applicable.
- Data that must not be used in tests: real secrets, private customer repositories, private keys, tokens, or certificate material.
