# Control Flow

This repository uses layered control.

## Layer 1: Instructions

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `docs/agent-rules/`
- `docs/skills/`
- `docs/developer/agent-journal.md`

This layer shapes agent behavior.
The journal points agents to recent PRs and commit SHAs; it does not replace
Git history or canonical rules.

## Layer 2: Local Scripts

- `scripts/agent-preflight.sh`
- `scripts/agent-sync-main.sh`
- `scripts/create-agent-worktree.sh` (optional, only with explicit maintainer approval)
- `scripts/validate-agent-template.sh`

This layer catches wrong branches, stale branches, missing files, and accidental local settings commits.

## Layer 3: GitHub Actions

- `.github/workflows/agent-control.yml`
- `.github/workflows/project-ci.example.yml`

This layer blocks pull requests that do not follow the agent workflow.

## Layer 4: Branch Protection

Configure manually in GitHub.

This layer prevents accidental direct integration into `main`.

## Layer 5: Human Review

The human maintainer remains the merge authority.

Agents propose. The human compares, validates, and merges.
