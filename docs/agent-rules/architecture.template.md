# AgentReady Architecture Rules

## Layers

```txt
CLI -> Source (local filesystem / GitHub API) -> Analyzer -> Detectors / Harness Readiness -> Report Renderers
```

## Allowed Dependencies

- `src/main.rs` may only parse the top-level CLI and delegate to `src/cli.rs`.
- `src/cli.rs` may parse targets, load snapshots through `src/source/`, call the engine, and perform explicit user-requested output writes.
- `src/source/` owns all repository acquisition: target parsing, local filesystem walking, and GitHub API access. It is the only module allowed to perform filesystem or network I/O.
- `src/analyzer/` owns the pure `RepositorySnapshot` data type and analysis assembly.
- `src/detectors/` owns pure stack and marker detection from relative file paths.
- `src/harness/` owns coding-agent harness readiness checks and declares the file contents it needs via `content_paths`.
- `src/report/` owns Markdown, JSON, and doctor output formatting.
- The engine (analyzer, detectors, harness, report) must stay pure: no filesystem or network I/O, so it remains WASM-compilable and reusable by the SaaS.
- Network access is triggered only by explicit `github:` or GitHub URL targets. Local scans must never touch the network.
- `server/` (`agentready-server`) is the SaaS HTTP API: an axum service that wraps the engine crate. It only accepts GitHub targets (never local paths), and blocking engine calls run in `spawn_blocking`.
- No AI SDK anywhere.

## File Routing

| Concern | Location |
|---|---|
| CLI parsing and command dispatch | `src/cli.rs` |
| Repository acquisition (local walk, GitHub API) | `src/source/` |
| Repository analysis | `src/analyzer/` |
| Stack detection | `src/detectors/` |
| Harness readiness checks | `src/harness/` |
| Report rendering | `src/report/` |
| Binary entrypoint | `src/main.rs` |
| Integration CLI tests | `tests/` |
| SaaS HTTP API (axum) | `server/` |

## Refactor Rules

- Prefer local changes over broad rewrites.
- Preserve public contracts unless the task explicitly changes them.
- Update docs when architecture changes.
- Add a decision record for important tradeoffs.
- Keep structs serializable when they are part of JSON output.
- Keep report output stable enough for tests and future agent consumption.
