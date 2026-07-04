# AgentReady Operations Rules

## Environments

| Environment | Purpose | Owner |
|---|---|---|
| local | development | developer |
| CI | pull request validation | GitHub Actions |
| release | future binary/crate publishing | Stephane Malho |

## Deployment

- Deployment platform: none in V1.
- Build command: `cargo build --release`.
- Release process: manual until a release workflow is explicitly designed.
- Rollback process: publish a corrective release or revert the relevant Git tag once release automation exists.

## Observability

- Logs: plain stderr/stdout only.
- Metrics: none in V1.
- Tracing: none in V1.
- Error reporting: command exits non-zero with a clear error message.

## Local Ports

AgentReady is a CLI and does not bind local ports in V1.

| Service | Default | Agent override |
|---|---:|---:|
| not applicable | `n/a` | `n/a` |
