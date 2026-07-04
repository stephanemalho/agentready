#!/usr/bin/env bash
set -euo pipefail

if git ls-files | grep -E '(^|/)\.env(\.|$)|(^|/)secrets/' | grep -v -E '(^|/)\.env\.example$' >/tmp/agent-secret-files.txt; then
  echo "error: committed env/secret-like files detected:" >&2
  cat /tmp/agent-secret-files.txt >&2
  exit 1
fi

if git grep -n -I -E '(sk-[A-Za-z0-9_-]{20,}|ghp_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|AIza[0-9A-Za-z_-]{20,})' -- . ':!README.md' ':!docs/**' >/tmp/agent-secret-grep.txt; then
  echo "error: possible committed secret detected:" >&2
  cat /tmp/agent-secret-grep.txt >&2
  exit 1
fi

echo "basic secret scan passed"
