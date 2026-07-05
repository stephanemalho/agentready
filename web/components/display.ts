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
  info: "border-severity-info text-severity-info",
  low: "border-severity-low text-severity-low",
  medium: "border-severity-medium text-severity-medium",
  high: "border-severity-high text-severity-high",
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
