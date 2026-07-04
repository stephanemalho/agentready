---
topic: claude-code
last_reviewed: 2026-07-04
source_policy: official-docs-first
staleness_limit_days: 30
---

# Claude Code Harness Guide

## Purpose

Claude Code uses `CLAUDE.md` and `.claude/` files to load project context, settings, rules, agents, hooks, and local preferences.

## Source Of Truth

- Canonical: `AGENTS.md`
- Claude adapter: `CLAUDE.md`
- Rules: `docs/agent-rules/`
- Workflows: `docs/skills/`

## What Belongs In `.claude/`

- Claude-specific agents
- Claude-specific path rules
- Safe project settings
- Local settings examples

## What Must Stay Neutral

- Business rules
- Architecture rules
- Test policy
- Data model rules
- Security policy

## Official Docs

- https://code.claude.com/docs/en/memory
- https://code.claude.com/docs/en/settings
- https://code.claude.com/docs/en/hooks
- https://code.claude.com/docs/en/skills
- https://code.claude.com/docs/en/sub-agents
- https://code.claude.com/docs/en/common-workflows

## Warning

Do not duplicate project rules here. `CLAUDE.md` imports `AGENTS.md`; keep it thin.

