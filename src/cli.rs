use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::analyzer::analyze_repository;
use crate::harness::{HarnessFilter, analyze_harness_readiness};
use crate::report::{
    render_doctor, render_harness_json, render_harness_markdown, render_json, render_markdown,
};

#[derive(Debug, Parser)]
#[command(name = "repolens")]
#[command(about = "Scan repositories and check coding-agent harness readiness.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze a local repository and write a report.
    Scan(ScanArgs),
    /// Print a quick health summary for a local repository.
    Doctor(PathArgs),
    /// Check whether a repository is ready for coding-agent harnesses.
    Harness(HarnessArgs),
}

#[derive(Debug, Args)]
struct ScanArgs {
    /// Repository path to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    /// Write the report to a file instead of stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct PathArgs {
    /// Repository path to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(Debug, Args)]
struct HarnessArgs {
    /// Repository path to analyze.
    #[arg(default_value = ".")]
    path: PathBuf,

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
    let analysis = analyze_repository(&args.path)?;
    let content = match args.format {
        OutputFormat::Markdown => render_markdown(&analysis),
        OutputFormat::Json => render_json(&analysis)?,
    };

    if let Some(output) = args.output {
        fs::write(&output, content)
            .with_context(|| format!("failed to write report to {}", output.display()))?;
    } else {
        println!("{content}");
    }

    Ok(())
}

fn run_doctor(args: PathArgs) -> anyhow::Result<()> {
    let analysis = analyze_repository(args.path)?;
    println!("{}", render_doctor(&analysis));
    Ok(())
}

fn run_harness(args: HarnessArgs) -> anyhow::Result<()> {
    let report = analyze_harness_readiness(args.path, args.harness.into())?;
    let content = match args.format {
        OutputFormat::Markdown => render_harness_markdown(&report),
        OutputFormat::Json => render_harness_json(&report)?,
    };

    if let Some(output) = args.output {
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
