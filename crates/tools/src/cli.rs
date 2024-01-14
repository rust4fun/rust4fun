mod generate_client;
mod generate_specs;

use generate_client::GenerateClient;
use anyhow::Result;
use clap::{Subcommand, Parser};

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
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Subcommands::GenerateClient(args) => args.run()?
        }

        Ok(())
    }
}
