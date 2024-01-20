use rust_study_client as client;

use anyhow::Result;
use clap::{Parser, Subcommand};
use client::{Client, ClientArticlesExt};
use std::env;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct Api {
    #[command(subcommand)]
    request: Requests,
}

impl Api {
    pub async fn run(&self) -> Result<()> {
        match &self.request {
            Requests::FindArticleById(args) => args.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Requests {
    // article
    FindArticleById(FindArticleById),
}

/// ID から記事を取得する
#[derive(Debug, Parser)]
pub struct FindArticleById {
    id: Uuid,
}

impl FindArticleById {
    pub async fn run(&self) -> Result<()> {
        let base = env::var("API_URL")?;

        let client = Client::new(&base);

        let article = client.get_item().id(self.id).send().await?;

        let s = serde_json::to_string(&article.into_inner())?;

        println!("{s}");

        Ok(())
    }
}
