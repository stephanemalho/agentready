#!/usr/bin/env bash
set -euo pipefail

MAIN_BRANCH="${MAIN_BRANCH:-main}"
REMOTE="${REMOTE:-origin}"
BRANCH="$(git branch --show-current)"

if [[ -z "${BRANCH}" ]]; then
  echo "error: detached HEAD is not allowed for agent work" >&2
  exit 1
fi

if [[ "${BRANCH}" == "${MAIN_BRANCH}" ]]; then
  echo "error: agents must not work directly on ${MAIN_BRANCH}" >&2
  exit 1
fi

if [[ ! "${BRANCH}" =~ ^agent/[a-z0-9._-]+/[a-z0-9._-]+/[a-z0-9._-]+$ && ! "${BRANCH}" =~ ^agent/(codex|claude|gemini)/template$ ]]; then
  echo "error: branch '${BRANCH}' does not match agent/<harness>/<ticket-or-task>/<short-slug>" >&2
  echo "       seed template branches may use agent/<harness>/template" >&2
  exit 1
fi

if git remote get-url "${REMOTE}" >/dev/null 2>&1; then
  git fetch "${REMOTE}" "${MAIN_BRANCH}" --quiet
  REMOTE_MAIN="${REMOTE}/${MAIN_BRANCH}"
  BEHIND="$(git rev-list --count "HEAD..${REMOTE_MAIN}")"
  AHEAD="$(git rev-list --count "${REMOTE_MAIN}..HEAD")"
  echo "branch=${BRANCH}"
  echo "remote_main=${REMOTE_MAIN}"
  echo "ahead=${AHEAD}"
  echo "behind=${BEHIND}"

  if [[ "${BEHIND}" != "0" ]]; then
    echo "error: branch is behind ${REMOTE_MAIN}; run scripts/agent-sync-main.sh" >&2
    exit 2
  fi
else
  echo "warning: remote '${REMOTE}' is not configured; skipped origin/main sync check" >&2
  echo "branch=${BRANCH}"
fi

echo "agent preflight passed"

