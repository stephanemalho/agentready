import { LandingIcon } from "@/components/landing/LandingIcon";
import { Card, CardContent } from "@/components/ui/card";
import { landingFeatures } from "@/lib/landing";

export function FeaturesPreview() {
  return (
    <section className="bg-background px-5 py-20 sm:py-24">
      <div className="mx-auto grid w-full max-w-6xl gap-12 lg:grid-cols-[0.9fr_1.1fr] lg:items-start">
        <div className="space-y-5">
          <p
            id="features"
            className="scroll-mt-16 font-mono text-sm font-bold uppercase text-primary"
          >
            Features
          </p>
          <h2 className="max-w-xl text-balance text-4xl font-bold leading-tight sm:text-5xl">
            Everything you need to ship agent-first repos.
          </h2>
          <p className="max-w-xl text-lg leading-8 text-muted-foreground">
            Scan readiness rules once, then share clear reports across local
            CLI usage, pull requests, and hosted repository checks.
          </p>
        </div>

        <div className="grid gap-4 sm:grid-cols-3 lg:grid-cols-1">
          {landingFeatures.map((feature) => (
            <Card key={feature.title} className="bg-card/70">
              <CardContent className="flex gap-4">
                <span className="grid size-10 shrink-0 place-items-center rounded-lg border border-primary/30 bg-primary/10 text-primary">
                  <LandingIcon name={feature.icon} />
                </span>
                <div className="min-w-0 space-y-1">
                  <h3 className="font-semibold">{feature.title}</h3>
                  <p className="text-sm leading-6 text-muted-foreground">
                    {feature.description}
                  </p>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>

        <div className="lg:col-span-2">
          <Card className="bg-card/70">
            <CardContent className="flex flex-col justify-between gap-5 sm:flex-row sm:items-center">
              <div>
                <p className="text-sm font-semibold text-primary">Install</p>
                <p className="text-muted-foreground">
                  Run the same readiness engine locally before opening a PR.
                </p>
              </div>
              <code className="rounded-lg border bg-muted px-4 py-3 font-mono text-sm text-foreground">
                cargo install agentready
              </code>
            </CardContent>
          </Card>
        </div>
      </div>
    </section>
  );
}
