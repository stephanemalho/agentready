use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::analyzer::analyze_repository;
use crate::harness::{HarnessFilter, analyze_harness_readiness};
use crate::report::{
    render_doctor, render_harness_json, render_harness_markdown, render_json, render_markdown,
};
use crate::source::{load_snapshot, parse_target};

#[derive(Debug, Parser)]
#[command(name = "agentready")]
#[command(about = "Scan repositories and check coding-agent harness readiness.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze a repository and write a report.
    Scan(ScanArgs),
    /// Print a quick health summary for a repository.
    Doctor(TargetArgs),
    /// Check whether a repository is ready for coding-agent harnesses.
    Harness(HarnessArgs),
}

#[derive(Debug, Args)]
struct ScanArgs {
    /// Repository to analyze: local path, github:owner/repo, or GitHub URL.
    #[arg(default_value = ".")]
    target: String,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    /// Write the report to a file instead of stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct TargetArgs {
    /// Repository to analyze: local path, github:owner/repo, or GitHub URL.
    #[arg(default_value = ".")]
    target: String,
}

#[derive(Debug, Args)]
struct HarnessArgs {
    /// Repository to analyze: local path, github:owner/repo, or GitHub URL.
    #[arg(default_value = ".")]
    target: String,

    /// Harness to check.
    #[arg(long, value_enum, default_value_t = HarnessSelection::All)]
    harness: HarnessSelection,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    /// Write the report to a file instead of stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Markdown,
    Json,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum HarnessSelection {
    All,
    Codex,
    Claude,
    Gemini,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Scan(args) => run_scan(args),
        Command::Doctor(args) => run_doctor(args),
        Command::Harness(args) => run_harness(args),
    }
}

fn run_scan(args: ScanArgs) -> anyhow::Result<()> {
    let snapshot = load_snapshot(&parse_target(&args.target)?)?;
    let analysis = analyze_repository(&snapshot);
    let content = match args.format {
        OutputFormat::Markdown => render_markdown(&analysis),
        OutputFormat::Json => render_json(&analysis)?,
    };

    write_report(content, args.output)
}

fn run_doctor(args: TargetArgs) -> anyhow::Result<()> {
    let snapshot = load_snapshot(&parse_target(&args.target)?)?;
    let analysis = analyze_repository(&snapshot);
    println!("{}", render_doctor(&analysis));
    Ok(())
}

fn run_harness(args: HarnessArgs) -> anyhow::Result<()> {
    let snapshot = load_snapshot(&parse_target(&args.target)?)?;
    let report = analyze_harness_readiness(&snapshot, args.harness.into());
    let content = match args.format {
        OutputFormat::Markdown => render_harness_markdown(&report),
        OutputFormat::Json => render_harness_json(&report)?,
    };

    write_report(content, args.output)
}

fn write_report(content: String, output: Option<PathBuf>) -> anyhow::Result<()> {
    if let Some(output) = output {
        fs::write(&output, content)
            .with_context(|| format!("failed to write report to {}", output.display()))?;
    } else {
        println!("{content}");
    }

    Ok(())
}

impl From<HarnessSelection> for HarnessFilter {
    fn from(value: HarnessSelection) -> Self {
        match value {
            HarnessSelection::All => Self::All,
            HarnessSelection::Codex => Self::Codex,
            HarnessSelection::Claude => Self::Claude,
            HarnessSelection::Gemini => Self::Gemini,
        }
    }
}
