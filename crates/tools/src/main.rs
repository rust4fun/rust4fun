use clap::Parser;
use rust_study_tools as tools;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tools::cli::Cli::parse().run().await?;

    Ok(())
}
