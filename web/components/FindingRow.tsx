import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import {
  severityBadgeClasses,
  statusLabels,
  statusTextClasses,
} from "@/components/display";
import type { HarnessCheck } from "@/lib/contract";

interface FindingRowProps {
  check: HarnessCheck;
}

export function FindingRow({ check }: FindingRowProps) {
  return (
    <li className="border-b py-4 last:border-b-0">
      <div className="flex flex-wrap items-center gap-2">
        <span
          className={cn("text-sm font-semibold", statusTextClasses[check.status])}
        >
          {statusLabels[check.status]}
        </span>
        <h3 className="font-medium">{check.title}</h3>
        <Badge variant="outline" className="font-mono text-xs">
          {check.id}
        </Badge>
        <Badge
          variant="outline"
          className={cn("text-xs capitalize", severityBadgeClasses[check.severity])}
        >
          {check.severity}
        </Badge>
      </div>
      <p className="text-muted-foreground mt-1 text-sm">{check.message}</p>
      {check.evidence.length > 0 && (
        <p className="mt-1 font-mono text-xs">
          Evidence: {check.evidence.join(", ")}
        </p>
      )}
      {check.remediation && (
        <p className="mt-1 text-sm">
          <span className="font-medium">Remediation:</span> {check.remediation}
        </p>
      )}
    </li>
  );
}
