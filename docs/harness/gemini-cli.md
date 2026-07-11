---
topic: gemini-cli
last_reviewed: 2026-07-11
source_policy: official-docs-first
staleness_limit_days: 30
---

# Gemini CLI Harness Guide

## Purpose

Gemini CLI can load `GEMINI.md` context files and can be configured to read `AGENTS.md` directly.

## Source Of Truth

- Canonical: `AGENTS.md`
- Gemini adapter: `GEMINI.md`
- Rules: `docs/agent-rules/`
- Workflows: `docs/skills/`

## What Belongs In `.gemini/`

- Gemini CLI settings
- Gemini subagents
- Gemini custom commands
- Optional Gemini policy/hook configuration

## Native Agents

Gemini CLI may use its native subagents on the current Gemini branch. Follow the shared ownership rules in `docs/harness/multi-agent-workflow.md`. Worktree isolation remains an optional Gemini capability, not the default project workflow.

## What Must Stay Neutral

- Business rules
- Architecture rules
- Test policy
- Data model rules
- Security policy

## Official Docs

- https://google-gemini.github.io/gemini-cli/docs/
- https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html
- https://geminicli.com/docs/cli/git-worktrees/
- https://geminicli.com/docs/core/subagents/
- https://geminicli.com/docs/hooks/

## Warning

Do not duplicate project rules here. Reference neutral docs instead.
