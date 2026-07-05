import { ScanForm } from "@/components/ScanForm";

export function ScanDock() {
  return (
    <div className="mx-auto w-full max-w-4xl">
      <ScanForm />
      <p className="mt-4 text-left font-mono text-sm font-medium text-muted-foreground">
        Deterministic checks only - no AI model reads your code.
      </p>
    </div>
  );
}
