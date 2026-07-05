"use client";

import {
  useBackendStatus,
  type BackendStatus,
} from "@/components/BackendStatusProvider";
import { cn } from "@/lib/utils";

const dotConfig: Record<BackendStatus, { label: string; className: string }> = {
  checking: {
    label: "Checking scan engine status",
    className: "bg-muted-foreground/60 animate-pulse",
  },
  waking: {
    label: "Scan engine is waking up",
    className: "bg-status-warn animate-pulse",
  },
  online: {
    label: "Scan engine online",
    className: "bg-status-pass animate-pulse",
  },
  offline: {
    label: "Scan engine unreachable",
    className: "bg-status-fail",
  },
};

export function BackendStatusDot() {
  const status = useBackendStatus();
  const { label, className } = dotConfig[status];

  return (
    <span role="status" title={label} className="inline-flex items-center">
      <span aria-hidden="true" className={cn("size-2 rounded-full", className)} />
      <span className="sr-only">{label}</span>
    </span>
  );
}
