import { ScanForm } from "@/components/ScanForm";

export default function HomePage() {
  return (
    <section className="flex flex-col items-start gap-6 py-8">
      <div className="space-y-3">
        <h1 className="text-4xl font-bold tracking-tight">
          Is your repository ready for coding agents?
        </h1>
        <p className="text-muted-foreground max-w-2xl text-lg">
          AgentReady scans a public GitHub repository and checks whether it is
          ready for Codex, Claude Code, and Gemini CLI: canonical AGENTS.md,
          shared rules, harness configuration, CI policy, and more. Every
          finding comes with evidence and a remediation.
        </p>
      </div>
      <ScanForm />
      <p className="text-muted-foreground text-sm">
        Deterministic checks only — no AI model reads your code.
      </p>
    </section>
  );
}
