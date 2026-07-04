use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::analyzer::{RepositorySnapshot, RepositorySourceMetadata};

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
    pub source: RepositorySourceMetadata,
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
    pub id: String,
    pub severity: Severity,
    pub status: CheckStatus,
    pub title: String,
    pub message: String,
    pub evidence: Vec<String>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug)]
struct Rule {
    harness: HarnessName,
    id: &'static str,
    severity: Severity,
    title: &'static str,
    remediation: &'static str,
    source: &'static str,
}

const RULE_SHARED_AGENTS_MD: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.agents_md.exists",
    severity: Severity::High,
    title: "Canonical AGENTS.md",
    remediation: "Create AGENTS.md at the repository root and make it the canonical agent entrypoint.",
    source: "https://developers.openai.com/codex/guides/agents-md",
};

const RULE_SHARED_AGENT_RULES: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.agent_rules.readme",
    severity: Severity::Medium,
    title: "Shared agent rules",
    remediation: "Add docs/agent-rules/README.md with the harness-neutral project rules.",
    source: "docs/agent-rules/README.md",
};

const RULE_SHARED_SKILLS: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.skills.present",
    severity: Severity::Low,
    title: "Reusable workflows",
    remediation: "Add reusable workflow documents under docs/skills/.",
    source: "docs/skills/",
};

const RULE_SHARED_PREFLIGHT: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.scripts.preflight",
    severity: Severity::Medium,
    title: "Agent preflight",
    remediation: "Add scripts/agent-preflight.sh so agents can verify branch and sync state before working.",
    source: "AGENTS.md",
};

const RULE_SHARED_SYNC_MAIN: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.scripts.sync_main",
    severity: Severity::Medium,
    title: "Main sync helper",
    remediation: "Add scripts/agent-sync-main.sh so agents can catch up with origin/main deterministically.",
    source: "AGENTS.md",
};

const RULE_SHARED_CI_CONTROL: Rule = Rule {
    harness: HarnessName::Shared,
    id: "shared.ci.agent_control",
    severity: Severity::Medium,
    title: "CI branch policy",
    remediation: "Add .github/workflows/agent-control.yml to enforce branch and template policy in CI.",
    source: ".github/workflows/agent-control.yml",
};

const RULE_CODEX_CONFIG_TOML: Rule = Rule {
    harness: HarnessName::Codex,
    id: "codex.config.valid_toml",
    severity: Severity::High,
    title: "Codex config",
    remediation: "Create .codex/config.toml with valid TOML for Codex project configuration.",
    source: "https://developers.openai.com/codex/config-reference",
};

const RULE_CODEX_CONFIG_AGENTS: Rule = Rule {
    harness: HarnessName::Codex,
    id: "codex.config.references_agents_md",
    severity: Severity::Medium,
    title: "Codex config references AGENTS.md",
    remediation: "Reference AGENTS.md from .codex/config.toml so Codex loads the canonical rules.",
    source: "https://developers.openai.com/codex/guides/agents-md",
};

const RULE_CODEX_SKILLS: Rule = Rule {
    harness: HarnessName::Codex,
    id: "codex.skills.present",
    severity: Severity::Low,
    title: "Codex skills",
    remediation: "Add at least one reusable skill as .agents/skills/<name>/SKILL.md.",
    source: "https://developers.openai.com/codex/skills",
};

const RULE_CLAUDE_ADAPTER: Rule = Rule {
    harness: HarnessName::Claude,
    id: "claude.adapter.exists",
    severity: Severity::High,
    title: "Claude adapter",
    remediation: "Create CLAUDE.md so Claude Code loads the project memory entrypoint.",
    source: "https://code.claude.com/docs/en/memory",
};

const RULE_CLAUDE_IMPORTS_AGENTS: Rule = Rule {
    harness: HarnessName::Claude,
    id: "claude.adapter.imports_agents_md",
    severity: Severity::Medium,
    title: "Claude imports AGENTS.md",
    remediation: "Add an @AGENTS.md import to CLAUDE.md instead of duplicating rules.",
    source: "https://code.claude.com/docs/en/memory",
};

