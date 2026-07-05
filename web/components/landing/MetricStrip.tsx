import { LandingIcon } from "@/components/landing/LandingIcon";
import { landingMetrics } from "@/lib/landing";

export function MetricStrip() {
  return (
    <section aria-label="AgentReady metrics" className="border-y bg-card/30">
      <div className="mx-auto grid w-full max-w-6xl grid-cols-2 divide-x divide-y divide-border border-x md:grid-cols-4 md:divide-y-0">
        {landingMetrics.map((metric) => (
          <div key={metric.label} className="flex items-center gap-4 px-5 py-7">
            <span className="grid size-10 shrink-0 place-items-center rounded-lg border border-primary/30 bg-primary/10 text-primary">
              <LandingIcon name={metric.icon} />
            </span>
            <p className="min-w-0">
              <span className="block text-2xl font-bold leading-none text-foreground">
                {metric.value}
              </span>
              <span className="mt-1 block text-sm font-medium text-muted-foreground">
                {metric.label}
              </span>
            </p>
          </div>
        ))}
      </div>
    </section>
  );
}
