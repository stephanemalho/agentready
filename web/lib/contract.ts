import { z } from "zod";

// Mirrors the Rust engine JSON contract (src/harness/mod.rs, src/analyzer/mod.rs,
// server/src/lib.rs, server/src/store.rs). See docs/client/data-contract.md.
// Objects are loose so additive engine fields never break the UI.

export const checkStatusSchema = z.enum(["pass", "warn", "fail"]);
export const severitySchema = z.enum(["info", "low", "medium", "high"]);
export const harnessNameSchema = z.enum(["shared", "codex", "claude", "gemini"]);

export const harnessCheckSchema = z.looseObject({
  harness: harnessNameSchema,
  id: z.string(),
  severity: severitySchema,
  status: checkStatusSchema,
  title: z.string(),
  message: z.string(),
  evidence: z.array(z.string()),
  source: z.string(),
  remediation: z.string().optional(),
});

export const harnessSummarySchema = z.looseObject({
  passed: z.number().int(),
  warnings: z.number().int(),
  failed: z.number().int(),
});

export const harnessReportSchema = z.looseObject({
  root: z.string(),
  score: z.number(),
  summary: harnessSummarySchema,
  checks: z.array(harnessCheckSchema),
});

export const sourceMetadataSchema = z.looseObject({
  provider: z.enum(["local", "github"]),
  owner: z.string().optional(),
  repo: z.string().optional(),
  default_branch: z.string().optional(),
  commit_sha: z.string().optional(),
});

export const repoAnalysisSchema = z.looseObject({
  root: z.string(),
  source: sourceMetadataSchema,
  file_count: z.number().int(),
  top_level_directories: z.array(z.string()),
  detected_stacks: z.array(
    z.looseObject({ name: z.string(), evidence: z.array(z.string()) }),
  ),
  health: z.looseObject({
    readme: z.boolean(),
    gitignore: z.boolean(),
    ci: z.boolean(),
    license: z.boolean(),
    tests: z.boolean(),
  }),
});

export const scanResponseSchema = z.looseObject({
  target: z.string(),
  scan_id: z.number().int().optional(),
  analysis: repoAnalysisSchema,
  harness: harnessReportSchema,
});

export const scanSummarySchema = z.looseObject({
  id: z.number().int(),
  commit_sha: z.string().nullable(),
  score: z.number(),
  passed: z.number().int(),
  warnings: z.number().int(),
  failed: z.number().int(),
  created_at: z.string(),
});

export const storedScanSchema = z.looseObject({
  id: z.number().int(),
  owner: z.string(),
  name: z.string(),
  commit_sha: z.string().nullable(),
  score: z.number(),
  created_at: z.string(),
  report: z.looseObject({
    analysis: repoAnalysisSchema,
    harness: harnessReportSchema,
  }),
});

export type CheckStatus = z.infer<typeof checkStatusSchema>;
export type Severity = z.infer<typeof severitySchema>;
export type HarnessName = z.infer<typeof harnessNameSchema>;
export type HarnessCheck = z.infer<typeof harnessCheckSchema>;
export type HarnessReport = z.infer<typeof harnessReportSchema>;
export type RepoAnalysis = z.infer<typeof repoAnalysisSchema>;
export type ScanResponse = z.infer<typeof scanResponseSchema>;
export type ScanSummary = z.infer<typeof scanSummarySchema>;
export type StoredScan = z.infer<typeof storedScanSchema>;
