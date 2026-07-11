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
  "docs/agent-rules/agent-handoff.md"
  "docs/agent-rules/git-workflow.md"
  "docs/agent-rules/verification.md"
  "docs/developer/agent-journal.md"
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

if git ls-files --error-unmatch ".claude/settings.local.json" >/dev/null 2>&1; then
  echo "error: .claude/settings.local.json must not be committed" >&2
  exit 1
fi

if ! grep -q "docs/agent-rules" AGENTS.md; then
  echo "error: AGENTS.md must point to docs/agent-rules" >&2
  exit 1
fi

policy_files=(
  "AGENTS.md"
  "docs/SETUP.md"
  "docs/CONTROL-FLOW.md"
  "docs/agent-rules/git-workflow.md"
  "docs/harness/multi-agent-workflow.md"
)

obsolete_policy_patterns=(
  "Never run two coding agents in the same worktree"
  "Each model/harness must work in its own git worktree and branch"
  "Each harness gets a separate worktree"
)

for pattern in "${obsolete_policy_patterns[@]}"; do
  if grep -Fq "${pattern}" "${policy_files[@]}"; then
    echo "error: obsolete agent/worktree policy found: ${pattern}" >&2
    exit 1
  fi
done

if grep -q '"worktrees"[[:space:]]*:[[:space:]]*true' .gemini/settings.json; then
  echo "error: Gemini worktree isolation must not be enabled as the project default" >&2
  exit 1
fi

if ! grep -q "multiple native agents or subagents" AGENTS.md; then
  echo "error: AGENTS.md must allow harness-native multi-agent coordination" >&2
  exit 1
fi

if ! grep -q "non-overlapping file ownership" AGENTS.md; then
  echo "error: AGENTS.md must require non-overlapping ownership for parallel writers" >&2
  exit 1
fi

if ! grep -q "explicit approval from the human maintainer" AGENTS.md; then
  echo "error: AGENTS.md must require maintainer approval for additional worktrees" >&2
  exit 1
fi

if ! grep -q 'TARGET_PATH="$2"' scripts/create-agent-worktree.sh; then
  echo "error: worktree helper must require an explicit target path" >&2
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
