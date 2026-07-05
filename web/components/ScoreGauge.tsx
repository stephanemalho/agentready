import { cn } from "@/lib/utils";
import { scoreTextClass } from "@/components/display";

interface ScoreGaugeProps {
  score: number;
  label?: string;
}

export function ScoreGauge({ score, label = "Readiness" }: ScoreGaugeProps) {
  const radius = 52;
  const circumference = 2 * Math.PI * radius;
  const filled = (Math.min(Math.max(score, 0), 100) / 100) * circumference;

  return (
    <figure className="flex flex-col items-center gap-2">
      <svg
        viewBox="0 0 120 120"
        className="size-32"
        role="img"
        aria-label={`${label}: ${score} out of 100`}
      >
        <circle
          cx="60"
          cy="60"
          r={radius}
          fill="none"
          strokeWidth="10"
          className="stroke-muted"
        />
        <circle
          cx="60"
          cy="60"
          r={radius}
          fill="none"
          strokeWidth="10"
          strokeLinecap="round"
          strokeDasharray={`${filled} ${circumference - filled}`}
          transform="rotate(-90 60 60)"
          className={cn("stroke-current", scoreTextClass(score))}
        />
        <text
          x="60"
          y="66"
          textAnchor="middle"
          className={cn("fill-current text-3xl font-bold", scoreTextClass(score))}
        >
          {score}
        </text>
      </svg>
      <figcaption className="text-muted-foreground text-sm">{label}</figcaption>
    </figure>
  );
}
