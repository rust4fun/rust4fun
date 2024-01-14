use clap::Parser;
use rust_study_tools as tools;

use anyhow::Result;

fn main() -> Result<()> {
    tools::cli::Cli::parse().run()?;

    Ok(())
}
