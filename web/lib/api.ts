import "server-only";

import { apiUrl } from "@/lib/env";
import {
  scanResponseSchema,
  scanSummarySchema,
  storedScanSchema,
  type ScanResponse,
  type ScanSummary,
  type StoredScan,
} from "@/lib/contract";

export class ApiError extends Error {
  constructor(
    message: string,
    public readonly status: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function request(path: string, init?: RequestInit): Promise<unknown> {
  const response = await fetch(`${apiUrl()}${path}`, {
    ...init,
    cache: "no-store",
  });

  const body: unknown = await response.json().catch(() => null);

  if (!response.ok) {
    const message =
      body !== null &&
      typeof body === "object" &&
      "error" in body &&
      typeof body.error === "string"
        ? body.error
        : `API request failed (HTTP ${response.status})`;
    throw new ApiError(message, response.status);
  }

  return body;
}

export async function createScan(target: string): Promise<ScanResponse> {
  const body = await request("/api/scans", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ target }),
  });
  return scanResponseSchema.parse(body);
}

export async function listScans(
  owner: string,
  repo: string,
): Promise<ScanSummary[]> {
  const body = await request(
    `/api/repositories/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/scans`,
  );
  return scanSummarySchema.array().parse(body);
}

export async function getScan(id: number): Promise<StoredScan | null> {
  try {
    const body = await request(`/api/scans/${id}`);
    return storedScanSchema.parse(body);
  } catch (error) {
    if (error instanceof ApiError && error.status === 404) {
      return null;
    }
    throw error;
  }
}
