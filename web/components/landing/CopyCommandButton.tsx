"use client";

import { useState } from "react";
import { Check, Copy } from "lucide-react";

interface CopyCommandButtonProps {
  command: string;
}

export function CopyCommandButton({ command }: CopyCommandButtonProps) {
  const [copied, setCopied] = useState(false);

  function copyCommand(): void {
    void navigator.clipboard
      .writeText(command)
      .then(() => {
        setCopied(true);
        window.setTimeout(() => setCopied(false), 1200);
      })
      .catch(() => setCopied(false));
  }

  return (
    <button
      type="button"
      onClick={copyCommand}
      className="inline-flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
      aria-label={`Copy command: ${command}`}
    >
      {copied ? (
        <Check aria-hidden="true" className="size-4 text-primary" />
      ) : (
        <Copy aria-hidden="true" className="size-4" />
      )}
      <span aria-live="polite" className="sr-only">
        {copied ? "Command copied" : ""}
      </span>
    </button>
  );
}
