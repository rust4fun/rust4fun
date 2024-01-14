use anyhow::Result;
use clap::Parser;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/// server request client を自動生成する
#[derive(Debug, Parser)]
pub struct GenerateClient;

impl GenerateClient {
    pub fn run(&self) -> Result<()> {
        let base = env::var("workspace_path")?;

        let mut spec_path = PathBuf::from(&base);
        spec_path.push(env::var("spec_path")?);

        let file = File::open(spec_path)?;
        let spec = serde_yaml::from_reader(file)?;

        let mut generator = progenitor::Generator::default();
        let tokens = generator.generate_tokens(&spec)?;
        let ast = syn::parse2(tokens)?;
        let content = prettyplease::unparse(&ast);
        // TODO: 毎回刺青されるファイルの import を変更しないといけないのでなんとかしたい
        let content = content.replace("progenitor_client::", "progenitor::progenitor_client::");

        let mut out_file = PathBuf::from(&base);
        out_file.push(env::var("codegen_path")?);

        fs::write(out_file, content)?;

        Ok(())
    }
}
