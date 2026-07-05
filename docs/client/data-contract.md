# Web Client Data Contract

## Source Of Truth

The Rust engine's JSON output is the single source of truth. TypeScript types and Zod schemas in `web/lib/contract.ts` mirror it exactly; the front-end never invents fields.

Producers (do not change without updating both sides in the same PR):

- `POST /api/scans` → `{ target, scan_id?, analysis, harness }` (`server/src/lib.rs::ScanResponse`)
- `GET /api/repositories/{owner}/{repo}/scans` → `ScanSummary[]` (`server/src/store.rs`)
- `GET /api/scans/{id}` → `StoredScan` with `report = { analysis, harness }` (`server/src/store.rs`)

Key shapes (see `src/harness/mod.rs` and `src/analyzer/mod.rs` for the Rust originals):

- `HarnessCheck`: `harness` (`shared|codex|claude|gemini`), `id` (stable rule id), `severity` (`info|low|medium|high`), `status` (`pass|warn|fail`), `title`, `message`, `evidence[]`, `source`, `remediation?`
- `HarnessReadinessReport`: `root`, `score` (0-100), `summary {passed, warnings, failed}`, `checks[]`
- `RepoAnalysis.source`: `provider` (`local|github`), `owner?`, `repo?`, `default_branch?`, `commit_sha?`

## Validation Policy

- Every API response is parsed with the matching Zod schema before use; a mismatch throws early with a clear error instead of rendering garbage.
- The contract is additive (roadmap Phase 2 rule): unknown extra fields must not break parsing (`.passthrough()` where sensible), and removed/renamed fields are a breaking change requiring a synchronized PR.

## Score Algorithm

The engine only ships the global score. Per-harness scores (roadmap Phase 4 UI requirement) are computed in `web/lib/score.ts` with the exact formula of `src/harness/mod.rs::score`:

```txt
score = round((passed + 0.5 * warnings) / total * 100)   // 0 if total == 0
```

applied to the checks filtered by harness. This function is pure and unit-tested; no component computes scores inline.
