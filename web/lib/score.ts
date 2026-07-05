import type { HarnessCheck, HarnessName } from "@/lib/contract";

export interface HarnessScore {
  harness: HarnessName;
  score: number;
  passed: number;
  warnings: number;
  failed: number;
}

// Same formula as the engine (src/harness/mod.rs::score):
// round((passed + 0.5 * warnings) / total * 100), 0 when total is 0.
export function scoreFromCounts(
  passed: number,
  warnings: number,
  failed: number,
): number {
  const total = passed + warnings + failed;
  if (total === 0) {
    return 0;
  }
  return Math.round(((passed + 0.5 * warnings) / total) * 100);
}

export function perHarnessScores(checks: HarnessCheck[]): HarnessScore[] {
  const order: HarnessName[] = ["shared", "codex", "claude", "gemini"];

  return order
    .map((harness) => {
      const own = checks.filter((check) => check.harness === harness);
      const passed = own.filter((check) => check.status === "pass").length;
      const warnings = own.filter((check) => check.status === "warn").length;
      const failed = own.filter((check) => check.status === "fail").length;
      return {
        harness,
        score: scoreFromCounts(passed, warnings, failed),
        passed,
        warnings,
        failed,
      };
    })
    .filter((entry) => entry.passed + entry.warnings + entry.failed > 0);
}
