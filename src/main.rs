use clap::Parser;
use repolens::cli::{Cli, run};

fn main() -> anyhow::Result<()> {
    run(Cli::parse())
}
