# Workflow: Review Changes

Use this workflow for code review, PR review, or self-review.

## Review Priorities

1. Correctness and behavioral regressions
2. Security and secret handling
3. Data/model compatibility
4. Test coverage and validation gaps
5. Architecture and maintainability
6. Documentation and agent-rule consistency

## Commands

```bash
git fetch origin main
git diff --stat origin/main...HEAD
git diff --name-only origin/main...HEAD
```

Then inspect relevant files.

## Output

Lead with findings.

For each finding:

- severity
- file/line when possible
- issue
- impact
- recommended fix

If there are no findings, say so and mention residual risk or test gaps.

