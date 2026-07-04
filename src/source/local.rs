use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, bail};
use ignore::WalkBuilder;

use crate::analyzer::{RepositorySnapshot, RepositorySourceMetadata};
use crate::harness::content_paths;

pub fn snapshot(path: &Path) -> anyhow::Result<RepositorySnapshot> {
    if !path.exists() {
        bail!("repository path does not exist: {}", path.display());
    }

    if !path.is_dir() {
        bail!("repository path is not a directory: {}", path.display());
    }

    let root = path
        .canonicalize()
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;

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

    let mut contents = BTreeMap::new();
    for file in content_paths(&files) {
        if let Ok(text) = fs::read_to_string(root.join(&file)) {
            contents.insert(file, text);
        }
    }

    Ok(RepositorySnapshot {
        root: root.display().to_string(),
        source: RepositorySourceMetadata::local(),
        files,
        top_level_directories: top_level_directories.into_iter().collect(),
        contents,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn snapshots_files_and_prefetches_harness_contents() {
        let repo = tempdir().expect("tempdir");
        fs::write(repo.path().join("AGENTS.md"), "# Agents\n").unwrap();
        fs::write(repo.path().join("notes.txt"), "not prefetched\n").unwrap();
        fs::create_dir(repo.path().join("src")).unwrap();
        fs::write(repo.path().join("src/main.rs"), "fn main() {}\n").unwrap();

        let snapshot = snapshot(repo.path()).expect("snapshot");

        assert!(snapshot.has_file("AGENTS.md"));
        assert!(snapshot.has_file("src/main.rs"));
        assert_eq!(snapshot.source, RepositorySourceMetadata::local());
        assert_eq!(snapshot.top_level_directories, vec!["src".to_string()]);
        assert_eq!(
            snapshot.read_file("AGENTS.md").expect("content"),
            "# Agents\n"
        );
        assert!(snapshot.read_file("notes.txt").is_err());
        assert!(snapshot.read_file("missing.md").is_err());
    }

    #[test]
    fn rejects_missing_paths() {
        let error = snapshot(Path::new("/nonexistent/agentready-test-path")).unwrap_err();

        assert!(error.to_string().contains("does not exist"));
    }
}
