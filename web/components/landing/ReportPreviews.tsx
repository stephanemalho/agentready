import { TerminalWindow } from "@/components/landing/TerminalWindow";
import { demoChecks } from "@/lib/landing";

export function MarkdownReportPreview() {
  return (
    <TerminalWindow title="agentready harness ./project --format markdown">
      <div className="space-y-2 text-xs leading-6">
        <p className="text-sm font-bold text-primary"># AgentReady Report</p>
        <p className="text-muted-foreground">
          **Project:** my-project | **Score:** 6/8 | **Date:** 2026-07-05
        </p>
        <div className="border-t pt-3">
          <p className="font-bold text-foreground">## Critical</p>
          <ReportLine ok id="AGENT-001" label="AGENTS.md present" />
          <ReportLine ok id="AGENT-006" label="Harness config valid" />
        </div>
        <div>
          <p className="font-bold text-foreground">## High</p>
          <ReportLine ok id="AGENT-002" label="CLAUDE.md configured" />
          <ReportLine id="AGENT-003" label="GEMINI.md not found" />
          <ReportLine id="AGENT-008" label="Codex instructions missing" />
        </div>
        <div className="border-t pt-3 text-muted-foreground">
          <p>**Remediation:** Add `GEMINI.md` to repo root</p>
          <p className="text-primary">-&gt; https://ai.google.dev/gemini-api/docs</p>
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
      <pre className="overflow-x-auto text-xs leading-6 text-muted-foreground">
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
    <p className="flex gap-2">
      <span className={ok ? "text-status-pass" : "text-status-fail"}>
        {ok ? "ok" : "x"}
      </span>
      <span className={ok ? "text-foreground" : "text-status-fail"}>{id}</span>
      <span className="text-muted-foreground">{label}</span>
    </p>
  );
}
