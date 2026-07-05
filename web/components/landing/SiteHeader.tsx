import Link from "next/link";
import { Star } from "lucide-react";

import { BackendStatusDot } from "@/components/BackendStatusDot";
import { LandingIcon } from "@/components/landing/LandingIcon";
import { ThemeMenu } from "@/components/ThemeMenu";
import { landingNavItems } from "@/lib/landing";

export function SiteHeader() {
  return (
    <header className="sticky top-0 z-30 border-b bg-background/90 backdrop-blur-xl">
      <div className="mx-auto flex h-16 w-full max-w-6xl items-center justify-between gap-4 px-5">
        <div className="flex min-w-0 items-center gap-3">
          <Link href="/#top" className="flex min-w-0 items-center gap-3">
            <span className="grid size-9 place-items-center rounded-lg bg-primary text-primary-foreground">
              <LandingIcon name="terminal" className="size-5" />
            </span>
            <span className="text-lg font-bold">agentready</span>
            <span className="rounded-md border bg-muted px-2 py-1 font-mono text-xs text-muted-foreground">
              v0.1
            </span>
          </Link>
          <BackendStatusDot />
        </div>

        <nav
          aria-label="Primary navigation"
          className="hidden items-center gap-10 text-sm font-medium text-muted-foreground md:flex"
        >
          {landingNavItems.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className="transition-colors hover:text-foreground"
            >
              {item.label}
            </Link>
          ))}
        </nav>

        <div className="flex items-center gap-2">
          <ThemeMenu />
          <Link
            href="https://github.com/stephanemalho/agentready"
            target="_blank"
            rel="noreferrer"
            className="inline-flex h-9 items-center gap-2 rounded-lg border bg-card px-4 text-sm font-medium text-card-foreground transition-colors hover:bg-muted"
          >
            <Star aria-hidden="true" className="size-4 text-primary" />
            <span className="hidden sm:inline">Star on GitHub</span>
          </Link>
        </div>
      </div>
    </header>
  );
}
