import Link from "next/link";

import { listScans } from "@/lib/api";
import { scoreTextClass } from "@/components/display";
import { RescanButton } from "@/components/RescanButton";
import { cn } from "@/lib/utils";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

interface RepoScansPageProps {
  params: Promise<{ owner: string; repo: string }>;
}

export default async function RepoScansPage({ params }: RepoScansPageProps) {
  const { owner, repo } = await params;
  const scans = await listScans(owner, repo);

  return (
    <section className="space-y-6">
      <header className="flex flex-wrap items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold">
            {owner}/{repo}
          </h1>
          <p className="text-muted-foreground text-sm">
            {scans.length} stored scan{scans.length === 1 ? "" : "s"}
          </p>
        </div>
        <RescanButton owner={owner} repo={repo} />
      </header>

      {scans.length === 0 ? (
        <p className="text-muted-foreground">
          No scan stored for this repository yet. Run one from the{" "}
          <Link href="/" className="text-primary underline-offset-4 hover:underline">
            home page
          </Link>
          .
        </p>
      ) : (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Date</TableHead>
              <TableHead>Score</TableHead>
              <TableHead>Checks</TableHead>
              <TableHead>Commit</TableHead>
              <TableHead />
            </TableRow>
          </TableHeader>
          <TableBody>
            {scans.map((scan) => (
              <TableRow key={scan.id}>
                <TableCell>{new Date(scan.created_at).toLocaleString()}</TableCell>
                <TableCell
                  className={cn("font-bold", scoreTextClass(scan.score))}
                >
                  {scan.score}
                </TableCell>
                <TableCell className="text-muted-foreground text-sm">
                  {scan.passed} pass · {scan.warnings} warn · {scan.failed} fail
                </TableCell>
                <TableCell className="font-mono text-xs">
                  {scan.commit_sha ? scan.commit_sha.slice(0, 10) : "—"}
                </TableCell>
                <TableCell>
                  <Link
                    href={`/scans/${scan.id}`}
                    className="text-primary underline-offset-4 hover:underline"
                  >
                    View report
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </section>
  );
}
