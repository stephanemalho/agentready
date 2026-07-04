#!/usr/bin/env bash
set -euo pipefail

required_files=(
  "AGENTS.md"
  "CLAUDE.md"
  "GEMINI.md"
  ".codex/config.toml"
  ".claude/settings.json"
  ".claude/rules/README.md"
  ".gemini/settings.json"
  "docs/agent-rules/README.md"
  "docs/agent-rules/git-workflow.md"
  "docs/agent-rules/verification.md"
  "docs/skills/do_work.md"
  "docs/skills/sync_with_main.md"
  "docs/harness/codex.md"
  "docs/harness/claude-code.md"
  "docs/harness/gemini-cli.md"
  "docs/harness/multi-agent-workflow.md"
  "scripts/agent-preflight.sh"
  "scripts/agent-sync-main.sh"
)

for file in "${required_files[@]}"; do
  if [[ ! -f "${file}" ]]; then
    echo "error: missing required file: ${file}" >&2
    exit 1
  fi
done

if [[ -f ".claude/settings.local.json" ]]; then
  echo "error: .claude/settings.local.json must not be committed" >&2
  exit 1
fi

if ! grep -q "docs/agent-rules" AGENTS.md; then
  echo "error: AGENTS.md must point to docs/agent-rules" >&2
  exit 1
fi

if ! grep -q "@AGENTS.md" CLAUDE.md; then
  echo "error: CLAUDE.md must import AGENTS.md" >&2
  exit 1
fi

if ! grep -q "@AGENTS.md" GEMINI.md; then
  echo "error: GEMINI.md must import AGENTS.md" >&2
  exit 1
fi

python3 -m json.tool .claude/settings.json >/dev/null
python3 -m json.tool .gemini/settings.json >/dev/null

echo "template validation passed"

