import type { ReactNode } from "react";

interface TerminalWindowProps {
  title: string;
  children: ReactNode;
}

export function TerminalWindow({ title, children }: TerminalWindowProps) {
  return (
    <div className="overflow-hidden rounded-lg border bg-card/80">
      <div className="flex items-center gap-3 border-b px-4 py-3">
        <div className="flex gap-2">
          <span className="size-3 rounded-full bg-status-fail" />
          <span className="size-3 rounded-full bg-status-warn" />
          <span className="size-3 rounded-full bg-status-pass" />
        </div>
        <p className="truncate font-mono text-xs text-muted-foreground">{title}</p>
      </div>
      <div className="p-5 font-mono">{children}</div>
    </div>
  );
}
