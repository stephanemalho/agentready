# AGENTS.md

This file is the canonical entrypoint for all AI coding agents working in this repository.

Keep it short, current, and practical. Put detailed project knowledge in `docs/agent-rules/`, `docs/skills/`, and `docs/templates/`.

## Project Identity

- Project name: `PROJECT_NAME_REPLACE_ME`
- Project type: `PROJECT_TYPE_REPLACE_ME`
- Primary stack: `STACK_REPLACE_ME`
- Runtime/package manager: `RUNTIME_REPLACE_ME`
- Main branch: `main`
- Human maintainer: `OWNER_REPLACE_ME`

## Source Of Truth

Use this order when gathering project instructions:

1. `AGENTS.md`
2. `docs/agent-rules/README.md`
3. The task-specific files in `docs/agent-rules/`
4. The task-specific workflow in `docs/skills/`
5. Harness-specific adapter docs only when working on that harness

Do not put canonical business rules only in `.codex/`, `.claude/`, `.gemini/`, or another harness folder.

## Agent Operating Modes

### Explore

Read-only. Understand architecture, dependencies, existing decisions, and risks.

### Plan

Read-only. Produce an implementation plan, touched files, validation commands, and risks.

### Implement

Edit only inside the approved scope.

### Review

Read-only. Check correctness, tests, architecture, security, and rule compliance.

### Validate

Run deterministic checks and report exact command results.

## Parallel Agent Workflow

- Never run two coding agents in the same worktree.
- Each model/harness must work in its own git worktree and branch.
- Branch format: `agent/<harness>/<ticket-or-task>/<short-slug>`.
- Harness names: `codex`, `claude`, `gemini`, or another lowercase tool id.
- Each worktree must use separate local-only files when applicable: `.env.local`, database name, log folder, temp folder, cache folder, and ports.
- Agents may propose changes, but final merge requires human review.
- Agents must not merge or push directly to `main`.

## Required Main Sync

Before starting implementation, every agent must run:

```bash
scripts/agent-preflight.sh
```

If the branch is behind `origin/main`, the agent must run:

```bash
scripts/agent-sync-main.sh
```

The agent must repeat the preflight before opening a pull request or reporting completion.

## Task Routing

| Task type | Required reading |
|---|---|
| New feature or behavior change | `docs/skills/implement_feature.md`, `docs/agent-rules/architecture.template.md`, `docs/agent-rules/testing.template.md` |
| Bug fix | `docs/skills/implement_feature.md`, `docs/agent-rules/testing.template.md`, `docs/agent-rules/verification.md` |
| Business rule change | `docs/agent-rules/business-rules.template.md`, `docs/templates/project-profile.template.md` |
| Data model or schema change | `docs/agent-rules/data-model.template.md`, `docs/skills/implement_feature.md` |
| Security or secret handling | `docs/agent-rules/security.template.md`, `docs/agent-rules/verification.md` |
| CI or workflow change | `docs/agent-rules/git-workflow.md`, `.github/workflows/`, `scripts/` |
| Harness configuration change | `docs/harness/<harness>.md`, official harness docs |
| Rule or documentation update | `docs/skills/update_agent_rules.md`, `docs/agent-rules/README.md` |
| Completion/reporting | `docs/skills/do_work.md` |

## Validation Commands

Fill these commands for each project.

```bash
# Required generic checks
scripts/validate-agent-template.sh
scripts/agent-preflight.sh

# Project-specific checks
PROJECT_LINT_COMMAND_REPLACE_ME
PROJECT_TEST_COMMAND_REPLACE_ME
PROJECT_BUILD_COMMAND_REPLACE_ME
```

If a command is unavailable, report why. Do not claim validation passed without exact command output.

## Harness Adapters

- Codex: `.codex/config.toml` and `.agents/skills/`
- Claude Code: `CLAUDE.md`, `.claude/settings.json`, `.claude/rules/`, `.claude/agents/`
- Gemini CLI: `GEMINI.md`, `.gemini/settings.json`, `.gemini/agents/`, `.gemini/commands/`

Adapters should reference neutral rules instead of duplicating them.

## Official Docs Policy

Before editing harness configuration, check the official docs:

- Codex: https://developers.openai.com/codex/guides/agents-md
- Codex config: https://developers.openai.com/codex/config-basic
- Codex config reference: https://developers.openai.com/codex/config-reference
- Codex skills: https://developers.openai.com/codex/skills
- Codex subagents: https://developers.openai.com/codex/subagents
- Claude Code memory: https://code.claude.com/docs/en/memory
- Claude Code settings: https://code.claude.com/docs/en/settings
- Claude Code hooks: https://code.claude.com/docs/en/hooks
- Claude Code skills: https://code.claude.com/docs/en/skills
- Claude Code subagents: https://code.claude.com/docs/en/sub-agents
- Gemini CLI docs: https://google-gemini.github.io/gemini-cli/docs/
- Gemini CLI context: https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html
- Gemini CLI worktrees: https://geminicli.com/docs/cli/git-worktrees/

## Done Criteria

Before reporting completion:

1. Confirm the branch matches `agent/<harness>/<ticket-or-task>/<short-slug>`.
2. Confirm the branch is up to date with `origin/main`.
3. Run applicable validation commands.
4. Summarize changed files.
5. Report assumptions and any skipped validation.
6. Do not merge to `main`.

