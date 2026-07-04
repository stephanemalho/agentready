# RepoLens Testing Rules

## Test Commands

```bash
# Unit
cargo test --lib

# Integration
cargo test --test cli

# Full local validation
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Test Policy

- Do not delete or weaken tests to make a task pass.
- Do not skip failing tests unless the user explicitly approves and the reason is documented.
- New behavior should include focused test coverage.
- Bug fixes should include a regression test when practical.

## Fixtures

- Fixture location: inline temp directories for simple CLI tests; `tests/fixtures/` may be added when fixtures become reusable.
- Mocking policy: prefer real temporary directories and real files over mocks.
- Network policy: tests must not require network access.
- Database policy: no database in V1.

## Reporting

When reporting completion, include:

- exact command
- pass/fail status
- important failures
- skipped checks and why
