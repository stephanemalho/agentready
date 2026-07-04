use std::path::PathBuf;

use anyhow::bail;

use crate::analyzer::RepositorySnapshot;

mod github;
mod local;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryTarget {
    Local(PathBuf),
    GitHub { owner: String, repo: String },
}

impl RepositoryTarget {
    pub fn describe(&self) -> String {
        match self {
            Self::Local(path) => path.display().to_string(),
            Self::GitHub { owner, repo } => format!("github:{owner}/{repo}"),
        }
    }
}

pub fn parse_target(raw: &str) -> anyhow::Result<RepositoryTarget> {
    if let Some(slug) = raw.strip_prefix("github:") {
        return parse_github_slug(slug);
    }

    if let Some(rest) = raw
        .strip_prefix("https://github.com/")
        .or_else(|| raw.strip_prefix("http://github.com/"))
    {
        return parse_github_slug(rest.trim_end_matches('/'));
    }

    Ok(RepositoryTarget::Local(PathBuf::from(raw)))
}

pub fn load_snapshot(target: &RepositoryTarget) -> anyhow::Result<RepositorySnapshot> {
    match target {
        RepositoryTarget::Local(path) => local::snapshot(path),
        RepositoryTarget::GitHub { owner, repo } => github::snapshot(owner, repo),
    }
}

fn parse_github_slug(slug: &str) -> anyhow::Result<RepositoryTarget> {
    let mut parts = slug.split('/').filter(|part| !part.is_empty());
    let owner = parts.next();
    let repo = parts.next().map(|repo| repo.trim_end_matches(".git"));
    let extra = parts.next();

    match (owner, repo, extra) {
        (Some(owner), Some(repo), None)
            if is_valid_github_component(owner) && is_valid_github_component(repo) =>
        {
            Ok(RepositoryTarget::GitHub {
                owner: owner.to_string(),
                repo: repo.to_string(),
            })
        }
        _ => bail!("invalid GitHub target '{slug}'; expected github:owner/repo"),
    }
}

fn is_valid_github_component(component: &str) -> bool {
    !component.is_empty()
        && component
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_local_paths_by_default() {
        let target = parse_target(".").expect("target");

        assert_eq!(target, RepositoryTarget::Local(PathBuf::from(".")));
        assert_eq!(
            parse_target("/tmp/demo").expect("target"),
            RepositoryTarget::Local(PathBuf::from("/tmp/demo"))
        );
    }

    #[test]
    fn parses_github_slug_targets() {
        let target = parse_target("github:owner/repo").expect("target");

        assert_eq!(
            target,
            RepositoryTarget::GitHub {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
            }
        );
        assert_eq!(target.describe(), "github:owner/repo");
    }

    #[test]
    fn parses_github_url_targets() {
        for raw in [
            "https://github.com/owner/repo",
            "https://github.com/owner/repo/",
            "https://github.com/owner/repo.git",
            "http://github.com/owner/repo",
        ] {
            let target = parse_target(raw).expect("target");
            assert_eq!(
                target,
                RepositoryTarget::GitHub {
                    owner: "owner".to_string(),
                    repo: "repo".to_string(),
                },
                "failed for {raw}"
            );
        }
    }

    #[test]
    fn rejects_invalid_github_targets() {
        assert!(parse_target("github:owner").is_err());
        assert!(parse_target("github:owner/repo/extra").is_err());
        assert!(parse_target("github:owner/re po").is_err());
        assert!(parse_target("https://github.com/owner").is_err());
    }
}
