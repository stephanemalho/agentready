"use client";

import { useSyncExternalStore } from "react";
import { useTheme } from "next-themes";
import { Check, SwatchBook } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

const themeOptions = [
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
  { value: "system", label: "System" },
] as const;

// The active theme is only known client-side; render the check mark only
// after hydration to avoid a server/client mismatch.
const emptySubscribe = () => () => {};
function useMounted(): boolean {
  return useSyncExternalStore(
    emptySubscribe,
    () => true,
    () => false,
  );
}

export function ThemeMenu() {
  const { theme, setTheme } = useTheme();
  const mounted = useMounted();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button variant="outline" size="icon" aria-label="Choose theme">
            <SwatchBook aria-hidden="true" className="size-4" />
          </Button>
        }
      />
      <DropdownMenuContent align="end">
        {themeOptions.map((option) => (
          <DropdownMenuItem
            key={option.value}
            onClick={() => setTheme(option.value)}
          >
            <span className="flex-1">{option.label}</span>
            {mounted && theme === option.value && (
              <Check aria-hidden="true" className="size-4" />
            )}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
