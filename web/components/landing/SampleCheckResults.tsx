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
    <div className="mt-10 min-w-0">
      <p className="text-muted-foreground mb-4 font-mono text-xs font-semibold uppercase">
        Sample check results
      </p>
      <ul className="bg-card/80 min-w-0 overflow-hidden rounded-lg border">
        {demoChecks.map((check) => (
          <li
            key={check.id}
            className="hover:bg-muted/30 flex min-w-0 items-center gap-3 border-b px-4 py-3 transition-colors last:border-b-0"
          >
            <span
              aria-hidden="true"
              className={cn(
                "size-2 shrink-0 rounded-full",
                statusDotClasses[check.status],
              )}
            />
            <span className="sr-only">{statusLabels[check.status]}:</span>
            <span className="text-muted-foreground hidden w-56 shrink-0 truncate font-mono text-xs sm:block">
              {check.id}
            </span>
            <span
              className={cn(
                "min-w-0 flex-1 truncate text-sm font-medium",
                check.status === "pass"
                  ? "text-foreground"
                  : statusTextClasses[check.status],
              )}
            >
              {check.label}
            </span>
            <span
              className={cn(
                "shrink-0 rounded-md border px-2 py-1 font-mono text-xs font-semibold",
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
