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
        let spec_paths = vec![env::var("API_SPEC_PATH")?, env::var("AUTH_SPEC_PATH")?];
        let codegen_paths = vec![
            env::var("API_CODEGEN_PATH")?,
            env::var("AUTH_CODEGEN_PATH")?,
        ];

        for (s, c) in spec_paths.into_iter().zip(codegen_paths) {
            println!("{s} {c}");
            code_generate(&s, &c)?;
        }

        Ok(())
    }
}

fn code_generate(spec_path: &str, codegen_path: &str) -> Result<()> {
    let base = env::var("WORKSPACE_PATH")?;
    let spec = {
        let mut path = PathBuf::from(&base);
        path.push(spec_path);

        let file = File::open(path)?;
        serde_yaml::from_reader(file)?
    };

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
    out_file.push(codegen_path);

    fs::write(out_file, content)?;

    Ok(())
}
