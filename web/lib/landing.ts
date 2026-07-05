export type LandingIconName =
  | "check"
  | "chip"
  | "lock"
  | "shield"
  | "sparkles"
  | "terminal"
  | "zap";

export interface LandingNavItem {
  label: string;
  href: string;
}

export interface LandingMetric {
  icon: LandingIconName;
  value: string;
  label: string;
}

export interface LandingFeature {
  icon: LandingIconName;
  title: string;
  description: string;
}

export interface HarnessProfile {
  name: string;
  org: string;
  file: string;
  glyph: string;
  tone: "claude" | "codex" | "gemini";
}

export interface InstallCommand {
  label: string;
  command: string;
}

import type { CheckStatus, Severity } from "@/lib/contract";

// Demo content reuses the real contract vocabulary (rule ids, severities)
// so the landing page showcases exactly what the product outputs.
export interface DemoCheck {
  id: string;
  label: string;
  status: CheckStatus;
  severity: Severity;
}

export const landingNavItems: LandingNavItem[] = [
  { label: "Features", href: "#features" },
  { label: "Harnesses", href: "#harnesses" },
  { label: "Install", href: "#install" },
];

export const supportedHarnesses = ["Claude Code", "Codex", "Gemini CLI"];

export const landingMetrics: LandingMetric[] = [
  { icon: "check", value: "21", label: "Harness checks" },
  { icon: "chip", value: "3", label: "AI agents supported" },
  { icon: "lock", value: "100%", label: "Offline, no API calls" },
  { icon: "zap", value: "~50ms", label: "Scan time" },
];

export const landingFeatures: LandingFeature[] = [
  {
    icon: "shield",
    title: "Evidence-first reports",
    description:
      "Every readiness check links back to the file path that proves the result.",
  },
  {
    icon: "terminal",
    title: "Local-first engine",
    description:
      "The same deterministic Rust core powers CLI scans and hosted reports.",
  },
  {
    icon: "sparkles",
    title: "Harness-aware setup",
    description:
      "Codex, Claude Code, and Gemini CLI rules are checked side by side.",
  },
];

export const harnessProfiles: HarnessProfile[] = [
  {
    name: "Claude Code",
    org: "Anthropic",
    file: "CLAUDE.md",
    glyph: "C",
    tone: "claude",
  },
  {
    name: "Codex",
    org: "OpenAI",
    file: "AGENTS.md",
    glyph: "X",
    tone: "codex",
  },
  {
    name: "Gemini CLI",
    org: "Google",
    file: "GEMINI.md",
    glyph: "G",
    tone: "gemini",
  },
];

export const demoChecks: DemoCheck[] = [
  {
    id: "shared.agents_md.exists",
    label: "AGENTS.md present",
    status: "pass",
    severity: "high",
  },
  {
    id: "claude.adapter.imports_agents_md",
    label: "CLAUDE.md imports AGENTS.md",
    status: "pass",
    severity: "medium",
  },
  {
    id: "gemini.adapter.exists",
    label: "GEMINI.md not found",
    status: "fail",
    severity: "high",
  },
  {
    id: "shared.ci.agent_control",
    label: "CI branch policy detected",
    status: "pass",
    severity: "medium",
  },
  {
    id: "shared.skills.present",
    label: "Shared workflows folder",
    status: "warn",
    severity: "low",
  },
  {
    id: "codex.config.valid_toml",
    label: "Codex config valid",
    status: "pass",
    severity: "high",
  },
];

export const installCommands: InstallCommand[] = [
  { label: "cargo", command: "cargo install agentready" },
  { label: "scan local", command: "agentready harness ./my-project" },
  {
    label: "scan remote",
    command: "agentready harness github:owner/repo",
  },
];
