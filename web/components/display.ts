import type { CheckStatus, HarnessName, Severity } from "@/lib/contract";

// The single mapping from contract enums to labels and token-backed classes.
// Components must use these helpers instead of inlining colors (docs/client/style.md).

export const harnessLabels: Record<HarnessName, string> = {
  shared: "Shared Workflow",
  codex: "Codex",
  claude: "Claude Code",
  gemini: "Gemini CLI",
};

export const statusLabels: Record<CheckStatus, string> = {
  pass: "Pass",
  warn: "Warning",
  fail: "Fail",
};

export const statusTextClasses: Record<CheckStatus, string> = {
  pass: "text-status-pass",
  warn: "text-status-warn",
  fail: "text-status-fail",
};

export const severityBadgeClasses: Record<Severity, string> = {
  info: "border-severity-info/40 bg-severity-info/10 text-severity-info",
  low: "border-severity-low/40 bg-severity-low/10 text-severity-low",
  medium: "border-severity-medium/40 bg-severity-medium/10 text-severity-medium",
  high: "border-severity-high/40 bg-severity-high/10 text-severity-high",
};

export const statusDotClasses: Record<CheckStatus, string> = {
  pass: "bg-status-pass",
  warn: "bg-status-warn",
  fail: "bg-status-fail",
};

export type HarnessTone = "claude" | "codex" | "gemini";

export const harnessToneClasses: Record<HarnessTone, string> = {
  claude: "border-harness-claude/30 bg-harness-claude/10 text-harness-claude",
  codex: "border-harness-codex/30 bg-harness-codex/10 text-harness-codex",
  gemini: "border-harness-gemini/30 bg-harness-gemini/10 text-harness-gemini",
};

export function scoreTextClass(score: number): string {
  if (score >= 90) {
    return "text-status-pass";
  }
  if (score >= 60) {
    return "text-status-warn";
  }
  return "text-status-fail";
}
