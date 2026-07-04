# Decision Record: Repository Source Boundary And Pure Rule Engine

- Status: accepted
- Date: 2026-07-04
- Owner: Stephane Malho

## Context

Phase 3 of the roadmap requires scanning GitHub repositories without a local checkout, while local scans must stay deterministic and offline. Phase 4 will reuse the same engine from a SaaS backend hosted on a free tier (Render), with a possible fallback to compiling the engine to WASM and running it inside Next.js on Vercel.

## Decision

- All repository acquisition lives in `src/source/`: `RepositoryTarget` parsing, `LocalRepositorySource` (filesystem walk), and `GitHubRepositorySource` (GitHub REST API, no cloning).
- The rule engine (`analyzer`, `detectors`, `harness`, `report`) is pure: `RepositorySnapshot` carries file paths and prefetched file contents in memory, and performs no filesystem or network I/O. `harness::content_paths` declares which file contents the engine needs; every source prefetches exactly that list.
- Network access exists only in `src/source/github.rs`, is triggered only by explicit `github:owner/repo` or GitHub URL targets, and reads `GITHUB_TOKEN` from the environment without ever printing it.
- HTTP client: `ureq` (blocking, small dependency surface, rustls). Three REST calls per scan: repository metadata, recursive git tree, and raw contents of the readiness-relevant files.

## Consequences

- Positive: local and GitHub scans share one engine and produce the same report shape; the pure engine stays compilable to WASM, keeping the Render/Vercel hosting decision reversible; tests need no network.
- Negative: file contents are prefetched eagerly (negligible for readiness files); very large repositories with truncated GitHub trees are rejected with an explicit error instead of partially scanned.
- Follow-up: the SaaS backend (Phase 4) wraps the same engine; GitHub App tokens (Phase 5) replace `GITHUB_TOKEN` server-side.

## Links

- Source: `src/source/`, `docs/ROADMAP.md` Phase 3
- PR: agent/claude/bootstrap/repolens-cli
- Issue: none
