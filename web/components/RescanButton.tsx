"use client";

import { useActionState } from "react";

import { scanRepository, type ScanFormState } from "@/app/actions";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";

interface RescanButtonProps {
  owner: string;
  repo: string;
}

const initialState: ScanFormState = {};

export function RescanButton({ owner, repo }: RescanButtonProps) {
  const [state, formAction, pending] = useActionState(
    scanRepository,
    initialState,
  );

  return (
    <form action={formAction} className="flex flex-col items-end gap-2">
      <input type="hidden" name="target" value={`github:${owner}/${repo}`} />
      <Button type="submit" disabled={pending}>
        {pending ? "Rescanning…" : "Rescan now"}
      </Button>
      {state.error && (
        <Alert variant="destructive">
          <AlertDescription>{state.error}</AlertDescription>
        </Alert>
      )}
    </form>
  );
}
