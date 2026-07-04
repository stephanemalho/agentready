# Setup Guide

## New Repository

1. Create a repository from this template.
2. In repository settings, enable template mode if this repository itself should be reused.
3. Configure branch protection for `main` using `.github/branch-protection.md`.
4. Fill project facts:

```txt
docs/templates/project-profile.template.md
docs/agent-rules/*.template.md
AGENTS.md
```

5. Replace `.github/workflows/project-ci.example.yml` with real project CI.
6. Keep `Agent control gates` required for PRs into `main`.

## Existing Repository

Copy the template files into the project root.

Then run:

```bash
chmod +x scripts/*.sh
scripts/validate-agent-template.sh
```

Fill placeholders before asking agents to implement production work.

## Creating Agent Worktrees

Each worktree checks out the harness's assigned branch. Agents never create branches.

```bash
scripts/create-agent-worktree.sh codex ../repolens-codex
scripts/create-agent-worktree.sh claude ../repolens-claude
scripts/create-agent-worktree.sh gemini ../repolens-gemini
```

Each worktree should have its own local env:

```bash
cp .env.example .env.local
```

## Agent Start Checklist

Every agent must start with:

```bash
scripts/agent-preflight.sh
```

If behind `origin/main`:

```bash
scripts/agent-sync-main.sh
```

Then:

```bash
scripts/agent-preflight.sh
```

## Human Merge Checklist

Before merging an agent PR:

1. Confirm branch pattern.
2. Confirm CI passed.
3. Compare the branch with `main`.
4. Review changed files.
5. Run local checks when needed.
6. Merge only when satisfied.

Agents do not merge to `main`.

