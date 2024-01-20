mod api;
mod generate_client;
mod generate_specs;

use anyhow::Result;
use api::Api;
use clap::{Parser, Subcommand};
use generate_client::GenerateClient;
use generate_specs::GenerateSpecs;

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
    GenerateSpecs(GenerateSpecs),
    Api(Api),
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Subcommands::GenerateClient(args) => args.run()?,
            Subcommands::GenerateSpecs(args) => args.run()?,
            Subcommands::Api(args) => args.run().await?,
        }

        Ok(())
    }
}
