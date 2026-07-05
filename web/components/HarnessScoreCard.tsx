import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { harnessLabels, scoreTextClass } from "@/components/display";
import type { HarnessScore } from "@/lib/score";

interface HarnessScoreCardProps {
  entry: HarnessScore;
}

export function HarnessScoreCard({ entry }: HarnessScoreCardProps) {
  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium">
          {harnessLabels[entry.harness]}
        </CardTitle>
      </CardHeader>
      <CardContent className="flex items-baseline justify-between">
        <span className={cn("text-3xl font-bold", scoreTextClass(entry.score))}>
          {entry.score}
        </span>
        <span className="text-muted-foreground text-xs">
          {entry.passed} pass · {entry.warnings} warn · {entry.failed} fail
        </span>
      </CardContent>
    </Card>
  );
}
