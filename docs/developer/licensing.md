# AgentReady Licensing

## License

AgentReady is dual-licensed under **MIT OR Apache-2.0**, the Rust ecosystem convention (used by rustc, serde, tokio). Users pick whichever license suits them.

- `LICENSE-MIT` — MIT text, copyright (c) 2026 Stéphane Malho.
- `LICENSE-APACHE` — official Apache-2.0 text (unmodified, from apache.org).
- `Cargo.toml` declares `license = "MIT OR Apache-2.0"` (required by crates.io).
- The README carries the standard dual-license section and the contribution clause: any contribution submitted for inclusion is dual-licensed the same way, without additional terms.

## Why Dual Instead Of MIT Alone

- Apache-2.0 section 3 includes an explicit patent grant: a contributor cannot later sue over patents covering their own contribution.
- Matching the ecosystem convention removes legal friction when other Rust projects depend on the crate.
- MIT remains available to users, so nothing is lost versus MIT-only.

## Implications To Keep In Mind

- Both licenses are permissive: anyone may use AgentReady commercially, including hosting a competing service on the same engine. This is the accepted open-core trade-off; the SaaS value lives in hosting, the GitHub App, and team features (Phases 4-6).
- Changing the license later requires the agreement of every contributor. If a license change is ever considered, do it before accepting external contributions.
- The license health check in `src/analyzer/` recognizes `LICENSE`, `LICENSE.md`, `COPYING`, `LICENSE-MIT`, and `LICENSE-APACHE`.
