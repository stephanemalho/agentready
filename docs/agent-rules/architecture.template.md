# RepoLens Architecture Rules

## Layers

```txt
CLI -> Analyzer -> Detectors -> Report Renderers -> Filesystem
```

## Allowed Dependencies

- `src/main.rs` may only parse the top-level CLI and delegate to `src/cli.rs`.
- `src/cli.rs` may call analyzer and report rendering code, and may perform explicit user-requested output writes.
- `src/analyzer/` owns repository walking, file inventory, and analysis assembly.
- `src/detectors/` owns pure stack and marker detection from relative file paths.
- `src/report/` owns Markdown, JSON, and doctor output formatting.
- Detector and report modules must not perform filesystem or network I/O.
- V1 must not include an AI SDK or network-backed analysis layer.

## File Routing

| Concern | Location |
|---|---|
| CLI parsing and command dispatch | `src/cli.rs` |
| Repository analysis | `src/analyzer/` |
| Stack detection | `src/detectors/` |
| Report rendering | `src/report/` |
| Binary entrypoint | `src/main.rs` |
| Integration CLI tests | `tests/` |

## Refactor Rules

- Prefer local changes over broad rewrites.
- Preserve public contracts unless the task explicitly changes them.
- Update docs when architecture changes.
- Add a decision record for important tradeoffs.
- Keep structs serializable when they are part of JSON output.
- Keep report output stable enough for tests and future agent consumption.
