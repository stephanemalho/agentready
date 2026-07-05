import { demoChecks } from "@/lib/landing";

const statusDotClasses = {
  pass: "bg-status-pass",
  warn: "bg-status-warn",
  fail: "bg-status-fail",
};

const statusTextClasses = {
  pass: "text-foreground",
  warn: "text-status-warn",
  fail: "text-status-fail",
};

const severityClasses = {
  critical: "border-status-fail/40 bg-status-fail/10 text-status-fail",
  high: "border-status-warn/40 bg-status-warn/10 text-status-warn",
  medium: "border-primary/30 bg-primary/10 text-primary",
};

export function SampleCheckResults() {
  return (
    <div className="mt-10">
      <p className="mb-4 font-mono text-xs font-semibold uppercase text-muted-foreground">
        Sample check results
      </p>
      <div className="overflow-hidden rounded-lg border bg-card/80">
        {demoChecks.map((check) => (
          <div
            key={check.id}
            className="flex items-center gap-3 border-b px-4 py-3 transition-colors last:border-b-0 hover:bg-muted/30"
          >
            <span className={`size-2 rounded-full ${statusDotClasses[check.status]}`} />
            <span className="w-24 shrink-0 font-mono text-xs text-muted-foreground">
              {check.id}
            </span>
            <span className={`min-w-0 flex-1 text-sm font-medium ${statusTextClasses[check.status]}`}>
              {check.label}
            </span>
            <span
              className={`rounded-md border px-2 py-1 font-mono text-xs font-semibold ${severityClasses[check.severity]}`}
            >
              {check.severity}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
