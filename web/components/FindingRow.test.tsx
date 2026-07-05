import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { FindingRow } from "@/components/FindingRow";
import { check } from "@/lib/fixtures";

afterEach(cleanup);

describe("FindingRow", () => {
  it("renders title, rule id, severity, and evidence", () => {
    render(<FindingRow check={check()} />);

    expect(screen.getByText("Canonical AGENTS.md")).toBeDefined();
    expect(screen.getByText("shared.agents_md.exists")).toBeDefined();
    expect(screen.getByText("high")).toBeDefined();
    expect(screen.getByText(/Evidence: AGENTS\.md/)).toBeDefined();
  });

  it("shows remediation only for findings that carry one", () => {
    render(
      <FindingRow
        check={check({ status: "warn", remediation: "Add the file." })}
      />,
    );

    expect(screen.getByText(/Add the file\./)).toBeDefined();
    expect(screen.getByText("Warning")).toBeDefined();
  });
});