const RULE_CLAUDE_SETTINGS_JSON: Rule = Rule {
    harness: HarnessName::Claude,
    id: "claude.settings.valid_json",
    severity: Severity::Medium,
    title: "Claude settings",
    remediation: "Create .claude/settings.json with valid JSON project settings.",
    source: "https://code.claude.com/docs/en/settings",
};

const RULE_CLAUDE_RULES_README: Rule = Rule {
    harness: HarnessName::Claude,
    id: "claude.rules.readme",
    severity: Severity::Low,
    title: "Claude rules directory",
    remediation: "Add .claude/rules/README.md describing Claude-specific rule loading.",
    source: "https://code.claude.com/docs/en/memory",
};

const RULE_CLAUDE_LOCAL_SETTINGS: Rule = Rule {
    harness: HarnessName::Claude,
    id: "claude.settings.local_untracked",
    severity: Severity::High,
    title: "Claude local settings are untracked",
    remediation: "Remove .claude/settings.local.json from version control and keep it machine-local.",
    source: "https://code.claude.com/docs/en/settings",
};

const RULE_GEMINI_ADAPTER: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.adapter.exists",
    severity: Severity::High,
    title: "Gemini adapter",
    remediation: "Create GEMINI.md so Gemini CLI loads project context.",
    source: "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
};

const RULE_GEMINI_IMPORTS_AGENTS: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.adapter.imports_agents_md",
    severity: Severity::Medium,
    title: "Gemini imports AGENTS.md",
    remediation: "Add an @AGENTS.md import to GEMINI.md instead of duplicating rules.",
    source: "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
};

const RULE_GEMINI_SETTINGS_JSON: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.settings.valid_json",
    severity: Severity::Medium,
    title: "Gemini settings",
    remediation: "Create .gemini/settings.json with valid JSON settings.",
    source: "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
};

const RULE_GEMINI_CONTEXT_AGENTS: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.settings.context_agents",
    severity: Severity::Medium,
    title: "Gemini AGENTS.md context",
    remediation: "Add AGENTS.md to context.fileName in .gemini/settings.json.",
    source: "https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html",
};

const RULE_GEMINI_AGENTS: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.agents.present",
    severity: Severity::Low,
    title: "Gemini subagents",
    remediation: "Add subagent definitions under .gemini/agents/*.md.",
    source: "https://github.com/google-gemini/gemini-cli/blob/main/docs/core/subagents.md",
};

const RULE_GEMINI_COMMANDS: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.commands.present",
    severity: Severity::Low,
    title: "Gemini commands",
    remediation: "Add command definitions under .gemini/commands/.",
    source: "https://geminicli.com/docs/reference/commands/",
};

const RULE_GEMINI_COMMAND_TOML: Rule = Rule {
    harness: HarnessName::Gemini,
    id: "gemini.commands.valid_toml",
    severity: Severity::Medium,
    title: "Gemini command TOML",
    remediation: "Fix the command definition so it parses as valid TOML.",
    source: "https://geminicli.com/docs/reference/commands/",
};

pub fn analyze_harness_readiness(
    snapshot: &RepositorySnapshot,
    filter: HarnessFilter,
) -> HarnessReadinessReport {
    let mut checks = Vec::new();

    add_shared_checks(snapshot, &mut checks);

    match filter {
        HarnessFilter::All => {
            add_codex_checks(snapshot, &mut checks);
            add_claude_checks(snapshot, &mut checks);
            add_gemini_checks(snapshot, &mut checks);
        }
        HarnessFilter::Codex => add_codex_checks(snapshot, &mut checks),
        HarnessFilter::Claude => add_claude_checks(snapshot, &mut checks),
        HarnessFilter::Gemini => add_gemini_checks(snapshot, &mut checks),
    }

    let summary = summarize(&checks);
    let score = score(&summary);

    HarnessReadinessReport {
        root: snapshot.root.clone(),
        source: snapshot.source.clone(),
        score,
        summary,
        checks,
    }
}

