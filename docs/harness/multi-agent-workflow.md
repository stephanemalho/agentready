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

Agents must never create branches. Each harness works only on its assigned branch:

```txt
agent/codex/bootstrap/repolens-cli
agent/claude/bootstrap/repolens-cli
agent/gemini/bootstrap/repolens-cli
```

Keeping one long-running branch per harness is the accepted trade-off here; the sync contract below keeps it aligned with `origin/main`.

## Worktrees

Each harness gets a separate worktree, checked out on its assigned branch.

```bash
scripts/create-agent-worktree.sh codex ../repolens-codex
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

