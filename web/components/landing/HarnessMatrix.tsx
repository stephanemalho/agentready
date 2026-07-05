import { ChevronRight } from "lucide-react";

import { OutputPreview } from "@/components/landing/OutputPreview";
import { SampleCheckResults } from "@/components/landing/SampleCheckResults";
import { harnessProfiles } from "@/lib/landing";

const toneClasses = {
  claude: "border-harness-claude/30 bg-harness-claude/10 text-harness-claude",
  codex: "border-harness-codex/30 bg-harness-codex/10 text-harness-codex",
  gemini: "border-harness-gemini/30 bg-harness-gemini/10 text-harness-gemini",
};

export function HarnessMatrix() {
  return (
    <section
      className="relative isolate overflow-hidden border-t bg-background px-5 py-20 sm:py-24"
    >
      <div className="absolute inset-0 -z-10 bg-agent-grid" />
      <div className="mx-auto grid w-full max-w-6xl gap-16 lg:grid-cols-2 lg:items-start">
        <div>
          <p
            id="harnesses"
            className="mb-3 scroll-mt-16 font-mono text-sm font-bold uppercase text-primary"
          >
            Harnesses
          </p>
          <h2 className="text-balance text-4xl font-bold leading-tight text-foreground sm:text-5xl">
            Every major agent.
            <span className="block">One unified scan.</span>
          </h2>
          <p className="mt-6 max-w-2xl text-lg leading-8 text-muted-foreground">
            Each AI agent harness has its own instruction file format, config
            convention, and rule set. AgentReady checks all of them in a single
            pass and tells you exactly what to fix.
          </p>

          <div className="mt-10 space-y-3">
            {harnessProfiles.map((harness) => (
              <div
                key={harness.name}
                className="group flex items-center justify-between gap-4 rounded-lg border bg-card/80 p-4 transition-colors hover:bg-muted/30"
              >
                <div className="flex min-w-0 items-center gap-3">
                  <span
                    className={`grid size-9 shrink-0 place-items-center rounded-lg border text-xs font-black ${toneClasses[harness.tone]}`}
                  >
                    {harness.glyph}
                  </span>
                  <span className="min-w-0">
                    <span className="block text-sm font-semibold text-foreground">
                      {harness.name}
                    </span>
                    <span className="block text-xs text-muted-foreground">
                      {harness.org}
                    </span>
                  </span>
                </div>
                <div className="flex shrink-0 items-center gap-2">
                  <code className="rounded-md border bg-muted px-2 py-1 font-mono text-xs text-muted-foreground">
                    {harness.file}
                  </code>
                  <ChevronRight
                    aria-hidden="true"
                    className="size-4 text-muted-foreground transition-colors group-hover:text-primary"
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
