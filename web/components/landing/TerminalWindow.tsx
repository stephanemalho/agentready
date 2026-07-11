import type { ReactNode } from "react";

interface TerminalWindowProps {
  title: string;
  children: ReactNode;
}

export function TerminalWindow({ title, children }: TerminalWindowProps) {
  return (
    <div className="bg-card/80 min-w-0 overflow-hidden rounded-lg border">
      <div className="flex min-w-0 items-center gap-3 border-b px-4 py-3">
        <div className="flex shrink-0 gap-2">
          <span className="bg-status-fail size-3 rounded-full" />
          <span className="bg-status-warn size-3 rounded-full" />
          <span className="bg-status-pass size-3 rounded-full" />
        </div>
        <p className="text-muted-foreground min-w-0 truncate font-mono text-xs">
          {title}
        </p>
      </div>
      <div className="min-w-0 overflow-hidden p-5 font-mono">{children}</div>
    </div>
  );
}
