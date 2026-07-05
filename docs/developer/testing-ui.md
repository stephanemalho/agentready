# Running The UI Tests (web/)

Day-to-day guide for the front-end test suite. The policy (what must be tested) lives in `docs/client/testing.md`; this page is the practical how-to.

## Commands

```bash
cd web

npm run test              # run the whole suite once (vitest)
npx vitest                # watch mode while developing
npx vitest run score      # run only files matching "score"
npx vitest run --coverage # with a coverage report
```

The suite also runs in CI on every PR touching `web/**` (`.github/workflows/web-ci.yml`).

## Where Tests Live

Next to the code they cover, same name plus `.test`:

```txt
web/lib/score.ts            -> web/lib/score.test.ts
web/components/ThemeMenu.tsx -> web/components/ThemeMenu.test.tsx
```

## Stack

- **Vitest** runs the tests (config: `web/vitest.config.ts`, jsdom environment).
- **Testing Library** renders components and queries them like a user would (`getByRole`, `getByText`).
- **user-event** simulates real interactions (clicks, keyboard) for interactive components.
- Fixtures come from `web/lib/fixtures.ts` — extend that module instead of duplicating payloads.

## Writing A Component Test (pattern)

```tsx
import { cleanup, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it } from "vitest";

afterEach(cleanup);

it("does the user-visible thing", async () => {
  const user = userEvent.setup();
  render(<MyComponent />);

  await user.click(screen.getByRole("button", { name: "Choose theme" }));

  expect(await screen.findByText("Dark")).toBeDefined();
});
```

Rules of thumb:

- Query by **role and accessible name** first (`getByRole`) — it tests accessibility for free.
- Components needing context get it in the test (e.g. `ThemeMenu` is wrapped in `ThemeProvider`; see `ThemeMenu.test.tsx`).
- Pure `lib/` modules are tested without the DOM; keep them import-light so this stays possible.
- No network in tests, ever. API payloads are fixtures.
