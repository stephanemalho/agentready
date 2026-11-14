"use client";

import { useActionState } from "react";
import { ArrowUp, Search } from "lucide-react";

import { scanRepository, type ScanFormState } from "@/app/actions";
import { useBackendStatus } from "@/components/BackendStatusProvider";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

const initialState: ScanFormState = {};

interface ScanFormProps {
  className?: string;
}

export function ScanForm({ className }: ScanFormProps) {
  const [state, formAction, pending] = useActionState(
    scanRepository,
    initialState,
  );
  const backendStatus = useBackendStatus();
  // "checking" stays enabled to avoid a disabled flash on every page load;
  // only a confirmed waking/offline backend blocks the form.
  const engineDown = backendStatus === "waking" || backendStatus === "offline";
  const disabled = pending || engineDown;

  return (
    <form
      action={formAction}
      className={cn("flex w-full flex-col gap-3", className)}
    >
      <div className="relative mx-auto w-full max-w-3xl">
        <div className="relative min-w-0">
          <Search
            aria-hidden="true"
            className="pointer-events-none absolute left-4 top-1/2 size-5 -translate-y-1/2 text-muted-foreground"
          />
          <Input
            name="target"
            placeholder="github:owner/repo or https://github.com/owner/repo"
            required
            aria-label="GitHub repository to scan"
            disabled={disabled}
            className="h-14 rounded-2xl border-border/80 bg-card/90 pr-16 pl-12 font-mono text-sm shadow-lg shadow-foreground/5 backdrop-blur-sm sm:h-16"
          />
          <div className="group/scan absolute top-1/2 right-2.5 -translate-y-1/2">
            <button
              type="submit"
              disabled={disabled}
              aria-label={pending ? "Scanning repository" : "Scan repository"}
              className="inline-flex size-10 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-sm transition-all hover:scale-105 hover:bg-primary/85 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50 sm:size-11"
            >
              <ArrowUp aria-hidden="true" className="size-5" />
            </button>
            <span
              role="tooltip"
              className="pointer-events-none absolute right-0 bottom-full mb-3 w-max rounded-lg border bg-popover px-3 py-2 text-xs font-medium text-popover-foreground opacity-0 shadow-lg transition-all group-hover/scan:translate-y-0 group-hover/scan:opacity-100 group-focus-within/scan:translate-y-0 group-focus-within/scan:opacity-100"
            >
              Scanner votre dépôt
              <span className="absolute top-full right-4 size-2 -translate-y-1/2 rotate-45 border-r border-b bg-popover" />
            </span>
          </div>
        </div>
      </div>
      {pending && (
        <p className="text-muted-foreground text-sm">
          Fetching the repository from GitHub and running the readiness checks...
        </p>
      )}
      {backendStatus === "waking" && (
        <p aria-live="polite" className="text-muted-foreground text-sm">
          The scan engine is waking up (free hosting) - it should be ready in
          about a minute...
        </p>
      )}
      {backendStatus === "offline" && (
        <Alert variant="destructive">
          <AlertDescription>
            The scan engine is unreachable right now. Please try again in a few
            minutes.
          </AlertDescription>
        </Alert>
      )}
      {state.error && (
        <Alert variant="destructive">
          <AlertDescription>{state.error}</AlertDescription>
        </Alert>
      )}
    </form>
  );
}
