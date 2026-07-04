# Agent Rules

This directory is the canonical location for project rules shared by every harness.

Harness folders may reference these files, but must not become the only place where a project-wide rule exists.

The product roadmap lives in `docs/ROADMAP.md`. Read it before product, CLI, SaaS, GitHub, report, or milestone work.

## How to fill this directory

Start with the template files:

- `project-profile.template.md`
- `architecture.template.md`
- `business-rules.template.md`
- `data-model.template.md`
- `testing.template.md`
- `security.template.md`
- `operations.template.md`

For a real project, either rename the `.template.md` files or keep them and add project-specific files beside them.

## Rule quality bar

Good rules are:

- concrete enough to verify
- scoped to a project, directory, or task type
- short enough to stay in context
- linked to source documents or decisions when possible

Avoid:

- vague preferences
- rules that duplicate another file
- personal machine settings
- secrets or credentials
- stale commands that no longer run

## Maintenance

When an agent makes the same mistake twice, add or update a rule here.

When a rule only applies to one harness because of tool behavior, document it in `docs/harness/` and keep project logic here.

When roadmap scope, phase status, or product direction changes, update `docs/ROADMAP.md` in the same PR.
