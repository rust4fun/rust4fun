mod generate_client;

use anyhow::Result;
use clap::{Parser, Subcommand};
use generate_client::GenerateClient;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    GenerateClient(GenerateClient),
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Subcommands::GenerateClient(args) => args.run()?,
        }

        Ok(())
    }
}
