# RepoLens Business Rules

## Domain Vocabulary

| Term | Meaning | Source |
|---|---|---|
| Repository | A local directory being analyzed by RepoLens | Product definition |
| Detector | Pure logic that recognizes project technology or conventions from files | Architecture rules |
| Evidence | File paths that justify a detected stack or health signal | Product definition |
| Health check | A lightweight boolean signal about repository readiness | Product definition |
| Report | Markdown or JSON output generated from local repository files | Product definition |
| Agent context | Concise, safe information that an AI coding agent can read before editing | Product direction |

## Invariants

- V1 analysis must be deterministic from local files.
- V1 scans must not call external AI services.
- Reports must show evidence for detected stacks.
- RepoLens must respect `.gitignore` when walking repositories.
- RepoLens must not print secret values discovered in files.
- JSON output must remain machine-readable and generated from serializable analysis structs.

## Workflows

### Scan Repository

1. Trigger: user runs `repolens scan <path>`.
2. Preconditions: `<path>` exists and is a directory.
3. Steps: walk files, detect stacks, compute health checks, render Markdown or JSON.
4. Success state: report is printed to stdout or written to the requested output file.
5. Failure handling: return a non-zero exit code with a clear error message.

### Doctor Repository

1. Trigger: user runs `repolens doctor <path>`.
2. Preconditions: `<path>` exists and is a directory.
3. Steps: run the same analysis as `scan`, then render compact health lines.
4. Success state: terminal output lists `ok` or `warn` for each health check.
5. Failure handling: return a non-zero exit code with a clear error message.

## Copy And UX Rules

- Preferred wording: short English labels, concrete file evidence, plain terminal output.
- Forbidden wording: claims that RepoLens performed AI reasoning in V1.
- Locale/language: public repo copy should be English.
- Accessibility expectations: CLI output should work in plain terminals and CI logs.

## Approval Rules

- Changes requiring human approval: adding network calls, adding an AI SDK, publishing packages, changing branch protection, changing GitHub release policy.
- Changes agents may make directly: detectors, report formatting, tests, documentation, and internal refactors that preserve CLI behavior.