pub fn content_paths(files: &[String]) -> Vec<String> {
    const NAMED_FILES: [&str; 6] = [
        "AGENTS.md",
        "CLAUDE.md",
        "GEMINI.md",
        ".codex/config.toml",
        ".claude/settings.json",
        ".gemini/settings.json",
    ];

    let mut paths: Vec<String> = NAMED_FILES
        .iter()
        .map(|file| file.to_string())
        .filter(|file| files.contains(file))
        .collect();

    paths.extend(
        files
            .iter()
            .filter(|file| file.starts_with(".gemini/commands/"))
            .cloned(),
    );

    paths
}

fn add_shared_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        &RULE_SHARED_AGENTS_MD,
        "AGENTS.md",
        "Root AGENTS.md gives agents a predictable project entrypoint.",
    );
    check_exists(
        snapshot,
        checks,
        &RULE_SHARED_AGENT_RULES,
        "docs/agent-rules/README.md",
        "Shared project rules should live outside harness-specific adapters.",
    );
    check_dir(
        snapshot,
        checks,
        &RULE_SHARED_SKILLS,
        "docs/skills",
        "Neutral workflows should be available for every harness.",
    );
    check_exists(
        snapshot,
        checks,
        &RULE_SHARED_PREFLIGHT,
        "scripts/agent-preflight.sh",
        "Agents need a deterministic branch and main-sync check before working.",
    );
    check_exists(
        snapshot,
        checks,
        &RULE_SHARED_SYNC_MAIN,
        "scripts/agent-sync-main.sh",
        "Agents need a repeatable way to catch up with origin/main.",
    );
    check_exists(
        snapshot,
        checks,
        &RULE_SHARED_CI_CONTROL,
        ".github/workflows/agent-control.yml",
        "The repo should enforce branch and template controls before merge.",
    );
}

fn add_codex_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_toml_file(
        snapshot,
        checks,
        &RULE_CODEX_CONFIG_TOML,
        ".codex/config.toml",
        "Codex project configuration should parse as TOML.",
    );
    check_file_contains(
        snapshot,
        checks,
        &RULE_CODEX_CONFIG_AGENTS,
        ".codex/config.toml",
        "AGENTS.md",
    );
    let skill_files = skill_files(snapshot, ".agents/skills");
    if skill_files.is_empty() {
        checks.push(HarnessCheck::warn(
            &RULE_CODEX_SKILLS,
            "No .agents/skills/*/SKILL.md files were found.",
            vec![".agents/skills/".to_string()],
        ));
    } else {
        checks.push(HarnessCheck::pass(
            &RULE_CODEX_SKILLS,
            "Codex skills are available for reusable workflows.",
            skill_files,
        ));
    }
}

fn add_claude_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        &RULE_CLAUDE_ADAPTER,
        "CLAUDE.md",
        "Claude Code should have project memory instructions.",
    );
    check_file_contains(
        snapshot,
        checks,
        &RULE_CLAUDE_IMPORTS_AGENTS,
        "CLAUDE.md",
        "@AGENTS.md",
    );
    check_json_file(
        snapshot,
        checks,
        &RULE_CLAUDE_SETTINGS_JSON,
        ".claude/settings.json",
        "Claude project settings should parse as JSON.",
    );
    check_exists(
        snapshot,
        checks,
        &RULE_CLAUDE_RULES_README,
        ".claude/rules/README.md",
        "Claude rules should be explicit when project-specific Claude behavior is needed.",
    );
    if snapshot.has_file(".claude/settings.local.json") {
        checks.push(HarnessCheck::fail(
            &RULE_CLAUDE_LOCAL_SETTINGS,
            ".claude/settings.local.json should not be committed.",
            vec![".claude/settings.local.json".to_string()],
        ));
    } else {
        checks.push(HarnessCheck::pass(
            &RULE_CLAUDE_LOCAL_SETTINGS,
            "No committed .claude/settings.local.json file was found.",
            vec![".claude/settings.local.example.json".to_string()],
        ));
    }
}

