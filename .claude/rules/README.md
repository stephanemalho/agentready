# Claude Rules Adapter

Canonical project rules live in:

```txt
docs/agent-rules/
```

This folder is only for Claude-specific loading, scoping, or rule behavior.

No business, architecture, test, security, data-model, or workflow rule should exist only in `.claude/rules/`.

If Claude needs a path-scoped rule, make it reference the neutral rule file instead of duplicating the content.

