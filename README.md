# Multi-Harness Agent Template

Reusable starter structure for technical projects that use several coding-agent harnesses against one canonical source of truth.

The template is designed for Codex, Claude Code, Gemini CLI, and future harnesses. It keeps project knowledge neutral in `AGENTS.md` and `docs/`, while each harness has a thin adapter in its own configuration folder.

## Core idea

- `AGENTS.md` is the canonical entrypoint.
- `docs/agent-rules/` contains project rules that should apply to every harness.
- `docs/skills/` contains neutral workflows.
- `.codex/`, `.claude/`, `.gemini/`, and `.agents/skills/` adapt the neutral rules to each tool.
- Each agent works on its own branch and worktree.
- Pull requests are reviewed and merged by a human.

## Branch model

Use `main` as the protected integration branch.

Agent branches must use this pattern:

```txt
agent/<harness>/<ticket-or-task>/<short-slug>
```

Examples:

```txt
agent/codex/42/add-auth-tests
agent/claude/refactor/routing-cleanup
agent/gemini/docs/update-api-guide
```

Every agent must synchronize with `origin/main` before starting work and before opening a pull request.

## Quick start for a new project

1. Create a new repository from this template.
2. Fill the project facts in `docs/templates/project-profile.template.md`.
3. Replace placeholder rules in `docs/agent-rules/*.template.md`.
4. Update `AGENTS.md` with the project name, stack, commands, and critical constraints.
5. Keep the harness adapters thin. Do not duplicate business rules in `.codex/`, `.claude/`, or `.gemini/`.
6. Enable branch protection on `main`.
7. Require the `Agent control gates` workflow before merge.

## Quick start for an existing project

Copy these files and folders into the project root:

```txt
AGENTS.md
CLAUDE.md
GEMINI.md
.codex/
.claude/
.gemini/
.agents/
docs/agent-rules/
docs/skills/
docs/harness/
scripts/
.github/
```

Then adapt the placeholders to the real project.

## Human merge policy

Agents may propose changes. They must not merge to `main`.

The human maintainer compares the PR branch with `main`, reviews the diff, validates the output, and performs the final merge/push.

## Official references

- OpenAI Codex AGENTS.md: https://developers.openai.com/codex/guides/agents-md
- OpenAI Codex config: https://developers.openai.com/codex/config-basic
- OpenAI Codex skills: https://developers.openai.com/codex/skills
- Anthropic Claude Code memory: https://code.claude.com/docs/en/memory
- Anthropic Claude Code settings: https://code.claude.com/docs/en/settings
- Anthropic Claude Code hooks: https://code.claude.com/docs/en/hooks
- Gemini CLI context: https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html
- Gemini CLI worktrees: https://geminicli.com/docs/cli/git-worktrees/

