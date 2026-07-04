# Workflow: Implement Feature Or Fix

## Required Reading

- `AGENTS.md`
- `docs/skills/do_work.md`
- `docs/agent-rules/architecture.template.md`
- `docs/agent-rules/testing.template.md`
- task-specific project rule files

## Implementation Rules

- Prefer the smallest coherent change.
- Keep public contracts stable unless the task requires changing them.
- Add or update tests when behavior changes.
- Update docs when the user-facing or agent-facing behavior changes.
- Preserve existing naming, formatting, and architecture patterns.

## Validation

Run the narrowest useful checks first, then broader checks when risk justifies it.

Examples:

```bash
PROJECT_TARGETED_TEST_COMMAND_REPLACE_ME
PROJECT_TEST_COMMAND_REPLACE_ME
PROJECT_BUILD_COMMAND_REPLACE_ME
```

## Final Report

Include:

- what changed
- why
- tests/checks
- sync status with `origin/main`
- risks or follow-up work

