# RepoLens Data Model Rules

## Data Ownership

| Entity | Owner | Source of truth |
|---|---|---|
| `RepoAnalysis` | RepoLens analyzer | Local repository files |
| `DetectedStack` | RepoLens detectors | File marker evidence |
| `HealthChecks` | RepoLens analyzer | Local repository files |
| Markdown report | RepoLens report renderer | `RepoAnalysis` |
| JSON report | RepoLens report renderer | `RepoAnalysis` |

## Migration Rules

- RepoLens V1 has no database and no migrations.
- If persistent configuration is added later, document the file format and backward compatibility rules before implementation.

## API/Data Contracts

- Contract files: Rust serializable structs used for JSON output.
- Generated types: none.
- Compatibility policy: avoid unnecessary JSON field renames once released; additive fields are preferred.

## Test Data

- Fixtures location: temporary directories in tests; reusable fixtures may live under `tests/fixtures/`.
- Seed command: not applicable.
- Data that must not be used in tests: real secrets, private customer repositories, private keys, tokens, or certificate material.
