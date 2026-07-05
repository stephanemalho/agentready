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

## After A PR Merge

Pull requests are merged into `main` with merge commits (squash and rebase merging are disabled on the repository, and `main` must not require linear history). After each merge, realign the assigned branch:

```bash
git fetch origin
git rebase origin/main
git push --force-with-lease
```

Warnings like `skipped previously applied commit` during the rebase are normal: git drops commits whose changes already landed in `main`.

Never run `git pull` and never click the GitHub "Update branch" button on an assigned branch after a rebase: both merge the old remote history back and reintroduce duplicated commits.

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

