import {
  CheckCircle2,
  Cpu,
  Lock,
  ShieldCheck,
  Sparkles,
  Terminal,
  Zap,
  type LucideIcon,
} from "lucide-react";

import type { LandingIconName } from "@/lib/landing";
import { cn } from "@/lib/utils";

const icons: Record<LandingIconName, LucideIcon> = {
  check: CheckCircle2,
  chip: Cpu,
  lock: Lock,
  shield: ShieldCheck,
  sparkles: Sparkles,
  terminal: Terminal,
  zap: Zap,
};

interface LandingIconProps {
  name: LandingIconName;
  className?: string;
}

export function LandingIcon({ name, className }: LandingIconProps) {
  const Icon = icons[name];

  return <Icon aria-hidden="true" className={cn("size-4", className)} />;
}
