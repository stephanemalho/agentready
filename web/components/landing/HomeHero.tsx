import { LandingIcon } from "@/components/landing/LandingIcon";
import { MetricStrip } from "@/components/landing/MetricStrip";
import { ScanDock } from "@/components/landing/ScanDock";
import { supportedHarnesses } from "@/lib/landing";

export function HomeHero() {
  return (
    <section
      id="top"
      className="relative isolate scroll-mt-16 overflow-hidden border-b bg-background"
    >
      <div className="absolute inset-0 -z-10 bg-agent-grid" />
      <div className="absolute inset-x-0 top-0 -z-10 h-72 bg-agent-signal" />

      <div className="mx-auto flex min-h-[calc(100vh-4rem)] w-full max-w-6xl flex-col items-center justify-center px-5 py-20 text-center sm:py-24 lg:py-28">
        <div className="inline-flex items-center gap-2 rounded-lg border border-primary/30 bg-primary/10 px-4 py-2 font-mono text-sm font-semibold text-primary">
          <LandingIcon name="sparkles" />
          <span>Supports {supportedHarnesses.join(" · ")}</span>
        </div>

        <div className="mt-10 max-w-5xl space-y-7">
          <h1 className="text-balance text-5xl font-bold leading-none tracking-normal text-foreground sm:text-6xl lg:text-7xl">
            Is your repository
            <span className="block text-primary">ready for AI agents?</span>
          </h1>
          <p className="mx-auto max-w-3xl text-balance text-lg font-medium leading-8 text-muted-foreground sm:text-xl">
            AgentReady scans any GitHub repo and audits its harness readiness for
            every major AI coding agent, with evidence paths and remediation for
            every gap.
          </p>
        </div>

        <div className="mt-12 w-full">
          <ScanDock />
        </div>
      </div>

      <MetricStrip />
    </section>
  );
}
