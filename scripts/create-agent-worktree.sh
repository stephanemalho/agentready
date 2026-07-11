#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: scripts/create-agent-worktree.sh <harness> <explicit-path>" >&2
  echo "example: scripts/create-agent-worktree.sh codex ../repolens-codex" >&2
  echo "note: requires explicit maintainer approval and checks out an existing assigned branch" >&2
  exit 1
fi

HARNESS="$1"
TARGET_PATH="$2"
MAIN_BRANCH="${MAIN_BRANCH:-main}"
REMOTE="${REMOTE:-origin}"

if [[ ! "${HARNESS}" =~ ^(codex|claude|gemini)$ ]]; then
  echo "error: harness must be codex, claude, or gemini" >&2
  exit 1
fi

BRANCH="agent/${HARNESS}/bootstrap/repolens-cli"

git fetch "${REMOTE}" "${MAIN_BRANCH}"

if git show-ref --verify --quiet "refs/heads/${BRANCH}"; then
  git worktree add "${TARGET_PATH}" "${BRANCH}"
elif git show-ref --verify --quiet "refs/remotes/${REMOTE}/${BRANCH}"; then
  git worktree add "${TARGET_PATH}" --track -b "${BRANCH}" "${REMOTE}/${BRANCH}"
else
  echo "error: assigned branch '${BRANCH}' does not exist locally or on ${REMOTE}" >&2
  echo "       the human maintainer must create assigned branches" >&2
  exit 1
fi

echo "worktree ready: ${TARGET_PATH}"
echo "branch: ${BRANCH}"
echo "next:"
echo "  cd ${TARGET_PATH}"
echo "  cp .env.example .env.local"
echo "  scripts/agent-preflight.sh"
