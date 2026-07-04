#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 3 ]]; then
  echo "usage: scripts/create-agent-worktree.sh <harness> <ticket-or-task> <short-slug> [path]" >&2
  echo "example: scripts/create-agent-worktree.sh codex 42 add-auth-tests ../project-codex-42" >&2
  exit 1
fi

HARNESS="$1"
TASK="$2"
SLUG="$3"
TARGET_PATH="${4:-../$(basename "$(pwd)")-${HARNESS}-${TASK}-${SLUG}}"
MAIN_BRANCH="${MAIN_BRANCH:-main}"
REMOTE="${REMOTE:-origin}"

if [[ ! "${HARNESS}" =~ ^[a-z0-9._-]+$ || ! "${TASK}" =~ ^[a-z0-9._-]+$ || ! "${SLUG}" =~ ^[a-z0-9._-]+$ ]]; then
  echo "error: harness, task, and slug must be lowercase ids using letters, numbers, dot, underscore, or dash" >&2
  exit 1
fi

BRANCH="agent/${HARNESS}/${TASK}/${SLUG}"

git fetch "${REMOTE}" "${MAIN_BRANCH}"
git worktree add "${TARGET_PATH}" -b "${BRANCH}" "${REMOTE}/${MAIN_BRANCH}"

echo "created worktree: ${TARGET_PATH}"
echo "created branch: ${BRANCH}"
echo "next:"
echo "  cd ${TARGET_PATH}"
echo "  cp .env.example .env.local"
echo "  scripts/agent-preflight.sh"

