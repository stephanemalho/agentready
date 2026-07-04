# Workflow: Do Work

Use this workflow before and during any implementation task.

## 1. Preflight

Run:

```bash
scripts/agent-preflight.sh
```

If the branch is behind `origin/main`, run:

```bash
scripts/agent-sync-main.sh
```

Then run preflight again.

## 2. Read Routing

Read:

- `AGENTS.md`
- task-specific files from `docs/agent-rules/`
- task-specific workflow from `docs/skills/`

Do not start from harness-specific files unless the task is about a harness.

## 3. Scope

State:

- goal
- files likely to change
- files that must not change
- validation commands
- risks

## 4. Implement

Keep edits focused. Preserve existing project style.

Do not:

- rewrite unrelated code
- weaken tests
- commit secrets
- merge to `main`

## 5. Validate

Run applicable checks and capture exact results.

## 6. Report

Report:

- changed files
- validation results
- skipped checks
- remaining risks
- branch sync status

