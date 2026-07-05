# Agent Handoff Rules

This repository uses Git as the source of truth for completed work. The shared
handoff journal is an index into Git history, not a replacement for it.

## Shared Journal

The shared journal lives at:

```txt
docs/developer/agent-journal.md
```

Every agent must read the latest relevant entries after preflight and before
planning follow-up work. If an entry points at a commit or PR that affects the
task, inspect the referenced Git history with `git show` or `git log` instead
of relying only on the prose summary.

## Agent Identity

Every journal entry must identify who performed the work with exactly one of
these values:

```txt
codex
claude-code
gemini-cli
local-agent
```

Use `local-agent` for work done outside the assigned Codex, Claude Code, or
Gemini CLI harnesses, including local scripts or manual maintainer-guided
automation. Do not invent new identity labels in the journal.

## Entry Format

Each entry should include:

- date
- agent identity
- branch
- PR number and URL when available
- merge commit SHA when merged
- relevant commit SHAs and headlines
- exact Git commands a future agent can run to inspect the work
- short notes about validation, CI, or follow-up risk

Keep entries short. Prefer commit SHAs and commands over duplicated diff
descriptions.

## Update Policy

Update the journal when:

- a PR is merged
- an agent leaves substantial work for another harness to continue
- a CI or local-development failure required non-obvious diagnosis
- the next safe step depends on knowing recent history

Do not store secrets, tokens, private transcripts, or local-only machine state in
the journal. Do not put canonical business, architecture, data-model, security,
or testing rules only in the journal; those belong under `docs/agent-rules/`.
