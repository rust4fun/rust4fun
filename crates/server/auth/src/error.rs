use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("failed to initialize")]
    KeySetNotInitialize,
    #[error("invalid jwt in encoding: {0}")]
    EoncodeInvalidJwt(jsonwebtoken::errors::Error),
    #[error("invalid jwt in decoding: {0}")]
    DecodeInvalidJwt(jsonwebtoken::errors::Error),
}
