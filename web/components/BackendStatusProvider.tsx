"use client";

import { createContext, useContext, useEffect, useRef, useState } from "react";

export type BackendStatus = "checking" | "waking" | "online" | "offline";

// Exported so tests (and leaf components) can provide a fixed status.
export const BackendStatusContext = createContext<BackendStatus>("checking");

export function useBackendStatus(): BackendStatus {
  return useContext(BackendStatusContext);
}

const WAKE_POLL_MS = 5_000;
const OFFLINE_POLL_MS = 30_000;
const HEARTBEAT_MS = 10 * 60_000;
// ~90s of failed wake polls before reporting the backend as unreachable
// (a Render free-tier cold start takes up to a minute).
const MAX_WAKE_FAILURES = 18;

// Wake-on-visit strategy: polling /api/health both reports the backend
// state and wakes the free-tier service. Pings run only while the tab is
// visible, so an abandoned tab lets the backend go back to sleep.
export function BackendStatusProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [status, setStatus] = useState<BackendStatus>("checking");
  const failures = useRef(0);

  useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let intervalMs = WAKE_POLL_MS;
    let lastPingAt = 0;

    function schedule(ms: number) {
      intervalMs = ms;
      clearTimeout(timer);
      timer = setTimeout(() => void ping(), ms);
    }

    async function ping() {
      if (document.hidden) {
        schedule(intervalMs);
        return;
      }
      lastPingAt = Date.now();
      let ok = false;
      try {
        const response = await fetch("/api/health", { cache: "no-store" });
        ok =
          response.ok &&
          ((await response.json()) as { ok?: boolean }).ok === true;
      } catch {
        ok = false;
      }
      if (cancelled) return;
      if (ok) {
        failures.current = 0;
        setStatus("online");
        schedule(HEARTBEAT_MS);
        return;
      }
      failures.current += 1;
      if (failures.current >= MAX_WAKE_FAILURES) {
        setStatus("offline");
        schedule(OFFLINE_POLL_MS);
      } else {
        setStatus("waking");
        schedule(WAKE_POLL_MS);
      }
    }

    function onVisibilityChange() {
      if (!document.hidden && Date.now() - lastPingAt >= intervalMs) {
        clearTimeout(timer);
        void ping();
      }
    }

    document.addEventListener("visibilitychange", onVisibilityChange);
    void ping();
    return () => {
      cancelled = true;
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", onVisibilityChange);
    };
  }, []);

  return (
    <BackendStatusContext.Provider value={status}>
      {children}
    </BackendStatusContext.Provider>
  );
}
