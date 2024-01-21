use rust_study_client as client;

pub use client::types;
pub use client::{Client, ClientArticlesExt};

use std::env;

pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn new() -> Self {
        let base = env::var("API_URL").unwrap_or("https://rust4fun.shuttleapp.rs".to_string());

        Self {
            client: Client::new(&base),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
