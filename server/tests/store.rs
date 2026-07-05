use std::collections::BTreeMap;

use agentready::analyzer::{RepositorySnapshot, RepositorySourceMetadata, analyze_repository};
use agentready::harness::{HarnessFilter, analyze_harness_readiness};
use agentready_server::store;

fn github_snapshot() -> RepositorySnapshot {
    RepositorySnapshot {
        root: "github:demo/repo".to_string(),
        source: RepositorySourceMetadata::github(
            "demo",
            "repo",
            "main",
            Some("abc123".to_string()),
        ),
        files: vec!["AGENTS.md".to_string(), "README.md".to_string()],
        top_level_directories: vec![],
        contents: BTreeMap::from([("AGENTS.md".to_string(), "# Agents\n".to_string())]),
    }
}

#[tokio::test]
async fn persists_lists_and_fetches_scans() {
    let Ok(database_url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("skipping persists_lists_and_fetches_scans: TEST_DATABASE_URL is not set");
        return;
    };

    let pool = store::connect(&database_url).await.expect("connect");

    let snapshot = github_snapshot();
    let analysis = analyze_repository(&snapshot);
    let harness = analyze_harness_readiness(&snapshot, HarnessFilter::All);

    let scan_id = store::persist_scan(&pool, &analysis, &harness)
        .await
        .expect("persist");

    let scans = store::list_scans(&pool, "demo", "repo")
        .await
        .expect("list");
    let summary = scans
        .iter()
        .find(|scan| scan.id == scan_id)
        .expect("scan in history");
    assert_eq!(summary.commit_sha.as_deref(), Some("abc123"));
    assert_eq!(summary.score, i32::from(harness.score));

    let stored = store::get_scan(&pool, scan_id)
        .await
        .expect("get")
        .expect("stored scan");
    assert_eq!(stored.owner, "demo");
    assert_eq!(stored.name, "repo");
    assert!(stored.report["harness"]["score"].is_number());
    assert!(stored.report["analysis"]["file_count"].is_number());

    let missing = store::get_scan(&pool, i64::MAX).await.expect("get missing");
    assert!(missing.is_none());
}
