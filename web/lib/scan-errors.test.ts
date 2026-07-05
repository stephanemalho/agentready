import { describe, expect, it } from "vitest";

import {
  scanConfigurationErrorMessage,
  scanServiceUnreachableMessage,
} from "@/lib/scan-errors";

describe("scanConfigurationErrorMessage", () => {
  it("turns a missing API_URL error into an actionable local-dev message", () => {
    expect(
      scanConfigurationErrorMessage(
        new Error(
          "API_URL is not set: point it to the AgentReady API server (e.g. http://localhost:8080)",
        ),
      ),
    ).toBe(
      "The scan service is not configured. Restart the web dev server with API_URL=http://localhost:8080.",
    );
  });

  it("keeps unrelated failures on the generic unreachable message path", () => {
    expect(scanConfigurationErrorMessage(new Error("fetch failed"))).toBeNull();
    expect(scanServiceUnreachableMessage).toBe(
      "The scan service is unreachable. Try again shortly.",
    );
  });
});
