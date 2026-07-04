use std::fs;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn scan_outputs_markdown_report() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command.arg("scan").arg(repo.path());

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("# RepoLens Report"))
        .stdout(predicate::str::contains("## Top-Level Directories"))
        .stdout(predicate::str::contains("- `docs/`"))
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Multi-harness coding agents"));
}

#[test]
fn scan_writes_report_to_file() {
    let repo = fixture_repo();
    let output = repo.path().join("repolens-report.md");

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command
        .arg("scan")
        .arg(repo.path())
        .arg("--output")
        .arg(&output);

    command.assert().success();

    let content = fs::read_to_string(&output).expect("report file");
    assert!(content.contains("# RepoLens Report"));
}

#[test]
fn scan_fails_for_missing_path() {
    let mut command = Command::cargo_bin("repolens").expect("binary");
    command.arg("scan").arg("/nonexistent/repolens-test-path");

    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("repository path does not exist"));
}

#[test]
fn scan_outputs_json_report() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command
        .arg("scan")
        .arg(repo.path())
        .arg("--format")
        .arg("json");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("\"file_count\""))
        .stdout(predicate::str::contains("\"Rust\""));
}

#[test]
fn doctor_outputs_health_summary() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command.arg("doctor").arg(repo.path());

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("RepoLens doctor"))
        .stdout(predicate::str::contains("[ok] README"))
        .stdout(predicate::str::contains("[warn] License"));
}

#[test]
fn harness_outputs_readiness_report() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command.arg("harness").arg(repo.path());

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("# RepoLens Harness Readiness"))
        .stdout(predicate::str::contains("Shared Workflow"))
        .stdout(predicate::str::contains("Codex"))
        .stdout(predicate::str::contains("Gemini AGENTS.md context"));
}

#[test]
fn harness_outputs_json_report() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command
        .arg("harness")
        .arg(repo.path())
        .arg("--format")
        .arg("json");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("\"score\""))
        .stdout(predicate::str::contains("\"harness\""))
        .stdout(predicate::str::contains("\"codex\""));
}

#[test]
fn harness_writes_json_report_to_file() {
    let repo = fixture_repo();
    let output = repo.path().join("harness-report.json");

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command
        .arg("harness")
        .arg(repo.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg(&output);

    command.assert().success();

    let content = fs::read_to_string(&output).expect("report file");
    assert!(content.contains("\"score\""));
    assert!(content.contains("\"checks\""));
}

#[test]
fn harness_can_filter_to_codex() {
    let repo = fixture_repo();

    let mut command = Command::cargo_bin("repolens").expect("binary");
    command
        .arg("harness")
        .arg(repo.path())
        .arg("--harness")
        .arg("codex");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("## Codex"))
        .stdout(predicate::str::contains("## Claude Code").not())
        .stdout(predicate::str::contains("## Gemini CLI").not());
}

fn fixture_repo() -> tempfile::TempDir {
    let repo = tempdir().expect("tempdir");
    fs::write(
        repo.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\n",
    )
    .unwrap();
    fs::write(repo.path().join("README.md"), "# Demo\n").unwrap();
    fs::write(repo.path().join(".gitignore"), "/target/\n").unwrap();
    fs::write(repo.path().join("AGENTS.md"), "# Agents\n").unwrap();
    fs::write(repo.path().join("CLAUDE.md"), "@AGENTS.md\n").unwrap();
    fs::write(repo.path().join("GEMINI.md"), "@AGENTS.md\n").unwrap();
    fs::create_dir_all(repo.path().join(".codex")).unwrap();
    fs::write(
        repo.path().join(".codex/config.toml"),
        "developer_instructions = \"Read AGENTS.md\"\n",
    )
    .unwrap();
    fs::create_dir_all(repo.path().join(".agents/skills/do-work")).unwrap();
    fs::write(
        repo.path().join(".agents/skills/do-work/SKILL.md"),
        "# Skill\n",
    )
    .unwrap();
    fs::create_dir_all(repo.path().join(".claude/rules")).unwrap();
    fs::write(
        repo.path().join(".claude/rules/README.md"),
        "# Claude rules\n",
    )
    .unwrap();
    fs::write(repo.path().join(".claude/settings.json"), "{}\n").unwrap();
    fs::create_dir_all(repo.path().join(".gemini/agents")).unwrap();
    fs::write(
        repo.path().join(".gemini/agents/reviewer.md"),
        "# Reviewer\n",
    )
    .unwrap();
    fs::create_dir_all(repo.path().join(".gemini/commands/agent")).unwrap();
    fs::write(
        repo.path().join(".gemini/commands/agent/preflight.toml"),
        "prompt = \"Run preflight\"\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".gemini/settings.json"),
        "{ \"context\": { \"fileName\": [\"AGENTS.md\"] } }\n",
    )
    .unwrap();
    fs::create_dir_all(repo.path().join("docs/agent-rules")).unwrap();
    fs::write(repo.path().join("docs/agent-rules/README.md"), "# Rules\n").unwrap();
    fs::create_dir_all(repo.path().join("docs/skills")).unwrap();
    fs::write(repo.path().join("docs/skills/do_work.md"), "# Workflow\n").unwrap();
    fs::create_dir(repo.path().join("scripts")).unwrap();
    fs::write(
        repo.path().join("scripts/agent-preflight.sh"),
        "#!/usr/bin/env bash\n",
    )
    .unwrap();
    fs::write(
        repo.path().join("scripts/agent-sync-main.sh"),
        "#!/usr/bin/env bash\n",
    )
    .unwrap();
    fs::create_dir_all(repo.path().join(".github/workflows")).unwrap();
    fs::write(repo.path().join(".github/workflows/ci.yml"), "name: CI\n").unwrap();
    fs::write(
        repo.path().join(".github/workflows/agent-control.yml"),
        "name: Agent\n",
    )
    .unwrap();
    fs::create_dir(repo.path().join("tests")).unwrap();
    fs::write(repo.path().join("tests/cli.rs"), "#[test]\nfn ok() {}\n").unwrap();

    repo
}
