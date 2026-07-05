"use client";

import { useActionState } from "react";
import { Search, Zap } from "lucide-react";

import { scanRepository, type ScanFormState } from "@/app/actions";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
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

  return (
    <form
      action={formAction}
      className={cn("flex w-full flex-col gap-3", className)}
    >
      <div className="flex flex-col gap-3 sm:flex-row">
        <div className="relative min-w-0 flex-1">
          <Search
            aria-hidden="true"
            className="pointer-events-none absolute left-4 top-1/2 size-5 -translate-y-1/2 text-muted-foreground"
          />
          <Input
            name="target"
            placeholder="github:owner/repo or https://github.com/owner/repo"
            required
            aria-label="GitHub repository to scan"
            disabled={pending}
            className="h-12 bg-card/70 pl-12 font-mono text-sm sm:h-14"
          />
        </div>
        <Button
          type="submit"
          disabled={pending}
          className="h-12 gap-2 px-6 text-base sm:h-14 sm:px-8"
        >
          <Zap aria-hidden="true" className="size-4" />
          {pending ? "Scanning..." : "Scan"}
        </Button>
      </div>
      {pending && (
        <p className="text-muted-foreground text-sm">
          Fetching the repository from GitHub and running the readiness checks...
        </p>
      )}
      {state.error && (
        <Alert variant="destructive">
          <AlertDescription>{state.error}</AlertDescription>
        </Alert>
      )}
    </form>
  );
}
