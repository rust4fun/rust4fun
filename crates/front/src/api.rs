use rust_study_client as client;

pub use client::types;
pub use client::{Client, ClientArticlesExt};

pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::new("http://localhost:8080"),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
