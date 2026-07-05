import { describe, expect, it } from "vitest";

import { scanResponseSchema, storedScanSchema } from "@/lib/contract";
import { scanResponsePayload } from "@/lib/fixtures";

describe("scanResponseSchema", () => {
  it("parses a full engine payload", () => {
    const parsed = scanResponseSchema.parse(scanResponsePayload());

    expect(parsed.scan_id).toBe(42);
    expect(parsed.harness.checks).toHaveLength(2);
    expect(parsed.analysis.source.commit_sha).toBe("abc123");
  });

  it("accepts additive unknown fields (contract is additive)", () => {
    const payload = scanResponsePayload() as Record<string, unknown>;
    payload["future_field"] = true;

    expect(() => scanResponseSchema.parse(payload)).not.toThrow();
  });

  it("rejects payloads with invalid enums", () => {
    const payload = scanResponsePayload() as {
      harness: { checks: Array<Record<string, unknown>> };
    };
    payload.harness.checks[0]!["status"] = "maybe";

    expect(() => scanResponseSchema.parse(payload)).toThrow();
  });
});

describe("storedScanSchema", () => {
  it("parses a stored scan with nullable commit_sha", () => {
    const stored = {
      id: 1,
      owner: "demo",
      name: "repo",
      commit_sha: null,
      score: 75,
      created_at: "2026-07-05T08:00:00Z",
      report: {
        analysis: (scanResponsePayload() as { analysis: unknown }).analysis,
        harness: (scanResponsePayload() as { harness: unknown }).harness,
      },
    };

    const parsed = storedScanSchema.parse(stored);

    expect(parsed.commit_sha).toBeNull();
    expect(parsed.report.harness.score).toBe(75);
  });
});
