use rust_study_client as client;

pub use client::types;
pub use client::{Client, ClientArticlesExt};

use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;

pub struct ApiRequester {
    client: Client,
}

impl ApiRequester {
    pub fn new(token: &str) -> Self {
        // TODO: 供給先をどうするか
        let base = env::var("API_URL").unwrap_or("http://localhost:8080".to_string());
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let default_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client: Client::new_with_client(&base, default_client),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
