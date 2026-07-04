use anyhow::Context;

use crate::analyzer::{HealthChecks, RepoAnalysis};

pub fn render_markdown(analysis: &RepoAnalysis) -> String {
    let mut output = String::new();

    output.push_str("# RepoLens Report\n\n");
    output.push_str(&format!("- Root: `{}`\n", analysis.root));
    output.push_str(&format!("- Files scanned: `{}`\n", analysis.file_count));

    output.push_str("\n## Detected Stack\n\n");
    if analysis.detected_stacks.is_empty() {
        output.push_str("- No known stack markers detected.\n");
    } else {
        for stack in &analysis.detected_stacks {
            output.push_str(&format!(
                "- **{}**: `{}`\n",
                stack.name,
                stack.evidence.join("`, `")
            ));
        }
    }

    output.push_str("\n## Repository Health\n\n");
    output.push_str(&format_health("README", analysis.health.readme));
    output.push_str(&format_health(".gitignore", analysis.health.gitignore));
    output.push_str(&format_health("CI workflow", analysis.health.ci));
    output.push_str(&format_health("License", analysis.health.license));
    output.push_str(&format_health("Tests", analysis.health.tests));

    output.push_str("\n## Agent Context\n\n");
    output.push_str(
        "Use this report as a compact orientation snapshot before editing the repository. \
It is generated from local files only and does not call external AI services.\n",
    );

    output
}

pub fn render_json(analysis: &RepoAnalysis) -> anyhow::Result<String> {
    serde_json::to_string_pretty(analysis).context("failed to serialize analysis as JSON")
}

pub fn render_doctor(analysis: &RepoAnalysis) -> String {
    let mut output = String::new();

    output.push_str("RepoLens doctor\n");
    output.push_str(&format!("root: {}\n", analysis.root));
    output.push_str(&format!("files: {}\n\n", analysis.file_count));
    output.push_str(&format_doctor_line("README", analysis.health.readme));
    output.push_str(&format_doctor_line(".gitignore", analysis.health.gitignore));
    output.push_str(&format_doctor_line("CI workflow", analysis.health.ci));
    output.push_str(&format_doctor_line("License", analysis.health.license));
    output.push_str(&format_doctor_line("Tests", analysis.health.tests));

    output
}

fn format_health(label: &str, passed: bool) -> String {
    let marker = if passed { "x" } else { " " };
    format!("- [{marker}] {label}\n")
}

fn format_doctor_line(label: &str, passed: bool) -> String {
    let status = if passed { "ok" } else { "warn" };
    format!("[{status}] {label}\n")
}

#[allow(dead_code)]
fn _assert_health_checks_are_used(_: &HealthChecks) {}

#[cfg(test)]
mod tests {
    use crate::analyzer::{DetectedStack, HealthChecks, RepoAnalysis};

    use super::*;

    #[test]
    fn renders_markdown_summary() {
        let analysis = RepoAnalysis {
            root: "/tmp/demo".to_string(),
            file_count: 2,
            top_level_directories: vec!["src".to_string()],
            detected_stacks: vec![DetectedStack {
                name: "Rust".to_string(),
                evidence: vec!["Cargo.toml".to_string()],
            }],
            health: HealthChecks {
                readme: true,
                gitignore: true,
                ci: false,
                license: false,
                tests: true,
            },
        };

        let markdown = render_markdown(&analysis);

        assert!(markdown.contains("# RepoLens Report"));
        assert!(markdown.contains("**Rust**"));
        assert!(markdown.contains("- [x] README"));
        assert!(markdown.contains("- [ ] CI workflow"));
    }
}
