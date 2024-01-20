use rust_study_server as server;

use server::router::ApiDoc;
use utoipa::OpenApi;

use anyhow::Result;
use clap::Parser;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/// server request client を自動生成する
#[derive(Debug, Parser)]
pub struct GenerateClient {
    #[arg(short, long)]
    from_file: bool,
}

impl GenerateClient {
    pub fn run(&self) -> Result<()> {
        let base = env::var("WORKSPACE_PATH")?;

        let spec = if self.from_file {
            let mut spec_path = PathBuf::from(&base);
            spec_path.push(env::var("SPEC_PATH")?);

            let file = File::open(spec_path)?;
            serde_yaml::from_reader(file)?
        } else {
            let content = ApiDoc::openapi().to_yaml()?;
            serde_yaml::from_str(&content)?
        };

        // let content = ApiDoc::openapi().to_yaml()?;
        // std::fs::write("./doc/specifications/api_v1.yml", content).expect("Err generate yaml");

        let mut setting = progenitor::GenerationSettings::new();
        setting.with_tag(progenitor::TagStyle::Separate);
        setting.with_interface(progenitor::InterfaceStyle::Builder);

        let mut generator = progenitor::Generator::new(&setting);
        let tokens = generator.generate_tokens(&spec)?;
        let ast = syn::parse2(tokens)?;
        let content = prettyplease::unparse(&ast);
        // TODO: 毎回刺青されるファイルの import を変更しないといけないのでなんとかしたい
        let content = content.replace("progenitor_client::", "progenitor::progenitor_client::");

        let mut out_file = PathBuf::from(&base);
        out_file.push(env::var("CODEGEN_PATH")?);

        fs::write(out_file, content)?;

        Ok(())
    }
}
