# Workflow: Sync With Main

Use this workflow before implementation and before opening a pull request.

## Commands

```bash
git fetch origin main
git rev-list --count HEAD..origin/main
```

If the count is greater than `0`, sync:

```bash
scripts/agent-sync-main.sh
```

Then verify:

```bash
scripts/agent-preflight.sh
```

## Conflict Handling

If rebase or merge conflicts occur:

1. Stop.
2. List conflicted files.
3. Explain what changed on `main`.
4. Ask for human direction if the conflict touches business logic, migrations, or security.

## PR Note

The pull request must mention:

- last sync command
- whether the branch is ahead/behind `origin/main`
- validation commands run after sync

