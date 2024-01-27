use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("failed to initialize")]
    KeySetNotInitialize,
    #[error("failed to initialize")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}
