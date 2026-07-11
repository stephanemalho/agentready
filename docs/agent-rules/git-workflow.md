# Git Workflow Rules

## Branches

`main` is the protected integration branch.

Each active agent branch is owned by one harness. Agents must never create branches, and each harness works only on its assigned branch:

```txt
agent/codex/bootstrap/repolens-cli
agent/claude/bootstrap/repolens-cli
agent/gemini/bootstrap/repolens-cli
```

The harness coordinator may run multiple native agents or subagents on that branch. Parallel writers must have explicit, non-overlapping file ownership, and shared contracts or files must have one writer at a time.

A request for agents, subagents, or parallel work does not authorize a new branch, clone, or worktree. Additional worktrees require explicit approval from the human maintainer. If a checkout is on the wrong branch, stop and ask the maintainer for direction rather than switching away from unrelated work.

Harness-specific delegation and isolation behavior belongs in `docs/harness/`; do not impose one harness's agent model on another.

## Main Synchronization

Agents must be up to date with `origin/main`:

- before implementation
- before opening a pull request
- before reporting completion

Use:

```bash
scripts/agent-preflight.sh
scripts/agent-sync-main.sh
scripts/agent-preflight.sh
```

## Merge Authority

Agents must not merge to `main`.

The human maintainer:

1. Reviews the PR branch against `main`.
2. Checks CI results.
3. Runs any local validation they want.
4. Merges or asks for changes.

## Conflict Policy

If syncing with `main` creates conflicts, stop and report:

- conflicted files
- likely cause
- proposed resolution

Do not invent a conflict resolution for domain logic without human review.
