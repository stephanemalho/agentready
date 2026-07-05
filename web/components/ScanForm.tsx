"use client";

import { useActionState } from "react";

import { scanRepository, type ScanFormState } from "@/app/actions";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const initialState: ScanFormState = {};

export function ScanForm() {
  const [state, formAction, pending] = useActionState(
    scanRepository,
    initialState,
  );

  return (
    <form action={formAction} className="flex w-full max-w-xl flex-col gap-3">
      <div className="flex gap-2">
        <Input
          name="target"
          placeholder="github:owner/repo or https://github.com/owner/repo"
          required
          aria-label="GitHub repository to scan"
          disabled={pending}
        />
        <Button type="submit" disabled={pending}>
          {pending ? "Scanning…" : "Scan"}
        </Button>
      </div>
      {pending && (
        <p className="text-muted-foreground text-sm">
          Fetching the repository from GitHub and running the readiness checks…
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
