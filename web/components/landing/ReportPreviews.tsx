import { TerminalWindow } from "@/components/landing/TerminalWindow";
import { demoChecks } from "@/lib/landing";

export function MarkdownReportPreview() {
  return (
    <TerminalWindow title="agentready harness ./project --format markdown">
      <div className="min-w-0 space-y-2 text-xs leading-6">
        <p className="text-primary text-sm font-bold"># AgentReady Report</p>
        <p className="text-muted-foreground break-words">
          **Project:** my-project | **Score:** 6/8 | **Date:** 2026-07-05
        </p>
        <div className="border-t pt-3">
          <p className="text-foreground font-bold">## Critical</p>
          <ReportLine ok id="AGENT-001" label="AGENTS.md present" />
          <ReportLine ok id="AGENT-006" label="Harness config valid" />
        </div>
        <div>
          <p className="text-foreground font-bold">## High</p>
          <ReportLine ok id="AGENT-002" label="CLAUDE.md configured" />
          <ReportLine id="AGENT-003" label="GEMINI.md not found" />
          <ReportLine id="AGENT-008" label="Codex instructions missing" />
        </div>
        <div className="text-muted-foreground border-t pt-3">
          <p>**Remediation:** Add `GEMINI.md` to repo root</p>
          <p className="text-primary break-all">
            -&gt; https://ai.google.dev/gemini-api/docs
          </p>
        </div>
      </div>
    </TerminalWindow>
  );
}

export function JsonReportPreview() {
  const body = {
    project: "my-project",
    date: "2026-07-05",
    score: { passed: 6, total: 8 },
    checks: demoChecks.map((check) => ({
      id: check.id,
      status: check.status,
      severity: check.severity,
    })),
  };

  return (
    <TerminalWindow title="agentready harness ./project --format json">
      <pre className="text-muted-foreground max-w-full overflow-x-auto text-xs leading-6">
        {JSON.stringify(body, null, 2)}
      </pre>
    </TerminalWindow>
  );
}

interface ReportLineProps {
  id: string;
  label: string;
  ok?: boolean;
}

function ReportLine({ id, label, ok = false }: ReportLineProps) {
  return (
    <p className="flex min-w-0 gap-2">
      <span
        className={
          ok ? "text-status-pass shrink-0" : "text-status-fail shrink-0"
        }
      >
        {ok ? "ok" : "x"}
      </span>
      <span
        className={
          ok ? "text-foreground shrink-0" : "text-status-fail shrink-0"
        }
      >
        {id}
      </span>
      <span className="text-muted-foreground min-w-0 truncate">{label}</span>
    </p>
  );
}
