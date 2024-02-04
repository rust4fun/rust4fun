use rust_study_client as client;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("client: {0}")]
    Client(#[from] client::Error),
}
