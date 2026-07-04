use std::collections::BTreeMap;

use anyhow::anyhow;
use serde::Serialize;

use crate::detectors::detect_stacks;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RepoAnalysis {
    pub root: String,
    pub file_count: usize,
    pub top_level_directories: Vec<String>,
    pub detected_stacks: Vec<DetectedStack>,
    pub health: HealthChecks,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DetectedStack {
    pub name: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HealthChecks {
    pub readme: bool,
    pub gitignore: bool,
    pub ci: bool,
    pub license: bool,
    pub tests: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositorySnapshot {
    pub root: String,
    pub files: Vec<String>,
    pub top_level_directories: Vec<String>,
    pub contents: BTreeMap<String, String>,
}

impl RepositorySnapshot {
    pub fn has_file(&self, file: &str) -> bool {
        self.files.iter().any(|candidate| candidate == file)
    }

    pub fn has_dir(&self, dir: &str) -> bool {
        let prefix = format!("{}/", dir.trim_end_matches('/'));
        self.files.iter().any(|file| file.starts_with(&prefix))
    }

    pub fn files_under(&self, dir: &str) -> Vec<String> {
        let prefix = format!("{}/", dir.trim_end_matches('/'));
        self.files
            .iter()
            .filter(|file| file.starts_with(&prefix))
            .cloned()
            .collect()
    }

    pub fn read_file(&self, file: &str) -> anyhow::Result<String> {
        self.contents
            .get(file)
            .cloned()
            .ok_or_else(|| anyhow!("no content available for {file}"))
    }
}

pub fn analyze_repository(snapshot: &RepositorySnapshot) -> RepoAnalysis {
    RepoAnalysis {
        root: snapshot.root.clone(),
        file_count: snapshot.files.len(),
        top_level_directories: snapshot.top_level_directories.clone(),
        detected_stacks: detect_stacks(&snapshot.files),
        health: HealthChecks {
            readme: has_any(&snapshot.files, &["README.md", "README.markdown"]),
            gitignore: has_any(&snapshot.files, &[".gitignore"]),
            ci: snapshot
                .files
                .iter()
                .any(|file| file.starts_with(".github/workflows/")),
            license: has_any(
                &snapshot.files,
                &[
                    "LICENSE",
                    "LICENSE.md",
                    "COPYING",
                    "LICENSE-MIT",
                    "LICENSE-APACHE",
                ],
            ),
            tests: snapshot
                .files
                .iter()
                .any(|file| file.starts_with("tests/") || file.ends_with("_test.rs")),
        },
    }
}

fn has_any(files: &[String], candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| files.iter().any(|file| file == candidate))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::source::{RepositoryTarget, load_snapshot};

    use super::*;

    fn local_snapshot(path: &std::path::Path) -> RepositorySnapshot {
        load_snapshot(&RepositoryTarget::Local(path.to_path_buf())).expect("snapshot")
    }

    #[test]
    fn analyzes_basic_rust_repository() {
        let temp = tempdir().expect("tempdir");
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname = \"demo\"\n",
        )
        .unwrap();
        fs::write(temp.path().join("README.md"), "# Demo\n").unwrap();
        fs::create_dir(temp.path().join("tests")).unwrap();
        fs::write(temp.path().join("tests/cli.rs"), "#[test]\nfn ok() {}\n").unwrap();

        let analysis = analyze_repository(&local_snapshot(temp.path()));

        assert_eq!(analysis.file_count, 3);
        assert!(analysis.health.readme);
        assert!(analysis.health.tests);
        assert!(
            analysis
                .detected_stacks
                .iter()
                .any(|stack| stack.name == "Rust")
        );
    }

    #[test]
    fn detects_dual_license_files() {
        let temp = tempdir().expect("tempdir");
        fs::write(temp.path().join("LICENSE-MIT"), "MIT License\n").unwrap();
        fs::write(temp.path().join("LICENSE-APACHE"), "Apache License\n").unwrap();

        let analysis = analyze_repository(&local_snapshot(temp.path()));

        assert!(analysis.health.license);
    }
}
