# Git Workflow Rules

## Branches

`main` is the protected integration branch.

Agents must never create branches. Each harness works only on its assigned branch:

```txt
agent/codex/bootstrap/repolens-cli
agent/claude/bootstrap/repolens-cli
agent/gemini/bootstrap/repolens-cli
```

If an agent is not on its assigned branch, it must switch to it with `git switch`, never create a new one.

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

