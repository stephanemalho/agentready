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
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Multi-harness AI agents"));
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
    fs::create_dir_all(repo.path().join(".github/workflows")).unwrap();
    fs::write(repo.path().join(".github/workflows/ci.yml"), "name: CI\n").unwrap();
    fs::create_dir(repo.path().join("tests")).unwrap();
    fs::write(repo.path().join("tests/cli.rs"), "#[test]\nfn ok() {}\n").unwrap();

    repo
}
