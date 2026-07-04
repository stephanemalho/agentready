use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::analyzer::{RepositorySnapshot, snapshot_repository};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HarnessFilter {
    All,
    Codex,
    Claude,
    Gemini,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HarnessReadinessReport {
    pub root: String,
    pub score: u8,
    pub summary: HarnessSummary,
    pub checks: Vec<HarnessCheck>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HarnessSummary {
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HarnessCheck {
    pub harness: HarnessName,
    pub status: CheckStatus,
    pub title: String,
    pub message: String,
    pub evidence: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HarnessName {
    Shared,
    Codex,
    Claude,
    Gemini,
}

impl HarnessName {
    pub fn label(self) -> &'static str {
        match self {
            Self::Shared => "Shared Workflow",
            Self::Codex => "Codex",
            Self::Claude => "Claude Code",
            Self::Gemini => "Gemini CLI",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

pub fn analyze_harness_readiness(
    path: impl AsRef<std::path::Path>,
    filter: HarnessFilter,
) -> anyhow::Result<HarnessReadinessReport> {
    let snapshot = snapshot_repository(path)?;
    let mut checks = Vec::new();

    add_shared_checks(&snapshot, &mut checks);

    match filter {
        HarnessFilter::All => {
            add_codex_checks(&snapshot, &mut checks);
            add_claude_checks(&snapshot, &mut checks);
            add_gemini_checks(&snapshot, &mut checks);
        }
        HarnessFilter::Codex => add_codex_checks(&snapshot, &mut checks),
        HarnessFilter::Claude => add_claude_checks(&snapshot, &mut checks),
        HarnessFilter::Gemini => add_gemini_checks(&snapshot, &mut checks),
    }

    let summary = summarize(&checks);
    let score = score(&summary);

    Ok(HarnessReadinessReport {
        root: snapshot.root_display(),
        score,
        summary,
        checks,
    })
}

fn add_shared_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        HarnessName::Shared,
        "Canonical AGENTS.md",
        "AGENTS.md",
        "Root AGENTS.md gives agents a predictable project entrypoint.",
        "https://developers.openai.com/codex/guides/agents-md",
    );
    check_exists(
        snapshot,
        checks,
        HarnessName::Shared,
        "Shared agent rules",
        "docs/agent-rules/README.md",
        "Shared project rules should live outside harness-specific adapters.",
        "docs/agent-rules/README.md",
    );
    check_dir(
        snapshot,
        checks,
        HarnessName::Shared,
        "Reusable workflows",
        "docs/skills",
        "Neutral workflows should be available for every harness.",
        "docs/skills/",
    );
    check_exists(
        snapshot,
        checks,
        HarnessName::Shared,
        "Agent preflight",
        "scripts/agent-preflight.sh",
        "Agents need a deterministic branch and main-sync check before working.",
        "AGENTS.md",
    );
    check_exists(
        snapshot,
        checks,
        HarnessName::Shared,
        "Main sync helper",
        "scripts/agent-sync-main.sh",
        "Agents need a repeatable way to catch up with origin/main.",
        "AGENTS.md",
    );
    check_exists(
        snapshot,
        checks,
        HarnessName::Shared,
        "CI branch policy",
        ".github/workflows/agent-control.yml",
        "The repo should enforce branch and template controls before merge.",
        ".github/workflows/agent-control.yml",
    );
}

fn add_codex_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_toml_file(
        snapshot,
        checks,
        HarnessName::Codex,
        "Codex config",
        ".codex/config.toml",
        "Codex project configuration should parse as TOML.",
        "https://developers.openai.com/codex/config-reference",
    );
    check_file_contains(
        snapshot,
        checks,
        HarnessName::Codex,
        "Codex config references AGENTS.md",
        ".codex/config.toml",
        "AGENTS.md",
        "https://developers.openai.com/codex/guides/agents-md",
    );
    let skill_files = skill_files(snapshot, ".agents/skills");
    if skill_files.is_empty() {
        checks.push(HarnessCheck::warn(
            HarnessName::Codex,
            "Codex skills",
            "No .agents/skills/*/SKILL.md files were found.",
            vec![".agents/skills/".to_string()],
            "https://developers.openai.com/codex/skills",
        ));
    } else {
        checks.push(HarnessCheck::pass(
            HarnessName::Codex,
            "Codex skills",
            "Codex skills are available for reusable workflows.",
            skill_files,
            "https://developers.openai.com/codex/skills",
        ));
    }
}

fn add_claude_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        HarnessName::Claude,
        "Claude adapter",
        "CLAUDE.md",
        "Claude Code should have project memory instructions.",
        "https://code.claude.com/docs/en/memory",
    );
    check_file_contains(
        snapshot,
        checks,
        HarnessName::Claude,
        "Claude imports AGENTS.md",
        "CLAUDE.md",
        "@AGENTS.md",
        "https://code.claude.com/docs/en/memory",
    );
    check_json_file(
        snapshot,
        checks,
        HarnessName::Claude,
        "Claude settings",
        ".claude/settings.json",
        "Claude project settings should parse as JSON.",
        "https://code.claude.com/docs/en/settings",
    );
    check_exists(
        snapshot,
        checks,
        HarnessName::Claude,
        "Claude rules directory",
        ".claude/rules/README.md",
        "Claude rules should be explicit when project-specific Claude behavior is needed.",
        "https://code.claude.com/docs/en/memory",
    );
    if snapshot.has_file(".claude/settings.local.json") {
        checks.push(HarnessCheck::fail(
            HarnessName::Claude,
            "Claude local settings are untracked",
            ".claude/settings.local.json should not be committed.",
            vec![".claude/settings.local.json".to_string()],
            "https://code.claude.com/docs/en/settings",
        ));
    } else {
        checks.push(HarnessCheck::pass(
            HarnessName::Claude,
            "Claude local settings are untracked",
            "No committed .claude/settings.local.json file was found.",
            vec![".claude/settings.local.example.json".to_string()],
            "https://code.claude.com/docs/en/settings",
        ));
    }
}

fn add_gemini_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        HarnessName::Gemini,
        "Gemini adapter",
        "GEMINI.md",
        "Gemini CLI should have a project context file.",
        "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
    );
    check_file_contains(
        snapshot,
        checks,
        HarnessName::Gemini,
        "Gemini imports AGENTS.md",
        "GEMINI.md",
        "@AGENTS.md",
        "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
    );
    check_json_file(
        snapshot,
        checks,
        HarnessName::Gemini,
        "Gemini settings",
        ".gemini/settings.json",
        "Gemini settings should parse as JSON.",
        "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
    );
    check_gemini_context(snapshot, checks);
    check_gemini_agents(snapshot, checks);

    let command_files = snapshot.files_under(".gemini/commands");
    if command_files.is_empty() {
        checks.push(HarnessCheck::warn(
            HarnessName::Gemini,
            "Gemini commands",
            "No .gemini/commands files were found.",
            vec![".gemini/commands/".to_string()],
            "https://geminicli.com/docs/reference/commands/",
        ));
        return;
    }

    for file in command_files {
        check_toml_file(
            snapshot,
            checks,
            HarnessName::Gemini,
            "Gemini command TOML",
            &file,
            "Gemini command definitions should parse as TOML.",
            "https://geminicli.com/docs/reference/commands/",
        );
    }
}

