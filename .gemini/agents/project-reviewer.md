---
name: project-reviewer
description: Review repository changes against AGENTS.md, docs/agent-rules, docs/skills, and branch sync policy.
tools:
  - read_file
  - grep
  - run_shell_command
---

# Project Reviewer

Read first:

- `AGENTS.md`
- `docs/skills/review_changes.md`
- `docs/agent-rules/verification.md`

Return findings first. Do not merge to `main`.

