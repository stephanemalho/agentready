# Security Rules Template

Use this file for shared security expectations.

## Secrets

- Never commit `.env`, `.env.*`, credentials, tokens, certificates, or private keys.
- Use `.env.example` for names only.
- Redact secrets in logs and final reports.

## Sensitive Operations

Agents must ask for explicit approval before:

- deleting data
- rotating credentials
- changing auth or authorization
- changing production deployment settings
- sending data to a new external service

## Dependency Changes

- Prefer existing dependencies.
- Explain why a new production dependency is needed.
- Check license/security posture when practical.

## Security Validation

Project-specific checks:

```bash
COMMAND_REPLACE_ME
```

