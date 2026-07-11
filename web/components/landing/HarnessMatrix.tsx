import { ChevronRight } from "lucide-react";

import { harnessToneClasses } from "@/components/display";
import { OutputPreview } from "@/components/landing/OutputPreview";
import { SampleCheckResults } from "@/components/landing/SampleCheckResults";
import { harnessProfiles } from "@/lib/landing";

export function HarnessMatrix() {
  return (
    <section className="bg-background relative isolate overflow-hidden border-t px-5 py-20 sm:py-24">
      <div className="bg-agent-grid absolute inset-0 -z-10" />
      <div className="mx-auto grid w-full max-w-6xl min-w-0 gap-16 lg:grid-cols-2 lg:items-start">
        <div className="min-w-0">
          <p
            id="harnesses"
            className="text-primary mb-3 scroll-mt-16 font-mono text-sm font-bold uppercase"
          >
            Harnesses
          </p>
          <h2 className="text-foreground text-4xl leading-tight font-bold text-balance sm:text-5xl">
            Every major agent.
            <span className="block">One unified scan.</span>
          </h2>
          <p className="text-muted-foreground mt-6 max-w-2xl text-lg leading-8">
            Each AI agent harness has its own instruction file format, config
            convention, and rule set. AgentReady checks all of them in a single
            pass and tells you exactly what to fix.
          </p>

          <div className="mt-10 space-y-3">
            {harnessProfiles.map((harness) => (
              <div
                key={harness.name}
                className="group bg-card/80 hover:bg-muted/30 flex min-w-0 items-center gap-3 rounded-lg border p-4 transition-colors sm:gap-4"
              >
                <div className="flex min-w-0 flex-1 items-center gap-3">
                  <span
                    className={`grid size-9 shrink-0 place-items-center rounded-lg border text-xs font-black ${harnessToneClasses[harness.tone]}`}
                  >
                    {harness.glyph}
                  </span>
                  <span className="min-w-0">
                    <span className="text-foreground block truncate text-sm font-semibold">
                      {harness.name}
                    </span>
                    <span className="text-muted-foreground block truncate text-xs">
                      {harness.org}
                    </span>
                  </span>
                </div>
                <div className="flex min-w-0 items-center gap-2">
                  <code className="bg-muted text-muted-foreground block max-w-28 truncate rounded-md border px-2 py-1 font-mono text-xs">
                    {harness.file}
                  </code>
                  <ChevronRight
                    aria-hidden="true"
                    className="text-muted-foreground group-hover:text-primary size-4 shrink-0 transition-colors"
                  />
                </div>
              </div>
            ))}
          </div>

          <SampleCheckResults />
        </div>

        <OutputPreview />
      </div>
    </section>
  );
}
