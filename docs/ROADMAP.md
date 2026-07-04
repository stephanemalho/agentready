# RepoLens Roadmap

This roadmap is the product source of truth for planned RepoLens evolution.

Agents must read this file before planning or implementing product, CLI, report, SaaS, GitHub, or roadmap-related work. Keep changes incremental, offline-safe by default, and aligned with the current Rust CLI architecture.

## Product North Star

RepoLens helps teams verify whether their repositories are ready for reliable coding-agent workflows across Codex, Claude Code, Gemini CLI, and future harnesses.

The CLI remains the open-source core. Future SaaS work should reuse the same analysis engine instead of creating separate business logic.

## Non-Negotiable Principles

- RepoLens must not run, call, or embed AI models.
- Local CLI scans must remain deterministic, offline, and safe by default.
- SaaS scans must request the minimum GitHub permissions needed to read repository files.
- The same readiness rules must power CLI and SaaS reports.
- Reports must provide evidence paths for every check.
- Missing files or weak configuration should appear as findings, not runtime crashes.
- Secrets, token values, private keys, and full sensitive file contents must never be printed.
- New external dependencies must be justified in PR summaries.

## Phase 1: CLI Harness Readiness Core

Status: done.

Goal: make the local CLI a reliable harness-readiness checker for repositories.

Required capabilities:

- `repolens harness <path>` produces Markdown readiness reports.
- `repolens harness <path> --format json` produces machine-readable reports.
- `--harness codex|claude|gemini` filters harness-specific checks.
- Shared checks validate `AGENTS.md`, `docs/agent-rules/`, `docs/skills/`, sync scripts, and CI branch policy.
- Codex checks validate `.codex/config.toml` and `.agents/skills/*/SKILL.md`.
- Claude checks validate `CLAUDE.md`, `.claude/settings.json`, `.claude/rules/`, and absence of committed local settings.
- Gemini checks validate `GEMINI.md`, `.gemini/settings.json`, `.gemini/agents/`, and `.gemini/commands/`.

Exit criteria:

- CLI report works against RepoLens itself.
- JSON output is stable enough for a web API.
- Unit and integration tests cover pass/fail/warn readiness behavior.
- README documents `scan`, `doctor`, and `harness`.

## Phase 2: Rule Catalog And Findings Model

Status: done.

Goal: turn readiness checks into a reusable rule catalog with stable finding IDs.

Required capabilities:

- Introduce stable rule IDs, for example `shared.agents_md.exists`, `codex.skills.present`, `gemini.settings.context_agents`.
- Add finding severity: `info`, `low`, `medium`, `high`.
- Add finding status: `pass`, `warn`, `fail`.
- Add remediation text for warnings and failures.
- Add official-doc source URLs per rule.
- Keep JSON schema additive and backward-compatible.

Exit criteria:

- Every readiness check has a stable ID.
- Reports show title, severity, status, evidence, source, and remediation.
- Existing CLI output remains readable.

## Phase 3: GitHub Repository Source

Status: planned.

Goal: allow RepoLens logic to analyze a GitHub repository without requiring a local checkout.

Required capabilities:

- Introduce a repository source abstraction, for example `RepositorySource`.
- Keep `LocalRepositorySource` for CLI filesystem scans.
- Add `GitHubRepositorySource` behind a new module or crate boundary.
- Fetch repository tree and selected file contents through GitHub APIs.
- Avoid downloading full repositories when only readiness files are needed.
- Handle missing files, rate limits, private repositories, and large repositories gracefully.

Suggested CLI shape:

```bash
repolens harness github:owner/repo
repolens harness https://github.com/owner/repo
```

Exit criteria:

- Public GitHub repos can be scanned from a URL.
- Local and GitHub sources produce the same report shape.
- Network access is explicit and never used by ordinary local scans.

## Phase 4: SaaS MVP For Public Repositories

Status: planned.

Goal: build a web UI where users paste a public GitHub repository URL and receive a readiness report.

Required capabilities:

- Web form accepts GitHub repository URL.
- Backend validates owner/repo input.
- Backend runs the RepoLens readiness engine against GitHub repository files.
- UI displays global score, per-harness score, findings, evidence, and recommendations.
- Scans are stored with timestamp and source commit SHA when available.
- Users can rescan manually.

Suggested data entities:

- `Repository`: owner, name, provider, default branch, visibility.
- `Scan`: repository id, commit SHA, score, created timestamp, status.
- `Finding`: scan id, rule id, harness, severity, status, title, evidence, remediation.

Exit criteria:

- Public repo scan works without user login.
- Report UI is useful without raw JSON.
- Scan history displays at least score over time.

## Phase 5: GitHub App And Private Repositories

Status: planned.

Goal: support private repositories and automatic scans with a GitHub App.

Required capabilities:

- GitHub App with minimum repository contents read permission.
- Installation flow for selected repositories.
- Installation tokens handled server-side only.
- Webhook support for push and pull request events.
- Automatic scan on default branch push.
- Optional scan on pull requests.
- Clear UI showing which repositories are installed and when they were last scanned.

Exit criteria:

- Private repo scan works through GitHub App installation.
- Tokens are never exposed to the browser.
- Webhook scans are deduplicated and rate-limited.

## Phase 6: Team And Governance Features

Status: planned.

Goal: help teams maintain readiness over time.

Required capabilities:

- Organization-level dashboard.
- Readiness trends by repository.
- Required-readiness thresholds.
- Export reports as Markdown or JSON.
- Suggested PR checklist generated from findings.
- Optional badge endpoint for repository README.

Exit criteria:

- Teams can see which repositories are ready, drifting, or blocked.
- Reports are useful for review and governance.

## Deferred Ideas

These are intentionally not part of the near-term roadmap:

- Running AI models to interpret repositories.
- Automatic code changes from the SaaS UI.
- Secret scanning beyond conservative metadata/path checks.
- Full dependency vulnerability scanning.
- General-purpose static analysis.
- CI provider support beyond GitHub Actions.

These can be revisited after the harness-readiness product is stable.

## Agent Usage Rules

When implementing roadmap work:

- Start from the earliest incomplete phase unless the user explicitly chooses another phase.
- Keep each PR scoped to one phase or one coherent capability.
- Update this roadmap when a phase changes status or scope.
- Update README and agent rules when user-facing commands or product positioning changes.
- Do not introduce SaaS-specific code into the CLI core without a clear boundary.
- Do not add network access to existing local commands.
