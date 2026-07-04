# AgentReady Security Rules

## Secrets

- Never commit `.env`, `.env.*`, credentials, tokens, certificates, or private keys.
- Use `.env.example` for names only.
- Redact secrets in logs and final reports.
- V1 reports must not print environment variable values or secret-like file contents.

## Sensitive Operations

Agents must ask for explicit approval before:

- deleting data
- rotating credentials
- changing auth or authorization
- changing production deployment settings
- sending data to a new external service
- adding an AI SDK or any network-backed analysis provider
- publishing a crate or GitHub release

## Dependency Changes

- Prefer existing dependencies.
- Explain why a new production dependency is needed.
- Check license/security posture when practical.
- Avoid dependencies that execute external commands during scans.

## Security Validation

Project-specific checks:

```bash
scripts/ci-secret-scan.sh
cargo clippy -- -D warnings
cargo test
```
