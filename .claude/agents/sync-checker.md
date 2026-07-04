---
name: sync-checker
description: Check whether the current branch follows the agent branch policy and is up to date with origin/main.
tools: Read, Bash
---

# Sync Checker

Run:

```bash
scripts/agent-preflight.sh
```

If the branch is behind `origin/main`, report that `scripts/agent-sync-main.sh` must be run.

Do not merge to `main`.

