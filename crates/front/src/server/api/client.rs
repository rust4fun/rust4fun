use rust_study_client as client;

use super::error::Error;
pub use client::types::User;
pub use client::{Client, ClientRootExt, ResponseValue};

use reqwest::header::{HeaderMap, AUTHORIZATION};

pub struct ApiRequester {
    client: Client,
}

impl ApiRequester {
    pub fn new(token: &str) -> Self {
        let base = std::env!("API_URL");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        let default_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client: Client::new_with_client(base, default_client),
        }
    }

    pub async fn me(&self) -> Result<User, Error> {
        let res = self.client.me().send().await?;

        check_response(res)
    }
}

fn check_response<T>(response: ResponseValue<T>) -> Result<T, Error> {
    match response.status().as_u16() {
        200 => Ok(response.into_inner()),
        400 => Err(Error::BadRequest("wrong setting!".into())),
        401 => Err(Error::Unauthorized),
        404 => Err(Error::NotFound),
        _ => Err(Error::Unexpected),
    }
}
