---
topic: multi-agent-workflow
last_reviewed: 2026-07-04
source_policy: official-docs-first
staleness_limit_days: 30
---

# Multi-Agent Workflow

## Goal

Allow multiple harnesses to work on the same project without conflicting worktrees, stale branches, or duplicated rule ownership.

## Branches

Use:

```txt
agent/<harness>/<ticket-or-task>/<short-slug>
```

Do not use long-running generic branches such as only `codex`, `claude`, or `gemini` for real tasks. They drift too easily.

Seed branches may exist only as examples:

```txt
agent/codex/template
agent/claude/template
agent/gemini/template
```

## Worktrees

Each harness gets a separate worktree.

```bash
git fetch origin main
git worktree add ../project-codex-task -b agent/codex/task/short-slug origin/main
```

## Sync Contract

Agents must run preflight:

```bash
scripts/agent-preflight.sh
```

If behind main:

```bash
scripts/agent-sync-main.sh
```

## Human Gate

Only the human maintainer merges into `main`.

CI verifies:

- branch naming
- branch is not behind `origin/main`
- required files exist
- no local settings or obvious secrets are committed
- project-specific checks if configured

