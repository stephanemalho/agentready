"use client";

import { useState } from "react";

import { CopyCommandButton } from "@/components/landing/CopyCommandButton";
import {
  JsonReportPreview,
  MarkdownReportPreview,
} from "@/components/landing/ReportPreviews";
import { installCommands } from "@/lib/landing";

type OutputFormat = "md" | "json";

const tabs: Array<{ key: OutputFormat; label: string }> = [
  { key: "md", label: "--format markdown" },
  { key: "json", label: "--format json" },
];

export function OutputPreview() {
  const [activeOutput, setActiveOutput] = useState<OutputFormat>("md");

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center gap-2">
        {tabs.map((tab) => (
          <button
            key={tab.key}
            type="button"
            onClick={() => setActiveOutput(tab.key)}
            className={
              activeOutput === tab.key
                ? "rounded-lg border border-primary/30 bg-primary/10 px-3 py-2 font-mono text-xs font-semibold text-primary"
                : "rounded-lg border border-transparent px-3 py-2 font-mono text-xs font-semibold text-muted-foreground transition-colors hover:text-foreground"
            }
          >
            {tab.label}
          </button>
        ))}
      </div>

      {activeOutput === "md" ? <MarkdownReportPreview /> : <JsonReportPreview />}

      <div className="rounded-lg border bg-card/80 p-5">
        <p className="mb-3 font-mono text-xs font-semibold uppercase text-muted-foreground">
          Quick install
        </p>
        <div className="space-y-2">
          {installCommands.map((item) => (
            <div
              key={item.label}
              className="flex items-center gap-2 rounded-lg border bg-background/60 px-3 py-2"
            >
              <span className="shrink-0 font-mono text-xs font-semibold text-primary">
                $
              </span>
              <code className="min-w-0 flex-1 overflow-x-auto font-mono text-xs text-foreground">
                {item.command}
              </code>
              <CopyCommandButton command={item.command} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
