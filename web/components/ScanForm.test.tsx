import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { BackendStatusContext } from "@/components/BackendStatusProvider";
import { ScanForm } from "@/components/ScanForm";

// The real server action pulls in server-only modules; the form tests only
// exercise rendering, so a stub is enough.
vi.mock("@/app/actions", () => ({
  scanRepository: vi.fn(),
}));

afterEach(cleanup);

function input(): HTMLInputElement {
  return screen.getByLabelText("GitHub repository to scan");
}

describe("ScanForm", () => {
  it("disables the form and explains while the backend wakes up", () => {
    render(
      <BackendStatusContext.Provider value="waking">
        <ScanForm />
      </BackendStatusContext.Provider>,
    );

    expect(input().disabled).toBe(true);
    expect(
      (screen.getByRole("button", { name: /scan/i }) as HTMLButtonElement)
        .disabled,
    ).toBe(true);
    expect(screen.getByText(/waking up/i)).toBeDefined();
  });

  it("stays enabled when the backend is online", () => {
    render(
      <BackendStatusContext.Provider value="online">
        <ScanForm />
      </BackendStatusContext.Provider>,
    );

    expect(input().disabled).toBe(false);
    expect(screen.queryByText(/waking up/i)).toBeNull();
  });
});