fn add_gemini_checks(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    check_exists(
        snapshot,
        checks,
        &RULE_GEMINI_ADAPTER,
        "GEMINI.md",
        "Gemini CLI should have a project context file.",
    );
    check_file_contains(
        snapshot,
        checks,
        &RULE_GEMINI_IMPORTS_AGENTS,
        "GEMINI.md",
        "@AGENTS.md",
    );
    check_json_file(
        snapshot,
        checks,
        &RULE_GEMINI_SETTINGS_JSON,
        ".gemini/settings.json",
        "Gemini settings should parse as JSON.",
    );
    check_gemini_context(snapshot, checks);
    check_gemini_agents(snapshot, checks);

    let command_files = snapshot.files_under(".gemini/commands");
    if command_files.is_empty() {
        checks.push(HarnessCheck::warn(
            &RULE_GEMINI_COMMANDS,
            "No .gemini/commands files were found.",
            vec![".gemini/commands/".to_string()],
        ));
        return;
    }

    for file in command_files {
        check_toml_file(
            snapshot,
            checks,
            &RULE_GEMINI_COMMAND_TOML,
            &file,
            "Gemini command definitions should parse as TOML.",
        );
    }
}

fn check_gemini_context(snapshot: &RepositorySnapshot, checks: &mut Vec<HarnessCheck>) {
    let Ok(contents) = snapshot.read_file(".gemini/settings.json") else {
        checks.push(HarnessCheck::fail(
            &RULE_GEMINI_CONTEXT_AGENTS,
            ".gemini/settings.json could not be read.",
            vec![".gemini/settings.json".to_string()],
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
            &RULE_GEMINI_CONTEXT_AGENTS,
            "Gemini settings include AGENTS.md as a context file.",
            vec![".gemini/settings.json".to_string()],
        ));
    } else {
        checks.push(HarnessCheck::warn(
            &RULE_GEMINI_CONTEXT_AGENTS,
            "Gemini settings do not explicitly include AGENTS.md in context.fileName.",
            vec![".gemini/settings.json".to_string()],
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
            &RULE_GEMINI_AGENTS,
            "No .gemini/agents/*.md files were found.",
            vec![".gemini/agents/".to_string()],
        ));
    } else {
        checks.push(HarnessCheck::pass(
            &RULE_GEMINI_AGENTS,
            "Gemini subagent definitions are present.",
            agent_files,
        ));
    }
}

fn check_exists(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    rule: &Rule,
    file: &str,
    message: &str,
) {
    if snapshot.has_file(file) {
        checks.push(HarnessCheck::pass(rule, message, vec![file.to_string()]));
    } else {
        checks.push(HarnessCheck::fail(
            rule,
            &format!("Missing required file: {file}"),
            vec![file.to_string()],
        ));
    }
}

fn check_dir(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    rule: &Rule,
    dir: &str,
    message: &str,
) {
    if snapshot.has_dir(dir) {
        checks.push(HarnessCheck::pass(rule, message, vec![dir.to_string()]));
    } else {
        checks.push(HarnessCheck::warn(
            rule,
            &format!("No files were found under {dir}/"),
            vec![dir.to_string()],
        ));
    }
}

fn check_file_contains(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    rule: &Rule,
    file: &str,
    needle: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) if contents.contains(needle) => checks.push(HarnessCheck::pass(
            rule,
            &format!("{file} references {needle}."),
            vec![file.to_string()],
        )),
        Ok(_) => checks.push(HarnessCheck::warn(
            rule,
            &format!("{file} does not reference {needle}."),
            vec![file.to_string()],
        )),
        Err(_) => checks.push(HarnessCheck::fail(
            rule,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
        )),
    }
}

