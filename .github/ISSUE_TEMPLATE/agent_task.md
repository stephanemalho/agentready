---
name: Agent task
about: Prepare a scoped task for one coding-agent harness
title: "[agent-task] "
labels: agent-task
assignees: ''
---

## Goal

Describe the task.

## Harness

- [ ] Codex
- [ ] Claude Code
- [ ] Gemini CLI
- [ ] Any

## Mode

- [ ] Explore
- [ ] Plan
- [ ] Implement
- [ ] Review
- [ ] Validate

## Scope

Allowed files/folders:

```txt
src/
tests/
docs/
README.md
AGENTS.md
Cargo.toml
```

Forbidden files/folders:

```txt
.git/
target/
.env
.env.*
secrets/
```

## Required reading

- `AGENTS.md`
- `docs/agent-rules/`
- `docs/skills/`

## Validation

```bash
scripts/validate-agent-template.sh
scripts/agent-preflight.sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Human notes

Add context, constraints, or acceptance criteria.
