use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to migrate: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("failed in database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("already existed: {0}")]
    AlreadyExsited(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("unknown: {0}")]
    Unknown(String),
}
