# Publishing AgentReady To crates.io

## Why

crates.io is the official Rust package registry. Publishing there lets anyone install the CLI with:

```bash
cargo install agentready
```

This is a required exit criterion of roadmap Phase 3: it is the natural free distribution channel for the CLI and builds product credibility before the SaaS launch (Phase 4).

## API Token

- Created on crates.io (Account Settings -> API Tokens) on 2026-07-04.
- **Expires on 2027-07-04 (365 days validity). Renew it before that date** at https://crates.io/settings/tokens, or `cargo publish` will start failing with an authentication error.
- Scopes: `publish-new` and `publish-update` only (least privilege; no change-owners, no yank).
- Crate pattern: restrict the token to `agentready`.
- `cargo login` stores the token locally in `~/.cargo/credentials.toml`. Never commit or paste the token anywhere.
- A verified email address on the crates.io profile is required to publish.

## Publish Procedure

```bash
# 1. Working tree must be clean (publish from a commit)
git status

# 2. Bump the version in Cargo.toml (published versions are immutable)
# 3. Verify the package builds and uploads correctly
cargo publish --dry-run

# 4. Publish
cargo publish
```

Notes:

- A published version can never be overwritten, only yanked (`cargo yank`), so bump `version` in `Cargo.toml` for every release.
- `Cargo.toml` must declare `description`, `license`, and `repository` for the upload to be accepted.

## Naming History

The original crate name `repolens` is owned by an unrelated active project (`systm-d/repolens`, same product domain). The product was renamed **AgentReady** (crate `agentready`) on 2026-07-04 before any public visibility.
