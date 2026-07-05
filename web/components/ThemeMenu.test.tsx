import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ThemeProvider } from "next-themes";
import { afterEach, describe, expect, it } from "vitest";

import { ThemeMenu } from "@/components/ThemeMenu";

afterEach(cleanup);

function renderMenu() {
  return render(
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      <ThemeMenu />
    </ThemeProvider>,
  );
}

describe("ThemeMenu", () => {
  it("renders an accessible trigger", () => {
    renderMenu();

    expect(screen.getByRole("button", { name: "Choose theme" })).toBeDefined();
  });

  it("lists the three theme options when opened", async () => {
    const user = userEvent.setup();
    renderMenu();

    await user.click(screen.getByRole("button", { name: "Choose theme" }));

    for (const label of ["Light", "Dark", "System"]) {
      expect(await screen.findByText(label)).toBeDefined();
    }
  });

  it("applies the chosen theme to the document", async () => {
    const user = userEvent.setup();
    renderMenu();

    await user.click(screen.getByRole("button", { name: "Choose theme" }));
    await user.click(await screen.findByText("Dark"));

    expect(document.documentElement.classList.contains("dark")).toBe(true);
  });
});