fn check_gemini_context(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    let Ok(contents) = snapshot.read_file(".gemini/settings.json") else {
        checks.push(HarnessCheck::fail(
            HarnessName::Gemini,
            "Gemini AGENTS.md context",
            ".gemini/settings.json could not be read.",
            vec![".gemini/settings.json".to_string()],
            "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
        ));
        return;
    };

    let Ok(json) = serde_json::from_str::<JsonValue>(&contents) else {
        return;
    };

    let includes_agents = json
        .pointer("/context/fileName")
        .and_then(JsonValue::as_array)
        .is_some_and(|values| values.iter().any(|value| value == "AGENTS.md"));

    if includes_agents {
        checks.push(HarnessCheck::pass(
            HarnessName::Gemini,
            "Gemini AGENTS.md context",
            "Gemini settings include AGENTS.md as a context file.",
            vec![".gemini/settings.json".to_string()],
            "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
        ));
    } else {
        checks.push(HarnessCheck::warn(
            HarnessName::Gemini,
            "Gemini AGENTS.md context",
            "Gemini settings do not explicitly include AGENTS.md in context.fileName.",
            vec![".gemini/settings.json".to_string()],
            "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
        ));
    }
}

fn check_gemini_agents(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    let agent_files: Vec<String> = snapshot
        .files_under(".gemini/agents")
        .into_iter()
        .filter(|file| file.ends_with(".md"))
        .collect();

    if agent_files.is_empty() {
        checks.push(HarnessCheck::warn(
            HarnessName::Gemini,
            "Gemini subagents",
            "No .gemini/agents/*.md files were found.",
            vec![".gemini/agents/".to_string()],
            "https://github.com/google-gemini/gemini-cli/blob/main/docs/core/subagents.md",
        ));
    } else {
        checks.push(HarnessCheck::pass(
            HarnessName::Gemini,
            "Gemini subagents",
            "Gemini subagent definitions are present.",
            agent_files,
            "https://github.com/google-gemini/gemini-cli/blob/main/docs/core/subagents.md",
        ));
    }
}

