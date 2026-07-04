# RepoLens Project Profile

## Identity

- Project name: RepoLens
- Repository: `git@github.com:stephanemalho/repolens.git`
- Primary maintainer: Stephane Malho
- Product/domain: developer tooling, repository analysis, coding-agent harness readiness
- Audience/users: software developers, technical project maintainers, and teams preparing repositories for Codex, Claude Code, Gemini CLI, or similar harnesses
- Critical user workflows:
  - scan a local repository and print a Markdown report
  - scan a local repository and print machine-readable JSON
  - run a quick repository health check with `doctor`
  - verify whether a repository is ready for multi-harness coding-agent work
  - later: paste a GitHub repository URL into a SaaS UI and view readiness score history

## Stack

- Language(s): Rust 2024
- Framework(s): none
- Runtime: native CLI binary
- Package manager: Cargo
- Database: none
- External services: none in V1
- Deployment target: local binary first; GitHub Releases and SaaS backend/frontend in later roadmap phases

## Commands

```bash
# Install
cargo build

# Development server
not applicable

# Lint
cargo fmt --check
cargo clippy -- -D warnings

# Test
cargo test

# Build
cargo build --release
```

## Sensitive Areas

- Files or folders agents must not modify without explicit approval: `.github/branch-protection.md`, release configuration, publishing credentials, any future installer scripts that modify user machines
- Data that must never be logged: environment variable values, tokens, private keys, certificate contents, full secret-like strings found in scanned repositories
- Commands that require human approval: publishing crates, creating GitHub releases, deleting branches, changing remotes, adding network-backed AI providers
- External services that can incur cost: none in V1

## Roadmap

- Product roadmap: `docs/ROADMAP.md`
- Agents must consult the roadmap before changing product scope, SaaS architecture, GitHub integration, report schemas, or readiness rules.

## Definition Of Done

- Code updated: feature implemented in the correct Rust module with a thin CLI entrypoint
- Tests: relevant unit tests and/or integration CLI tests added or updated
- Documentation: README and agent rules updated when behavior changes
- Migration/deployment: not applicable for V1
- Human review: required before merge to `main`
