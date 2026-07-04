use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{Context, bail};
use ignore::WalkBuilder;
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

pub fn analyze_repository(path: impl AsRef<Path>) -> anyhow::Result<RepoAnalysis> {
    let root = path.as_ref();

    if !root.exists() {
        bail!("repository path does not exist: {}", root.display());
    }

    if !root.is_dir() {
        bail!("repository path is not a directory: {}", root.display());
    }

    let root = root
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", root.display()))?;

    let mut files = Vec::new();
    let mut top_level_directories = BTreeSet::new();

    let walker = WalkBuilder::new(&root)
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        let entry = entry.with_context(|| format!("failed to walk {}", root.display()))?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let relative = path
            .strip_prefix(&root)
            .with_context(|| format!("failed to relativize {}", path.display()))?;

        let first_component = relative
            .components()
            .next()
            .map(|component| component.as_os_str().to_string_lossy().into_owned());

        if first_component.as_deref() == Some(".git") {
            continue;
        }

        if let Some(first_component) = first_component
            && entry
                .file_type()
                .is_some_and(|file_type| file_type.is_dir())
        {
            top_level_directories.insert(first_component);
        }

        if entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            files.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }

    files.sort();

    Ok(RepoAnalysis {
        root: root.display().to_string(),
        file_count: files.len(),
        top_level_directories: top_level_directories.into_iter().collect(),
        detected_stacks: detect_stacks(&files),
        health: HealthChecks {
            readme: has_any(&files, &["README.md", "README.markdown"]),
            gitignore: has_any(&files, &[".gitignore"]),
            ci: files
                .iter()
                .any(|file| file.starts_with(".github/workflows/")),
            license: has_any(&files, &["LICENSE", "LICENSE.md", "COPYING"]),
            tests: files
                .iter()
                .any(|file| file.starts_with("tests/") || file.ends_with("_test.rs")),
        },
    })
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

    use super::*;

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

        let analysis = analyze_repository(temp.path()).expect("analysis");

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
}
