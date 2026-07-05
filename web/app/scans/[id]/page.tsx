import Link from "next/link";
import { notFound } from "next/navigation";

import { getScan } from "@/lib/api";
import { perHarnessScores } from "@/lib/score";
import { harnessLabels } from "@/components/display";
import { FindingRow } from "@/components/FindingRow";
import { HarnessScoreCard } from "@/components/HarnessScoreCard";
import { ScoreGauge } from "@/components/ScoreGauge";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";

interface ScanPageProps {
  params: Promise<{ id: string }>;
}

export default async function ScanPage({ params }: ScanPageProps) {
  const { id } = await params;
  const scanId = Number(id);
  if (!Number.isInteger(scanId)) {
    notFound();
  }

  const scan = await getScan(scanId);
  if (!scan) {
    notFound();
  }

  const { harness } = scan.report;
  const harnessScores = perHarnessScores(harness.checks);
  const harnesses = harnessScores.map((entry) => entry.harness);

  return (
    <article className="mx-auto w-full max-w-6xl space-y-8 px-5 py-10">
      <header className="flex flex-wrap items-center justify-between gap-6">
        <div className="space-y-1">
          <h1 className="text-2xl font-bold">
            {scan.owner}/{scan.name}
          </h1>
          <p className="text-muted-foreground text-sm">
            Scanned {new Date(scan.created_at).toLocaleString()}
            {scan.commit_sha && (
              <>
                {" · commit "}
                <span className="font-mono">{scan.commit_sha.slice(0, 10)}</span>
              </>
            )}
          </p>
          <p className="text-sm">
            {harness.summary.passed} passed · {harness.summary.warnings}{" "}
            warnings · {harness.summary.failed} failed ·{" "}
            <Link
              href={`/repos/${scan.owner}/${scan.name}`}
              className="text-primary underline-offset-4 hover:underline"
            >
              scan history
            </Link>
          </p>
        </div>
        <ScoreGauge score={scan.score} />
      </header>

      <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {harnessScores.map((entry) => (
          <HarnessScoreCard key={entry.harness} entry={entry} />
        ))}
      </section>

      <Separator />

      {harnesses.map((harnessName) => (
        <section key={harnessName} className="space-y-2">
          <h2 className="flex items-center gap-2 text-lg font-semibold">
            {harnessLabels[harnessName]}
            <Badge variant="secondary">
              {harness.checks.filter((c) => c.harness === harnessName).length}{" "}
              checks
            </Badge>
          </h2>
          <ul>
            {harness.checks
              .filter((check) => check.harness === harnessName)
              .map((check) => (
                <FindingRow key={`${check.id}-${check.evidence[0]}`} check={check} />
              ))}
          </ul>
        </section>
      ))}
    </article>
  );
}