fn check_exists(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    harness: HarnessName,
    title: &str,
    file: &str,
    message: &str,
    source: &str,
) {
    if snapshot.has_file(file) {
        checks.push(HarnessCheck::pass(
            harness,
            title,
            message,
            vec![file.to_string()],
            source,
        ));
    } else {
        checks.push(HarnessCheck::fail(
            harness,
            title,
            &format!("Missing required file: {file}"),
            vec![file.to_string()],
            source,
        ));
    }
}

fn check_dir(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    harness: HarnessName,
    title: &str,
    dir: &str,
    message: &str,
    source: &str,
) {
    if snapshot.has_dir(dir) {
        checks.push(HarnessCheck::pass(
            harness,
            title,
            message,
            vec![dir.to_string()],
            source,
        ));
    } else {
        checks.push(HarnessCheck::warn(
            harness,
            title,
            &format!("No files were found under {dir}/"),
            vec![dir.to_string()],
            source,
        ));
    }
}

fn check_file_contains(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    harness: HarnessName,
    title: &str,
    file: &str,
    needle: &str,
    source: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) if contents.contains(needle) => checks.push(HarnessCheck::pass(
            harness,
            title,
            &format!("{file} references {needle}."),
            vec![file.to_string()],
            source,
        )),
        Ok(_) => checks.push(HarnessCheck::warn(
            harness,
            title,
            &format!("{file} does not reference {needle}."),
            vec![file.to_string()],
            source,
        )),
        Err(_) => checks.push(HarnessCheck::fail(
            harness,
            title,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
            source,
        )),
    }
}

fn check_json_file(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    harness: HarnessName,
    title: &str,
    file: &str,
    message: &str,
    source: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) => match serde_json::from_str::<JsonValue>(&contents) {
            Ok(_) => checks.push(HarnessCheck::pass(
                harness,
                title,
                message,
                vec![file.to_string()],
                source,
            )),
            Err(error) => checks.push(HarnessCheck::fail(
                harness,
                title,
                &format!("{file} is not valid JSON: {error}"),
                vec![file.to_string()],
                source,
            )),
        },
        Err(_) => checks.push(HarnessCheck::fail(
            harness,
            title,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
            source,
        )),
    }
}

fn check_toml_file(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    harness: HarnessName,
    title: &str,
    file: &str,
    message: &str,
    source: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) => match toml::from_str::<toml::Value>(&contents) {
            Ok(_) => checks.push(HarnessCheck::pass(
                harness,
                title,
                message,
                vec![file.to_string()],
                source,
            )),
            Err(error) => checks.push(HarnessCheck::fail(
                harness,
                title,
                &format!("{file} is not valid TOML: {error}"),
                vec![file.to_string()],
                source,
            )),
        },
        Err(_) => checks.push(HarnessCheck::fail(
            harness,
            title,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
            source,
        )),
    }
}

fn skill_files(snapshot: &RepositorySnapshot, dir: &str) -> Vec<String> {
    snapshot
        .files_under(dir)
        .into_iter()
        .filter(|file| file.ends_with("/SKILL.md"))
        .collect()
}

fn summarize(checks: &[HarnessCheck]) -> HarnessSummary {
    HarnessSummary {
        passed: checks
            .iter()
            .filter(|check| check.status == CheckStatus::Pass)
            .count(),
        warnings: checks
            .iter()
            .filter(|check| check.status == CheckStatus::Warn)
            .count(),
        failed: checks
            .iter()
            .filter(|check| check.status == CheckStatus::Fail)
            .count(),
    }
}

fn score(summary: &HarnessSummary) -> u8 {
    let total = summary.passed + summary.warnings + summary.failed;
    if total == 0 {
        return 0;
    }

    let weighted = (summary.passed as f32) + (summary.warnings as f32 * 0.5);
    ((weighted / total as f32) * 100.0).round() as u8
}

impl HarnessCheck {
    fn pass(
        harness: HarnessName,
        title: &str,
        message: &str,
        evidence: Vec<String>,
        source: &str,
    ) -> Self {
        Self::new(harness, CheckStatus::Pass, title, message, evidence, source)
    }

    fn warn(
        harness: HarnessName,
        title: &str,
        message: &str,
        evidence: Vec<String>,
        source: &str,
    ) -> Self {
        Self::new(harness, CheckStatus::Warn, title, message, evidence, source)
    }

    fn fail(
        harness: HarnessName,
        title: &str,
        message: &str,
        evidence: Vec<String>,
        source: &str,
    ) -> Self {
        Self::new(harness, CheckStatus::Fail, title, message, evidence, source)
    }

