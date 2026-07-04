use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::analyzer::analyze_repository;
use crate::report::{render_doctor, render_json, render_markdown};

#[derive(Debug, Parser)]
#[command(name = "repolens")]
#[command(about = "Scan repositories and generate developer onboarding reports.")]
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

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Markdown,
    Json,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Scan(args) => run_scan(args),
        Command::Doctor(args) => run_doctor(args),
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
