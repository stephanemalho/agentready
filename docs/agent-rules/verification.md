# Verification Rules

Every agent report must distinguish between:

- checks that passed
- checks that failed
- checks that were skipped
- checks that were not available

## Minimum Generic Checks

```bash
scripts/validate-agent-template.sh
scripts/agent-preflight.sh
git status --short
```

## Project Checks

Fill these in `AGENTS.md` and project-specific rule files:

```bash
PROJECT_LINT_COMMAND_REPLACE_ME
PROJECT_TEST_COMMAND_REPLACE_ME
PROJECT_BUILD_COMMAND_REPLACE_ME
```

## Report Format

Use concise, concrete reporting:

```txt
Changed:
- file A: reason
- file B: reason

Validation:
- passed: command
- failed: command and summary
- skipped: command and reason

Branch:
- current branch
- sync status with origin/main
```