    fn new(
        harness: HarnessName,
        status: CheckStatus,
        title: &str,
        message: &str,
        evidence: Vec<String>,
        source: &str,
    ) -> Self {
        Self {
            harness,
            status,
            title: title.to_string(),
            message: message.to_string(),
            evidence,
            source: source.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn reports_ready_multi_harness_project() {
        let repo = tempdir().expect("tempdir");
        write_ready_project(repo.path());

        let report = analyze_harness_readiness(repo.path(), HarnessFilter::All).expect("report");

        assert_eq!(report.summary.failed, 0);
        assert!(report.score >= 90);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.title == "Gemini AGENTS.md context")
        );
    }

    #[test]
    fn reports_missing_harness_files() {
        let repo = tempdir().expect("tempdir");
        fs::write(repo.path().join("README.md"), "# Demo\n").unwrap();

        let report = analyze_harness_readiness(repo.path(), HarnessFilter::All).expect("report");

        assert!(report.summary.failed > 0);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.status == CheckStatus::Fail
                    && check.title == "Canonical AGENTS.md")
        );
    }

    #[test]
    fn flags_invalid_and_committed_configuration() {
        let repo = tempdir().expect("tempdir");
        write_ready_project(repo.path());
        fs::write(repo.path().join(".codex/config.toml"), "invalid = [\n").unwrap();
        fs::write(repo.path().join(".claude/settings.json"), "{ invalid\n").unwrap();
        fs::write(repo.path().join(".claude/settings.local.json"), "{}\n").unwrap();
        fs::write(repo.path().join("CLAUDE.md"), "# Claude\n").unwrap();

        let report = analyze_harness_readiness(repo.path(), HarnessFilter::All).expect("report");

        let failed_titles: Vec<&str> = report
            .checks
            .iter()
            .filter(|check| check.status == CheckStatus::Fail)
            .map(|check| check.title.as_str())
            .collect();

        assert!(failed_titles.contains(&"Codex config"));
        assert!(failed_titles.contains(&"Claude settings"));
        assert!(failed_titles.contains(&"Claude local settings are untracked"));
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.status == CheckStatus::Warn
                    && check.title == "Claude imports AGENTS.md")
        );
        assert!(report.score < 90);
    }

    fn write_ready_project(root: &std::path::Path) {
        fs::write(root.join("AGENTS.md"), "# Agents\n").unwrap();
        fs::write(root.join("CLAUDE.md"), "@AGENTS.md\n").unwrap();
        fs::write(root.join("GEMINI.md"), "@AGENTS.md\n").unwrap();
        fs::create_dir_all(root.join("docs/agent-rules")).unwrap();
        fs::write(root.join("docs/agent-rules/README.md"), "# Rules\n").unwrap();
        fs::create_dir_all(root.join("docs/skills/do_work")).unwrap();
        fs::write(root.join("docs/skills/do_work.md"), "# Workflow\n").unwrap();
        fs::create_dir(root.join("scripts")).unwrap();
        fs::write(
            root.join("scripts/agent-preflight.sh"),
            "#!/usr/bin/env bash\n",
        )
        .unwrap();
        fs::write(
            root.join("scripts/agent-sync-main.sh"),
            "#!/usr/bin/env bash\n",
        )
        .unwrap();
        fs::create_dir_all(root.join(".github/workflows")).unwrap();
        fs::write(
            root.join(".github/workflows/agent-control.yml"),
            "name: Agent\n",
        )
        .unwrap();
        fs::create_dir(root.join(".codex")).unwrap();
        fs::write(
            root.join(".codex/config.toml"),
            "project_doc_fallback_filenames = [\"CLAUDE.md\"]\ndeveloper_instructions = \"Read AGENTS.md\"\n",
        )
        .unwrap();
        fs::create_dir_all(root.join(".agents/skills/do-work")).unwrap();
        fs::write(root.join(".agents/skills/do-work/SKILL.md"), "# Skill\n").unwrap();
        fs::create_dir_all(root.join(".claude/rules")).unwrap();
        fs::write(root.join(".claude/rules/README.md"), "# Claude rules\n").unwrap();
        fs::write(root.join(".claude/settings.json"), "{}\n").unwrap();
        fs::create_dir_all(root.join(".gemini/agents")).unwrap();
        fs::write(root.join(".gemini/agents/reviewer.md"), "# Reviewer\n").unwrap();
        fs::create_dir_all(root.join(".gemini/commands/agent")).unwrap();
        fs::write(
            root.join(".gemini/commands/agent/preflight.toml"),
            "prompt = \"Run preflight\"\n",
        )
        .unwrap();
        fs::write(
            root.join(".gemini/settings.json"),
            "{ \"context\": { \"fileName\": [\"AGENTS.md\"] } }\n",
        )
        .unwrap();
    }
}
