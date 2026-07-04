use agentready::cli::{Cli, run};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    run(Cli::parse())
}
