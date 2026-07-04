# RepoLens Agent Rules

This file is the canonical entrypoint for all AI coding agents working in this repository.

Keep this file short, current, and practical. Put detailed project knowledge in `docs/agent-rules/`, `docs/skills/`, and `docs/templates/`.

## Project Identity

- Project name: `repolens`
- Project type: Rust command-line application
- Primary stack: Rust 2024, Cargo, clap, ignore, serde
- Runtime/package manager: Rust toolchain with Cargo
- Repository: `git@github.com:stephanemalho/repolens.git`
- Main branch: `main`
- Human maintainer: Stephane Malho

## Product Direction

RepoLens is an offline repository analysis CLI. It scans local repositories, reports project health, and checks whether a repo is ready for coding-agent harnesses such as Codex, Claude Code, and Gemini CLI.

Product roadmap: `docs/ROADMAP.md`.

Agents must read `docs/ROADMAP.md` before planning or implementing product, CLI, report, SaaS, GitHub, or roadmap-related work.

V1 must stay deterministic and local-first:

- no external AI SDK
- no network calls during scans
- no API keys required
- no secret values printed in output
- Markdown and JSON reports generated from local files only
- harness readiness checks validate files and configuration, not model behavior

## Source Of Truth

Use this order when gathering project instructions:

1. `AGENTS.md`
2. `docs/ROADMAP.md` for product direction, milestones, SaaS plans, and future GitHub integration
3. `docs/agent-rules/README.md`
4. The task-specific files in `docs/agent-rules/`
5. The task-specific workflow in `docs/skills/`
6. Harness-specific adapter docs only when working on that harness

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
| New feature or behavior change | `docs/ROADMAP.md`, `docs/skills/implement_feature.md`, `docs/agent-rules/architecture.template.md`, `docs/agent-rules/testing.template.md` |
| Bug fix | `docs/skills/implement_feature.md`, `docs/agent-rules/testing.template.md`, `docs/agent-rules/verification.md` |
| Business rule change | `docs/agent-rules/business-rules.template.md`, `docs/templates/project-profile.template.md` |
| Data model or schema change | `docs/agent-rules/data-model.template.md`, `docs/skills/implement_feature.md` |
| SaaS, GitHub API, or hosted scan work | `docs/ROADMAP.md`, `docs/agent-rules/architecture.template.md`, `docs/agent-rules/security.template.md` |
| Security or secret handling | `docs/agent-rules/security.template.md`, `docs/agent-rules/verification.md` |
| CI or workflow change | `docs/agent-rules/git-workflow.md`, `.github/workflows/`, `scripts/` |
| Harness configuration change | `docs/harness/<harness>.md`, official harness docs |
| Roadmap update | `docs/ROADMAP.md`, `docs/skills/update_agent_rules.md` |
| Rule or documentation update | `docs/skills/update_agent_rules.md`, `docs/agent-rules/README.md` |
| Completion/reporting | `docs/skills/do_work.md` |

## Validation Commands

```bash
# Required generic checks
scripts/validate-agent-template.sh
scripts/agent-preflight.sh

# Project-specific checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

If a command is unavailable, report why. Do not claim validation passed without exact command output.

## Rust Implementation Rules

- Keep `src/main.rs` as a thin entrypoint.
- Put CLI parsing in `src/cli.rs`.
- Put repository analysis in `src/analyzer/`.
- Put stack detection in `src/detectors/`.
- Put coding-agent harness readiness checks in `src/harness/`.
- Put output formatting in `src/report/`.
- Prefer small pure functions and unit tests for detectors/renderers.
- Use integration tests for command behavior in `tests/`.
- Do not add a dependency when the standard library is enough.
- Explain every new runtime dependency in the PR summary.

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
