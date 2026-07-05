import {
  severityBadgeClasses,
  statusDotClasses,
  statusLabels,
  statusTextClasses,
} from "@/components/display";
import { demoChecks } from "@/lib/landing";
import { cn } from "@/lib/utils";

export function SampleCheckResults() {
  return (
    <div className="mt-10">
      <p className="mb-4 font-mono text-xs font-semibold uppercase text-muted-foreground">
        Sample check results
      </p>
      <ul className="overflow-hidden rounded-lg border bg-card/80">
        {demoChecks.map((check) => (
          <li
            key={check.id}
            className="flex items-center gap-3 border-b px-4 py-3 transition-colors last:border-b-0 hover:bg-muted/30"
          >
            <span
              aria-hidden="true"
              className={cn(
                "size-2 rounded-full",
                statusDotClasses[check.status],
              )}
            />
            <span className="sr-only">{statusLabels[check.status]}:</span>
            <span className="hidden w-56 shrink-0 truncate font-mono text-xs text-muted-foreground sm:block">
              {check.id}
            </span>
            <span
              className={cn(
                "min-w-0 flex-1 text-sm font-medium",
                check.status === "pass"
                  ? "text-foreground"
                  : statusTextClasses[check.status],
              )}
            >
              {check.label}
            </span>
            <span
              className={cn(
                "rounded-md border px-2 py-1 font-mono text-xs font-semibold",
                severityBadgeClasses[check.severity],
              )}
            >
              {check.severity}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
