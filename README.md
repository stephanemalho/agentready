# RepoLens

A Rust-powered repository analysis CLI for generating developer onboarding, project health, and AI-agent context reports.

RepoLens is local-first by design. V1 scans files on disk, respects `.gitignore`, and does not call external AI services.

## Quick Start

```bash
cargo run -- scan .
cargo run -- scan . --format json
cargo run -- doctor .
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
```

## Current Signals

The first version detects:

- Rust projects through `Cargo.toml`
- Node.js projects through `package.json`
- Python projects through `pyproject.toml` or `requirements.txt`
- Go projects through `go.mod`
- Docker usage through `Dockerfile`
- GitHub Actions through `.github/workflows/`
- multi-harness AI agent setup through `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md`
- basic health checks for README, `.gitignore`, CI, license, and tests

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Agent Workflow

This repository uses a multi-harness agent template. `AGENTS.md` is the canonical source of truth, and harness-specific files for Codex, Claude Code, and Gemini CLI should reference the shared project rules instead of duplicating them.

Agents work on branches matching:

```txt
agent/<harness>/<ticket-or-task>/<short-slug>
```

The human maintainer reviews and merges to `main`.
