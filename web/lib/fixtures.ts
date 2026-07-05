import type { HarnessCheck } from "@/lib/contract";

// Canonical test fixture mirroring docs/client/data-contract.md.
// Reused by every test; extend it rather than duplicating payloads.

export function check(overrides: Partial<HarnessCheck> = {}): HarnessCheck {
  return {
    harness: "shared",
    id: "shared.agents_md.exists",
    severity: "high",
    status: "pass",
    title: "Canonical AGENTS.md",
    message: "Root AGENTS.md gives agents a predictable project entrypoint.",
    evidence: ["AGENTS.md"],
    source: "https://developers.openai.com/codex/guides/agents-md",
    ...overrides,
  };
}

export function scanResponsePayload(): unknown {
  return {
    target: "github:demo/repo",
    scan_id: 42,
    analysis: {
      root: "github:demo/repo",
      source: {
        provider: "github",
        owner: "demo",
        repo: "repo",
        default_branch: "main",
        commit_sha: "abc123",
      },
      file_count: 2,
      top_level_directories: ["src"],
      detected_stacks: [{ name: "Rust", evidence: ["Cargo.toml"] }],
      health: {
        readme: true,
        gitignore: true,
        ci: false,
        license: false,
        tests: true,
      },
    },
    harness: {
      root: "github:demo/repo",
      score: 75,
      summary: { passed: 1, warnings: 1, failed: 0 },
      checks: [
        check(),
        check({
          harness: "codex",
          id: "codex.skills.present",
          severity: "low",
          status: "warn",
          title: "Codex skills",
          message: "No .agents/skills/*/SKILL.md files were found.",
          evidence: [".agents/skills/"],
          remediation:
            "Add at least one reusable skill as .agents/skills/<name>/SKILL.md.",
        }),
      ],
    },
  };
}
