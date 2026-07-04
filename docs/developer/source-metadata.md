# Repository Source Metadata

AgentReady includes source metadata in `scan --format json` and
`harness --format json` reports so the future SaaS can store scan provenance
without duplicating repository acquisition logic outside the CLI core.

## Contract

Local scans expose only the provider:

```json
{
  "source": {
    "provider": "local"
  }
}
```

GitHub scans expose the repository identity and default branch:

```json
{
  "source": {
    "provider": "github",
    "owner": "stephanemalho",
    "repo": "agentready",
    "default_branch": "main",
    "commit_sha": "..."
  }
}
```

`commit_sha` is included when GitHub provides a branch head commit SHA. If that
metadata cannot be read, AgentReady omits the field and still completes the scan
from the repository tree and selected file contents.

## Boundaries

- Local scans remain offline and never call the network.
- GitHub metadata is fetched only for explicit `github:owner/repo` or GitHub URL
  targets.
- `root` remains in reports for backward compatibility; `source` is the
  structured provenance contract for future API and SaaS consumers.
