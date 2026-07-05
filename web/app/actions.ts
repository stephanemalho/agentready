"use server";

import { redirect } from "next/navigation";

import { ApiError, createScan } from "@/lib/api";

export interface ScanFormState {
  error?: string;
}

const githubTargetPattern =
  /^(github:[\w.-]+\/[\w.-]+|https?:\/\/github\.com\/[\w.-]+\/[\w.-]+\/?)$/;

export async function scanRepository(
  _previous: ScanFormState,
  formData: FormData,
): Promise<ScanFormState> {
  const target = String(formData.get("target") ?? "").trim();

  if (!githubTargetPattern.test(target)) {
    return {
      error: "Enter a GitHub repository as github:owner/repo or a GitHub URL.",
    };
  }

  let scanId: number | undefined;
  try {
    const scan = await createScan(target);
    scanId = scan.scan_id;
  } catch (error) {
    if (error instanceof ApiError) {
      return { error: error.message };
    }
    return { error: "The scan service is unreachable. Try again shortly." };
  }

  if (scanId === undefined) {
    return {
      error:
        "The scan succeeded but the server runs without a database, so the report cannot be stored and displayed.",
    };
  }

  redirect(`/scans/${scanId}`);
}
