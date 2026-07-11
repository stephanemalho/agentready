---
topic: multi-agent-workflow
last_reviewed: 2026-07-11
source_policy: official-docs-first
staleness_limit_days: 30
---

# Multi-Agent Workflow

## Goal

Allow each harness to coordinate multiple native agents on its assigned branch without stale branches, overlapping writes, or duplicated rule ownership.

## Branches

Each active branch is owned by one harness. Agents must never create branches, and each harness works only on its assigned branch:

```txt
agent/codex/bootstrap/repolens-cli
agent/claude/bootstrap/repolens-cli
agent/gemini/bootstrap/repolens-cli
```

Keeping one long-running branch per harness is the accepted trade-off here; the sync contract below keeps it aligned with `origin/main`.

## Native Agent Coordination

The harness coordinator may run multiple agents or subagents using the harness's native capabilities. Codex, Claude Code, and Gemini CLI do not need to expose identical delegation, isolation, nesting, or tool behavior.

Parallel writing is allowed only when the coordinator:

- assigns explicit, non-overlapping files to each writer
- keeps one writer responsible for shared contracts and shared files
- integrates and validates the combined result on the harness branch

Read-heavy exploration, review, and validation can be delegated without granting write ownership.

## Worktrees

Agent or subagent delegation stays in the current checkout by default. A request for parallel work never implies creating a branch, clone, or worktree.

Additional worktrees are exceptional and require explicit maintainer approval. When approved, the maintainer may use `scripts/create-agent-worktree.sh <harness> <explicit-path>` to check out an existing assigned branch. The helper must not be treated as the default agent startup path.

## Sync Contract

Agents must run preflight:

```bash
scripts/agent-preflight.sh
```

If behind main:

```bash
scripts/agent-sync-main.sh
```

## Handoff Journal

After preflight and before planning follow-up work, agents must read:

```txt
docs/developer/agent-journal.md
```

The journal is a short index into Git history. Entries must use one of these
agent identities:

```txt
codex
claude-code
gemini-cli
local-agent
```

Future agents should inspect the referenced commits with `git show` / `git log`
when those commits affect the task. The journal must not duplicate full diffs or
own canonical project rules; those stay in `docs/agent-rules/`.

## Human Gate

Only the human maintainer merges into `main`.

CI verifies:

- branch naming
- branch is not behind `origin/main`
- required files exist
- no local settings or obvious secrets are committed
- project-specific checks if configured
