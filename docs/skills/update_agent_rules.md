# Workflow: Update Agent Rules

Use this workflow when a repeated mistake, review comment, or new project convention should become persistent guidance.

## Rule Placement

Put canonical project rules in:

```txt
docs/agent-rules/
```

Put neutral workflows in:

```txt
docs/skills/
```

Put harness-specific behavior in:

```txt
docs/harness/
.codex/
.claude/
.gemini/
```

## Do Not Duplicate

If a rule applies to all agents, do not copy it separately into each harness adapter.

Instead:

- add it once under `docs/agent-rules/`
- reference it from `AGENTS.md` or a relevant adapter

## Review Checklist

- Is the rule concrete?
- Is it still true?
- Does it belong to the project rather than a tool?
- Does it mention exact files, commands, or examples?
- Is there any conflicting rule?

