import { describe, expect, it } from "vitest";

import { perHarnessScores, scoreFromCounts } from "@/lib/score";
import { check } from "@/lib/fixtures";

describe("scoreFromCounts", () => {
  it("matches the engine formula", () => {
    // 21 passed, 0 warn, 0 fail -> 100 (the repository's own score)
    expect(scoreFromCounts(21, 0, 0)).toBe(100);
    // 1 passed, 1 warn, 0 fail -> round(1.5 / 2 * 100) = 75
    expect(scoreFromCounts(1, 1, 0)).toBe(75);
    expect(scoreFromCounts(0, 0, 5)).toBe(0);
  });

  it("returns 0 when there are no checks", () => {
    expect(scoreFromCounts(0, 0, 0)).toBe(0);
  });
});

describe("perHarnessScores", () => {
  it("groups checks by harness and scores each group", () => {
    const checks = [
      check({ harness: "shared", status: "pass" }),
      check({ harness: "codex", status: "warn" }),
      check({ harness: "codex", status: "pass" }),
      check({ harness: "claude", status: "fail" }),
    ];

    const scores = perHarnessScores(checks);

    expect(scores).toEqual([
      { harness: "shared", score: 100, passed: 1, warnings: 0, failed: 0 },
      { harness: "codex", score: 75, passed: 1, warnings: 1, failed: 0 },
      { harness: "claude", score: 0, passed: 0, warnings: 0, failed: 1 },
    ]);
  });

  it("omits harnesses without checks", () => {
    const scores = perHarnessScores([check({ harness: "gemini" })]);

    expect(scores).toHaveLength(1);
    expect(scores[0]?.harness).toBe("gemini");
  });
});
