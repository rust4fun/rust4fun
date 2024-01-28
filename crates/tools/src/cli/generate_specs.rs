use rust_study_server as server;

use server::router::{ApiDoc, AuthDoc};
use utoipa::OpenApi;

use anyhow::Result;
use clap::Parser;
use std::fs;

/// server request client を自動生成する
#[derive(Debug, Parser)]
pub struct GenerateSpecs;

impl GenerateSpecs {
    pub fn run(&self) -> Result<()> {
        let content = ApiDoc::openapi().to_yaml()?;
        fs::write("./doc/specifications/api_v1.yml", content)?;

        let content = AuthDoc::openapi().to_yaml()?;
        fs::write("./doc/specifications/root.yml", content)?;
        Ok(())
    }
}
