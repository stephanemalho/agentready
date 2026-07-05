use anyhow::{Context, bail};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

use agentready::analyzer::RepoAnalysis;
use agentready::harness::HarnessReadinessReport;

pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("failed to connect to the database")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;

    Ok(pool)
}

#[derive(Debug, Serialize)]
pub struct ScanSummary {
    pub id: i64,
    pub commit_sha: Option<String>,
    pub score: i32,
    pub passed: i32,
    pub warnings: i32,
    pub failed: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StoredScan {
    pub id: i64,
    pub owner: String,
    pub name: String,
    pub commit_sha: Option<String>,
    pub score: i32,
    pub created_at: DateTime<Utc>,
    pub report: JsonValue,
}

pub async fn persist_scan(
    pool: &PgPool,
    analysis: &RepoAnalysis,
    harness: &HarnessReadinessReport,
) -> anyhow::Result<i64> {
    let source = &analysis.source;
    let provider = serde_json::to_value(source.provider)?;
    let provider = provider.as_str().unwrap_or("unknown").to_string();
    let (Some(owner), Some(name)) = (source.owner.as_deref(), source.repo.as_deref()) else {
        bail!("cannot persist a scan without repository owner and name");
    };

    let mut transaction = pool.begin().await?;

    let repository_id: i64 = sqlx::query_scalar(
        "INSERT INTO repositories (provider, owner, name, default_branch)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (provider, owner, name)
         DO UPDATE SET default_branch = EXCLUDED.default_branch
         RETURNING id",
    )
    .bind(&provider)
    .bind(owner)
    .bind(name)
    .bind(source.default_branch.as_deref())
    .fetch_one(&mut *transaction)
    .await?;

    let report = serde_json::json!({
        "analysis": analysis,
        "harness": harness,
    });

    let scan_id: i64 = sqlx::query_scalar(
        "INSERT INTO scans (repository_id, commit_sha, score, passed, warnings, failed, report)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id",
    )
    .bind(repository_id)
    .bind(source.commit_sha.as_deref())
    .bind(i32::from(harness.score))
    .bind(harness.summary.passed as i32)
    .bind(harness.summary.warnings as i32)
    .bind(harness.summary.failed as i32)
    .bind(&report)
    .fetch_one(&mut *transaction)
    .await?;

    for check in &harness.checks {
        let check_json = serde_json::to_value(check)?;
        sqlx::query(
            "INSERT INTO findings
                 (scan_id, rule_id, harness, severity, status, title, message, evidence, source, remediation)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(scan_id)
        .bind(&check.id)
        .bind(check_json["harness"].as_str().unwrap_or("unknown"))
        .bind(check_json["severity"].as_str().unwrap_or("unknown"))
        .bind(check_json["status"].as_str().unwrap_or("unknown"))
        .bind(&check.title)
        .bind(&check.message)
        .bind(serde_json::to_value(&check.evidence)?)
        .bind(&check.source)
        .bind(check.remediation.as_deref())
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(scan_id)
}

pub async fn list_scans(
    pool: &PgPool,
    owner: &str,
    name: &str,
) -> anyhow::Result<Vec<ScanSummary>> {
    let rows = sqlx::query(
        "SELECT s.id, s.commit_sha, s.score, s.passed, s.warnings, s.failed, s.created_at
         FROM scans s
         JOIN repositories r ON r.id = s.repository_id
         WHERE r.owner = $1 AND r.name = $2
         ORDER BY s.created_at DESC
         LIMIT 100",
    )
    .bind(owner)
    .bind(name)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ScanSummary {
            id: row.get("id"),
            commit_sha: row.get("commit_sha"),
            score: row.get("score"),
            passed: row.get("passed"),
            warnings: row.get("warnings"),
            failed: row.get("failed"),
            created_at: row.get("created_at"),
        })
        .collect())
}

pub async fn get_scan(pool: &PgPool, id: i64) -> anyhow::Result<Option<StoredScan>> {
    let row = sqlx::query(
        "SELECT s.id, r.owner, r.name, s.commit_sha, s.score, s.created_at, s.report
         FROM scans s
         JOIN repositories r ON r.id = s.repository_id
         WHERE s.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| StoredScan {
        id: row.get("id"),
        owner: row.get("owner"),
        name: row.get("name"),
        commit_sha: row.get("commit_sha"),
        score: row.get("score"),
        created_at: row.get("created_at"),
        report: row.get("report"),
    }))
}