fn check_json_file(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    rule: &Rule,
    file: &str,
    message: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) => match serde_json::from_str::<JsonValue>(&contents) {
            Ok(_) => checks.push(HarnessCheck::pass(rule, message, vec![file.to_string()])),
            Err(error) => checks.push(HarnessCheck::fail(
                rule,
                &format!("{file} is not valid JSON: {error}"),
                vec![file.to_string()],
            )),
        },
        Err(_) => checks.push(HarnessCheck::fail(
            rule,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
        )),
    }
}

fn check_toml_file(
    snapshot: &RepositorySnapshot,
    checks: &mut Vec<HarnessCheck>,
    rule: &Rule,
    file: &str,
    message: &str,
) {
    match snapshot.read_file(file) {
        Ok(contents) => match toml::from_str::<toml::Value>(&contents) {
            Ok(_) => checks.push(HarnessCheck::pass(rule, message, vec![file.to_string()])),
            Err(error) => checks.push(HarnessCheck::fail(
                rule,
                &format!("{file} is not valid TOML: {error}"),
                vec![file.to_string()],
            )),
        },
        Err(_) => checks.push(HarnessCheck::fail(
            rule,
            &format!("Missing or unreadable file: {file}"),
            vec![file.to_string()],
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
    fn pass(rule: &Rule, message: &str, evidence: Vec<String>) -> Self {
        Self::new(rule, CheckStatus::Pass, message, evidence, None)
    }

    fn warn(rule: &Rule, message: &str, evidence: Vec<String>) -> Self {
        Self::new(
            rule,
            CheckStatus::Warn,
            message,
            evidence,
            Some(rule.remediation.to_string()),
        )
    }

    fn fail(rule: &Rule, message: &str, evidence: Vec<String>) -> Self {
        Self::new(
            rule,
            CheckStatus::Fail,
            message,
            evidence,
            Some(rule.remediation.to_string()),
        )
    }

    fn new(
        rule: &Rule,
        status: CheckStatus,
        message: &str,
        evidence: Vec<String>,
        remediation: Option<String>,
    ) -> Self {
        Self {
            harness: rule.harness,
            id: rule.id.to_string(),
            severity: rule.severity,
            status,
            title: rule.title.to_string(),
            message: message.to_string(),
            evidence,
            source: rule.source.to_string(),
            remediation,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;

    use tempfile::tempdir;

    use crate::source::{RepositoryTarget, load_snapshot};

    use super::*;

    fn snapshot_of(path: &std::path::Path) -> RepositorySnapshot {
        load_snapshot(&RepositoryTarget::Local(path.to_path_buf())).expect("snapshot")
    }

    #[test]
    fn selects_content_paths_for_harness_checks() {
        let files = vec![
            "AGENTS.md".to_string(),
            "README.md".to_string(),
            ".codex/config.toml".to_string(),
            ".gemini/commands/agent/preflight.toml".to_string(),
        ];

        let paths = content_paths(&files);

        assert!(paths.contains(&"AGENTS.md".to_string()));
        assert!(paths.contains(&".codex/config.toml".to_string()));
        assert!(paths.contains(&".gemini/commands/agent/preflight.toml".to_string()));
        assert!(!paths.contains(&"README.md".to_string()));
        assert!(!paths.contains(&"CLAUDE.md".to_string()));
    }

    const ALL_RULES: [&Rule; 21] = [
        &RULE_SHARED_AGENTS_MD,
        &RULE_SHARED_AGENT_RULES,
        &RULE_SHARED_SKILLS,
        &RULE_SHARED_PREFLIGHT,
        &RULE_SHARED_SYNC_MAIN,
        &RULE_SHARED_CI_CONTROL,
        &RULE_CODEX_CONFIG_TOML,
        &RULE_CODEX_CONFIG_AGENTS,
        &RULE_CODEX_SKILLS,
        &RULE_CLAUDE_ADAPTER,
        &RULE_CLAUDE_IMPORTS_AGENTS,
        &RULE_CLAUDE_SETTINGS_JSON,
        &RULE_CLAUDE_RULES_README,
        &RULE_CLAUDE_LOCAL_SETTINGS,
        &RULE_GEMINI_ADAPTER,
        &RULE_GEMINI_IMPORTS_AGENTS,
        &RULE_GEMINI_SETTINGS_JSON,
        &RULE_GEMINI_CONTEXT_AGENTS,
        &RULE_GEMINI_AGENTS,
        &RULE_GEMINI_COMMANDS,
        &RULE_GEMINI_COMMAND_TOML,
    ];

    #[test]
    fn reports_ready_multi_harness_project() {
        let repo = tempdir().expect("tempdir");
        write_ready_project(repo.path());

        let report = analyze_harness_readiness(&snapshot_of(repo.path()), HarnessFilter::All);

        assert_eq!(report.source, RepositorySourceMetadata::local());
        assert_eq!(report.summary.failed, 0);
        assert!(report.score >= 90);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.id == "gemini.settings.context_agents")
        );
    }

    #[test]
    fn reports_missing_harness_files() {
        let repo = tempdir().expect("tempdir");
        fs::write(repo.path().join("README.md"), "# Demo\n").unwrap();

        let report = analyze_harness_readiness(&snapshot_of(repo.path()), HarnessFilter::All);

        assert!(report.summary.failed > 0);
        assert!(report.checks.iter().any(
            |check| check.status == CheckStatus::Fail && check.id == "shared.agents_md.exists"
        ));
    }

    #[test]
    fn flags_invalid_and_committed_configuration() {
        let repo = tempdir().expect("tempdir");
        write_ready_project(repo.path());
        fs::write(repo.path().join(".codex/config.toml"), "invalid = [\n").unwrap();
        fs::write(repo.path().join(".claude/settings.json"), "{ invalid\n").unwrap();
        fs::write(repo.path().join(".claude/settings.local.json"), "{}\n").unwrap();
        fs::write(repo.path().join("CLAUDE.md"), "# Claude\n").unwrap();

        let report = analyze_harness_readiness(&snapshot_of(repo.path()), HarnessFilter::All);

        let failed_ids: Vec<&str> = report
            .checks
            .iter()
            .filter(|check| check.status == CheckStatus::Fail)
            .map(|check| check.id.as_str())
            .collect();

        assert!(failed_ids.contains(&"codex.config.valid_toml"));
        assert!(failed_ids.contains(&"claude.settings.valid_json"));
        assert!(failed_ids.contains(&"claude.settings.local_untracked"));
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.status == CheckStatus::Warn
                    && check.id == "claude.adapter.imports_agents_md")
        );
        assert!(report.score < 90);
    }

    #[test]
    fn rule_ids_are_stable_and_unique() {
        let mut seen = BTreeSet::new();

        for rule in ALL_RULES {
            assert!(
                rule.id.split('.').count() >= 2,
                "rule id must be namespaced: {}",
                rule.id
            );
            assert!(
                rule.id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == '.' || c == '_'),
                "rule id must be lowercase dotted: {}",
                rule.id
            );
            assert!(
                !rule.remediation.is_empty(),
                "{} needs remediation",
                rule.id
            );
            assert!(!rule.source.is_empty(), "{} needs a source", rule.id);
            assert!(seen.insert(rule.id), "duplicate rule id: {}", rule.id);
        }
    }

    #[test]
    fn warn_and_fail_checks_carry_remediation() {
        let repo = tempdir().expect("tempdir");
        fs::write(repo.path().join("README.md"), "# Demo\n").unwrap();

        let report = analyze_harness_readiness(&snapshot_of(repo.path()), HarnessFilter::All);

        for check in &report.checks {
            match check.status {
                CheckStatus::Pass => assert!(
                    check.remediation.is_none(),
                    "{} should not carry remediation when passing",
                    check.id
                ),
                CheckStatus::Warn | CheckStatus::Fail => assert!(
                    check
                        .remediation
                        .as_deref()
                        .is_some_and(|text| !text.is_empty()),
                    "{} must carry remediation",
                    check.id
                ),
            }
        }
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
