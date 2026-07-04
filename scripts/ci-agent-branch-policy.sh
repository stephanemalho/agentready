#!/usr/bin/env bash
set -euo pipefail

MAIN_BRANCH="${MAIN_BRANCH:-main}"
BASE_REF="${GITHUB_BASE_REF:-${MAIN_BRANCH}}"
HEAD_REF="${GITHUB_HEAD_REF:-$(git branch --show-current)}"

if [[ "${BASE_REF}" != "${MAIN_BRANCH}" ]]; then
  echo "base branch is ${BASE_REF}; branch policy only enforces PRs into ${MAIN_BRANCH}"
  exit 0
fi

if [[ "${HEAD_REF}" == "${MAIN_BRANCH}" ]]; then
  echo "error: PR head must not be ${MAIN_BRANCH}" >&2
  exit 1
fi

if [[ ! "${HEAD_REF}" =~ ^agent/[a-z0-9._-]+/[a-z0-9._-]+/[a-z0-9._-]+$ && ! "${HEAD_REF}" =~ ^agent/(codex|claude|gemini)/template$ && ! "${HEAD_REF}" =~ ^dependabot/.+ ]]; then
  echo "error: PR branch '${HEAD_REF}' must match agent/<harness>/<ticket-or-task>/<short-slug> or dependabot/*" >&2
  exit 1
fi

git fetch origin "${MAIN_BRANCH}" --quiet
BEHIND="$(git rev-list --count "HEAD..origin/${MAIN_BRANCH}")"

if [[ "${BEHIND}" != "0" ]]; then
  echo "error: branch is behind origin/${MAIN_BRANCH}; sync with main before review" >&2
  exit 2
fi

echo "branch policy passed"
