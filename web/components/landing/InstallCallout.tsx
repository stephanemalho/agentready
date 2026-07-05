import Link from "next/link";
import { Package, Star } from "lucide-react";

import { CopyCommandButton } from "@/components/landing/CopyCommandButton";

export function InstallCallout() {
  const installCommand = "cargo install agentready";

  return (
    <section
      className="relative isolate overflow-hidden border-t bg-background px-5 py-20 sm:py-24"
    >
      <div className="absolute inset-0 -z-10 bg-agent-grid" />
      <div className="mx-auto w-full max-w-6xl">
        <div className="relative overflow-hidden rounded-lg border bg-card/80 px-5 py-14 text-center sm:px-10 sm:py-20">
          <div className="absolute inset-x-0 top-0 -z-10 h-32 bg-agent-signal" />
          <p
            id="install"
            className="mb-4 scroll-mt-16 font-mono text-sm font-bold uppercase text-primary"
          >
            Install
          </p>
          <div className="mb-6 inline-flex items-center gap-2 rounded-lg border border-primary/30 bg-primary/10 px-3 py-2 font-mono text-xs font-semibold text-primary">
            <Package aria-hidden="true" className="size-4" />
            Available on crates.io - Apache-2.0 / MIT
          </div>

          <h2 className="mx-auto max-w-4xl text-balance text-4xl font-bold leading-tight text-foreground sm:text-5xl">
            Ship agent-ready repos.
            <span className="block">Starting now.</span>
          </h2>
          <p className="mx-auto mt-5 max-w-xl text-lg leading-8 text-muted-foreground">
            One install. Zero cloud calls. Full harness diagnostics in under
            100ms.
          </p>

          <div className="mt-10 flex flex-col items-center justify-center gap-3 sm:flex-row">
            <div className="flex w-full max-w-md items-center gap-2 rounded-lg border bg-background/60 px-4 py-3 font-mono text-sm sm:w-auto">
              <span className="text-primary">$</span>
              <code className="min-w-0 flex-1 overflow-x-auto text-foreground">
                {installCommand}
              </code>
              <CopyCommandButton command={installCommand} />
            </div>
            <Link
              href="https://github.com/stephanemalho/agentready"
              target="_blank"
              rel="noreferrer"
              className="inline-flex h-11 items-center gap-2 rounded-lg bg-primary px-5 text-sm font-semibold text-primary-foreground transition-colors hover:bg-primary/80"
            >
              <Star aria-hidden="true" className="size-4" />
              Star on GitHub
            </Link>
          </div>
        </div>
      </div>
    </section>
  );
}
