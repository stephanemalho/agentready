use anyhow::Context;

use crate::analyzer::{HealthChecks, RepoAnalysis};
use crate::harness::{CheckStatus, HarnessName, HarnessReadinessReport};

pub fn render_markdown(analysis: &RepoAnalysis) -> String {
    let mut output = String::new();

    output.push_str("# RepoLens Report\n\n");
    output.push_str(&format!("- Root: `{}`\n", analysis.root));
    output.push_str(&format!("- Files scanned: `{}`\n", analysis.file_count));

    output.push_str("\n## Top-Level Directories\n\n");
    if analysis.top_level_directories.is_empty() {
        output.push_str("- No top-level directories detected.\n");
    } else {
        for directory in &analysis.top_level_directories {
            output.push_str(&format!("- `{directory}/`\n"));
        }
    }

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

    output.push_str("\n## Orientation Notes\n\n");
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

pub fn render_harness_markdown(report: &HarnessReadinessReport) -> String {
    let mut output = String::new();

    output.push_str("# RepoLens Harness Readiness\n\n");
    output.push_str(&format!("- Root: `{}`\n", report.root));
    output.push_str(&format!("- Score: `{}/100`\n", report.score));
    output.push_str(&format!(
        "- Checks: `{}` passed, `{}` warnings, `{}` failed\n",
        report.summary.passed, report.summary.warnings, report.summary.failed
    ));

    for harness in [
        HarnessName::Shared,
        HarnessName::Codex,
        HarnessName::Claude,
        HarnessName::Gemini,
    ] {
        let checks: Vec<_> = report
            .checks
            .iter()
            .filter(|check| check.harness == harness)
            .collect();

        if checks.is_empty() {
            continue;
        }

        output.push_str(&format!("\n## {}\n\n", harness.label()));
        for check in checks {
            output.push_str(&format!(
                "- {} **{}**: {}\n",
                status_marker(check.status),
                check.title,
                check.message
            ));

            if !check.evidence.is_empty() {
                output.push_str(&format!("  Evidence: `{}`\n", check.evidence.join("`, `")));
            }
        }
    }

    output.push_str("\n## Notes\n\n");
    output.push_str(
        "Harness readiness checks validate project files and configuration only. \
RepoLens does not run, call, or embed any AI model.\n",
    );

    output
}

pub fn render_harness_json(report: &HarnessReadinessReport) -> anyhow::Result<String> {
    serde_json::to_string_pretty(report).context("failed to serialize harness report as JSON")
}

fn format_health(label: &str, passed: bool) -> String {
    let marker = if passed { "x" } else { " " };
    format!("- [{marker}] {label}\n")
}

fn format_doctor_line(label: &str, passed: bool) -> String {
    let status = if passed { "ok" } else { "warn" };
    format!("[{status}] {label}\n")
}

fn status_marker(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "[x]",
        CheckStatus::Warn => "[!]",
        CheckStatus::Fail => "[ ]",
    }
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
        assert!(markdown.contains("## Top-Level Directories"));
        assert!(markdown.contains("- `src/`"));
        assert!(markdown.contains("**Rust**"));
        assert!(markdown.contains("- [x] README"));
        assert!(markdown.contains("- [ ] CI workflow"));
    }

    #[test]
    fn renders_harness_readiness() {
        let report = HarnessReadinessReport {
            root: "/tmp/demo".to_string(),
            score: 75,
            summary: crate::harness::HarnessSummary {
                passed: 1,
                warnings: 1,
                failed: 0,
            },
            checks: vec![
                crate::harness::HarnessCheck {
                    harness: HarnessName::Shared,
                    status: CheckStatus::Pass,
                    title: "Canonical AGENTS.md".to_string(),
                    message: "AGENTS.md found.".to_string(),
                    evidence: vec!["AGENTS.md".to_string()],
                    source: "docs".to_string(),
                },
                crate::harness::HarnessCheck {
                    harness: HarnessName::Codex,
                    status: CheckStatus::Warn,
                    title: "Codex skills".to_string(),
                    message: "No skills found.".to_string(),
                    evidence: vec![".agents/skills/".to_string()],
                    source: "docs".to_string(),
                },
            ],
        };

        let markdown = render_harness_markdown(&report);

        assert!(markdown.contains("# RepoLens Harness Readiness"));
        assert!(markdown.contains("Score: `75/100`"));
        assert!(markdown.contains("## Codex"));
        assert!(markdown.contains("[!] **Codex skills**"));
    }
}
