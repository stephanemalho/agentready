import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { BackendStatusDot } from "@/components/BackendStatusDot";
import {
  BackendStatusContext,
  BackendStatusProvider,
} from "@/components/BackendStatusProvider";

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

describe("BackendStatusDot", () => {
  it.each([
    ["checking", "Checking scan engine status"],
    ["waking", "Scan engine is waking up"],
    ["online", "Scan engine online"],
    ["offline", "Scan engine unreachable"],
  ] as const)("announces the %s status", (status, label) => {
    render(
      <BackendStatusContext.Provider value={status}>
        <BackendStatusDot />
      </BackendStatusContext.Provider>,
    );

    expect(screen.getByRole("status").textContent).toContain(label);
  });
});

describe("BackendStatusProvider", () => {
  it("reports online when /api/health answers ok", async () => {
    vi.stubGlobal(
      "fetch",
      vi
        .fn()
        .mockResolvedValue(
          new Response(JSON.stringify({ ok: true }), { status: 200 }),
        ),
    );

    render(
      <BackendStatusProvider>
        <BackendStatusDot />
      </BackendStatusProvider>,
    );

    await waitFor(() =>
      expect(screen.getByRole("status").textContent).toContain(
        "Scan engine online",
      ),
    );
  });

  it("reports waking while /api/health answers not ok", async () => {
    vi.stubGlobal(
      "fetch",
      vi
        .fn()
        .mockResolvedValue(
          new Response(JSON.stringify({ ok: false }), { status: 200 }),
        ),
    );

    render(
      <BackendStatusProvider>
        <BackendStatusDot />
      </BackendStatusProvider>,
    );

    await waitFor(() =>
      expect(screen.getByRole("status").textContent).toContain(
        "Scan engine is waking up",
      ),
    );
  });
});
