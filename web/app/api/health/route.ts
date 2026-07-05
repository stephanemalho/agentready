import { NextResponse } from "next/server";

import { apiUrl } from "@/lib/env";

export const dynamic = "force-dynamic";

// Health proxy for the browser (BFF: the client never calls the Rust API).
// Hitting this route also wakes the Render free-tier service; a cold start
// takes ~1 min, so the fetch below times out fast and the client retries.
export async function GET() {
  try {
    const response = await fetch(`${apiUrl()}/health`, {
      cache: "no-store",
      signal: AbortSignal.timeout(8_000),
    });
    return NextResponse.json({ ok: response.ok });
  } catch {
    return NextResponse.json({ ok: false });
  }
}
