---
topic: codex
last_reviewed: 2026-07-11
source_policy: official-docs-first
staleness_limit_days: 30
---

# Codex Harness Guide

## Purpose

Codex uses `AGENTS.md` as the canonical entrypoint and `.codex/config.toml` as a thin project adapter.

## Source Of Truth

- Canonical: `AGENTS.md`
- Rules: `docs/agent-rules/`
- Workflows: `docs/skills/`
- Codex skill wrappers: `.agents/skills/`

## What Belongs In `.codex/`

- Codex client settings that are safe to commit
- Pointers to canonical docs
- Project-specific instruction routing
- Optional hooks/rules when they enforce Codex behavior

## Native Agents

Codex may coordinate native subagents on the current Codex branch. Follow the shared ownership rules in `docs/harness/multi-agent-workflow.md`; do not create worktrees merely because work is delegated.

## What Must Stay Neutral

- Business rules
- Architecture rules
- Test policy
- Data model rules
- Security policy

## Official Docs

- https://developers.openai.com/codex/guides/agents-md
- https://developers.openai.com/codex/config-basic
- https://developers.openai.com/codex/config-advanced
- https://developers.openai.com/codex/config-reference
- https://developers.openai.com/codex/skills
- https://developers.openai.com/codex/subagents

## Warning

Do not duplicate project rules here. Reference neutral docs instead.
