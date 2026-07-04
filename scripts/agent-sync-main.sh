#!/usr/bin/env bash
set -euo pipefail

MAIN_BRANCH="${MAIN_BRANCH:-main}"
REMOTE="${REMOTE:-origin}"
SYNC_MODE="${SYNC_MODE:-rebase}"
BRANCH="$(git branch --show-current)"

if [[ -z "${BRANCH}" ]]; then
  echo "error: detached HEAD is not allowed for agent work" >&2
  exit 1
fi

if [[ "${BRANCH}" == "${MAIN_BRANCH}" ]]; then
  echo "error: refusing to sync while on ${MAIN_BRANCH}" >&2
  exit 1
fi

if ! git remote get-url "${REMOTE}" >/dev/null 2>&1; then
  echo "error: remote '${REMOTE}' is not configured" >&2
  exit 1
fi

git fetch "${REMOTE}" "${MAIN_BRANCH}"
REMOTE_MAIN="${REMOTE}/${MAIN_BRANCH}"
BEHIND="$(git rev-list --count "HEAD..${REMOTE_MAIN}")"

if [[ "${BEHIND}" == "0" ]]; then
  echo "branch '${BRANCH}' is already up to date with ${REMOTE_MAIN}"
  exit 0
fi

echo "syncing '${BRANCH}' with ${REMOTE_MAIN} using ${SYNC_MODE}"

case "${SYNC_MODE}" in
  rebase)
    git rebase "${REMOTE_MAIN}"
    ;;
  merge)
    git merge --no-ff "${REMOTE_MAIN}"
    ;;
  *)
    echo "error: SYNC_MODE must be 'rebase' or 'merge'" >&2
    exit 1
    ;;
esac

scripts/agent-preflight.sh

