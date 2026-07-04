# RepoLens

A Rust CLI for repository analysis and coding-agent harness readiness checks.

RepoLens checks whether a project is understandable for developers and ready to be worked on by coding-agent harnesses such as Codex, Claude Code, and Gemini CLI. It scans local repositories or public GitHub repositories.

RepoLens is local-first by design. Local scans read files on disk, respect `.gitignore`, and never touch the network. Only explicit `github:owner/repo` or GitHub URL targets call the GitHub API, and no AI service is ever called.

## Quick Start

```bash
cargo run -- scan .
cargo run -- scan . --format json
cargo run -- doctor .
cargo run -- harness .
```

## Commands

```bash
# Markdown report
repolens scan /path/to/repo

# JSON report
repolens scan /path/to/repo --format json

# Write a report to disk
repolens scan /path/to/repo --output repolens-report.md

# Compact health summary
repolens doctor /path/to/repo

# Multi-harness readiness report
repolens harness /path/to/repo

# Harness-specific readiness checks
repolens harness /path/to/repo --harness codex
repolens harness /path/to/repo --harness claude
repolens harness /path/to/repo --harness gemini

# JSON readiness report
repolens harness /path/to/repo --format json

# Scan a public GitHub repository without cloning it
repolens harness github:owner/repo
repolens harness https://github.com/owner/repo
repolens scan github:owner/repo --format json
```

GitHub targets use the GitHub REST API. Set the optional `GITHUB_TOKEN`
environment variable to raise the API rate limit; its value is never printed.

## Current Signals

The first version detects:

- Rust projects through `Cargo.toml`
- Node.js projects through `package.json`
- Python projects through `pyproject.toml` or `requirements.txt`
- Go projects through `go.mod`
- Docker usage through `Dockerfile`
- GitHub Actions through `.github/workflows/`
- multi-harness coding-agent setup through `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md`
- basic health checks for README, `.gitignore`, CI, license, and tests

## Harness Readiness

`repolens harness` checks whether a repository has the files and conventions needed for serious multi-agent work:

- canonical `AGENTS.md`
- shared rules under `docs/agent-rules/`
- shared workflows under `docs/skills/`
- preflight and main-sync scripts
- CI branch policy workflow
- Codex `.codex/config.toml` and `.agents/skills/*/SKILL.md`
- Claude `CLAUDE.md`, `.claude/settings.json`, and `.claude/rules/`
- Gemini `GEMINI.md`, `.gemini/settings.json`, `.gemini/agents/`, and `.gemini/commands/`

Every check is backed by a stable rule ID (for example `shared.agents_md.exists` or `gemini.settings.context_agents`) with a severity (`info`, `low`, `medium`, `high`), evidence paths, an official-doc source, and remediation guidance for warnings and failures. These fields appear in both the Markdown and JSON reports, so the JSON output can be consumed as a stable contract.

The readiness report validates project files and configuration only. RepoLens does not run, call, or embed any AI model.

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Agent Workflow

This repository uses a multi-harness agent template. `AGENTS.md` is the canonical source of truth, and harness-specific files for Codex, Claude Code, and Gemini CLI should reference the shared project rules instead of duplicating them.

Agents never create branches. Each harness works only on its assigned branch:

```txt
agent/codex/bootstrap/repolens-cli
agent/claude/bootstrap/repolens-cli
agent/gemini/bootstrap/repolens-cli
```

The human maintainer reviews and merges to `main`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
