# RepoLens

A Rust CLI for offline repository analysis and coding-agent harness readiness checks.

RepoLens checks whether a local project is understandable for developers and ready to be worked on by coding-agent harnesses such as Codex, Claude Code, and Gemini CLI.

RepoLens is local-first by design. It scans files on disk, respects `.gitignore`, and does not call external AI services.

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
```

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

Agents work on branches matching:

```txt
agent/<harness>/<ticket-or-task>/<short-slug>
```

The human maintainer reviews and merges to `main`.
