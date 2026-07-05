import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SampleCheckResults } from "@/components/landing/SampleCheckResults";
import { demoChecks } from "@/lib/landing";

afterEach(cleanup);

describe("SampleCheckResults", () => {
  it("renders every demo check with its real rule id", () => {
    render(<SampleCheckResults />);

    for (const check of demoChecks) {
      expect(screen.getByText(check.id)).toBeDefined();
      expect(screen.getByText(check.label)).toBeDefined();
    }
  });

  it("announces the status to screen readers", () => {
    render(<SampleCheckResults />);

    expect(screen.getAllByText("Fail:").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Warning:").length).toBeGreaterThan(0);
  });
});
