# Testing Rules Template

Use this file for testing expectations shared by every harness.

## Test Commands

```bash
# Unit
COMMAND_REPLACE_ME

# Integration
COMMAND_REPLACE_ME

# E2E
COMMAND_REPLACE_ME
```

## Test Policy

- Do not delete or weaken tests to make a task pass.
- Do not skip failing tests unless the user explicitly approves and the reason is documented.
- New behavior should include focused test coverage.
- Bug fixes should include a regression test when practical.

## Fixtures

- Fixture location:
- Mocking policy:
- Network policy:
- Database policy:

## Reporting

When reporting completion, include:

- exact command
- pass/fail status
- important failures
- skipped checks and why

