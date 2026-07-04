use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, bail};
use serde_json::Value as JsonValue;

use crate::analyzer::RepositorySnapshot;
use crate::harness::content_paths;

const API_ROOT: &str = "https://api.github.com";

pub fn snapshot(owner: &str, repo: &str) -> anyhow::Result<RepositorySnapshot> {
    let token = std::env::var("GITHUB_TOKEN").ok();

    let metadata = get_json(
        &format!("{API_ROOT}/repos/{owner}/{repo}"),
        token.as_deref(),
    )
    .with_context(|| format!("failed to read GitHub repository github:{owner}/{repo}"))?;
    let default_branch = parse_default_branch(&metadata)?;

    let tree = get_json(
        &format!("{API_ROOT}/repos/{owner}/{repo}/git/trees/{default_branch}?recursive=1"),
        token.as_deref(),
    )
    .with_context(|| format!("failed to list files of github:{owner}/{repo}"))?;
    let listing = parse_tree(&tree)?;

    if listing.truncated {
        bail!(
            "github:{owner}/{repo} is too large: the GitHub tree listing was truncated; \
clone it locally and scan the local path instead"
        );
    }

    let mut contents = BTreeMap::new();
    for file in content_paths(&listing.files) {
        let text = get_raw(
            &format!("{API_ROOT}/repos/{owner}/{repo}/contents/{file}?ref={default_branch}"),
            token.as_deref(),
        )
        .with_context(|| format!("failed to fetch {file} from github:{owner}/{repo}"))?;

        if let Some(text) = text {
            contents.insert(file, text);
        }
    }

    Ok(RepositorySnapshot {
        root: format!("github:{owner}/{repo}"),
        files: listing.files,
        top_level_directories: listing.top_level_directories,
        contents,
    })
}

struct TreeListing {
    files: Vec<String>,
    top_level_directories: Vec<String>,
    truncated: bool,
}

fn parse_default_branch(metadata: &JsonValue) -> anyhow::Result<String> {
    metadata
        .pointer("/default_branch")
        .and_then(JsonValue::as_str)
        .map(str::to_string)
        .context("GitHub repository metadata has no default_branch field")
}

fn parse_tree(tree: &JsonValue) -> anyhow::Result<TreeListing> {
    let entries = tree
        .pointer("/tree")
        .and_then(JsonValue::as_array)
        .context("GitHub tree response has no tree array")?;

    let mut files = Vec::new();
    let mut top_level_directories = BTreeSet::new();

    for entry in entries {
        let Some(path) = entry.pointer("/path").and_then(JsonValue::as_str) else {
            continue;
        };

        match entry.pointer("/type").and_then(JsonValue::as_str) {
            Some("blob") => files.push(path.to_string()),
            Some("tree") if !path.contains('/') => {
                top_level_directories.insert(path.to_string());
            }
            _ => {}
        }
    }

    files.sort();

    Ok(TreeListing {
        files,
        top_level_directories: top_level_directories.into_iter().collect(),
        truncated: tree
            .pointer("/truncated")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false),
    })
}

fn get_json(url: &str, token: Option<&str>) -> anyhow::Result<JsonValue> {
    let response = request(url, token, "application/vnd.github+json")
        .call()
        .map_err(map_github_error)?;
    let body = response
        .into_string()
        .context("failed to read GitHub API response body")?;

    serde_json::from_str(&body).context("GitHub API returned invalid JSON")
}

fn get_raw(url: &str, token: Option<&str>) -> anyhow::Result<Option<String>> {
    match request(url, token, "application/vnd.github.raw+json").call() {
        Ok(response) => Ok(Some(
            response
                .into_string()
                .context("failed to read GitHub file content")?,
        )),
        Err(ureq::Error::Status(404, _)) => Ok(None),
        Err(error) => Err(map_github_error(error)),
    }
}

fn request(url: &str, token: Option<&str>, accept: &str) -> ureq::Request {
    let mut request = ureq::get(url)
        .set("User-Agent", "repolens")
        .set("Accept", accept)
        .set("X-GitHub-Api-Version", "2022-11-28");

    if let Some(token) = token {
        request = request.set("Authorization", &format!("Bearer {token}"));
    }

    request
}

fn map_github_error(error: ureq::Error) -> anyhow::Error {
    match error {
        ureq::Error::Status(404, _) => anyhow::anyhow!(
            "GitHub returned 404: the repository does not exist or is private \
(set GITHUB_TOKEN to access private repositories)"
        ),
        ureq::Error::Status(status @ (403 | 429), response) => {
            let rate_limited = response
                .header("x-ratelimit-remaining")
                .is_some_and(|remaining| remaining == "0");
            if rate_limited {
                anyhow::anyhow!(
                    "GitHub API rate limit exceeded (HTTP {status}); \
set GITHUB_TOKEN to raise the limit"
                )
            } else {
                anyhow::anyhow!("GitHub API denied the request (HTTP {status})")
            }
        }
        ureq::Error::Status(status, _) => {
            anyhow::anyhow!("GitHub API request failed (HTTP {status})")
        }
        transport => anyhow::Error::new(transport).context("could not reach the GitHub API"),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parses_default_branch_from_metadata() {
        let metadata = json!({ "name": "repolens", "default_branch": "main" });

        assert_eq!(parse_default_branch(&metadata).expect("branch"), "main");
        assert!(parse_default_branch(&json!({})).is_err());
    }

    #[test]
    fn parses_tree_into_files_and_directories() {
        let tree = json!({
            "truncated": false,
            "tree": [
                { "path": "AGENTS.md", "type": "blob" },
                { "path": "src", "type": "tree" },
                { "path": "src/main.rs", "type": "blob" },
                { "path": "docs", "type": "tree" },
                { "path": "docs/skills", "type": "tree" },
                { "path": "docs/skills/do_work.md", "type": "blob" }
            ]
        });

        let listing = parse_tree(&tree).expect("listing");

        assert_eq!(
            listing.files,
            vec![
                "AGENTS.md".to_string(),
                "docs/skills/do_work.md".to_string(),
                "src/main.rs".to_string(),
            ]
        );
        assert_eq!(
            listing.top_level_directories,
            vec!["docs".to_string(), "src".to_string()]
        );
        assert!(!listing.truncated);
    }

    #[test]
    fn flags_truncated_trees() {
        let tree = json!({ "truncated": true, "tree": [] });

        assert!(parse_tree(&tree).expect("listing").truncated);
    }

    #[test]
    fn rejects_malformed_tree_responses() {
        assert!(parse_tree(&json!({ "message": "Not Found" })).is_err());
    }
}
