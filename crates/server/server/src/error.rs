use rust_study_auth as auth;
use rust_study_db_connector as db;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    AuthError(#[from] auth::Error),

    #[error("{0}")]
    RequiredAuthorization(String),

    #[error("{0}")]
    DbError(#[from] db::Error),

    #[error("not found secrets: {0}")]
    NotFoundSecrets(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("unknown: {0}")]
    Unknown(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{self}");
        let status = match self {
            Error::AuthError(auth::Error::DecodeInvalidJwt(_)) => StatusCode::UNAUTHORIZED,
            Error::RequiredAuthorization(_) => StatusCode::UNAUTHORIZED,
            Error::DbError(db::Error::AlreadyExsited(_)) => StatusCode::CONFLICT,
            Error::DbError(db::Error::NotFound(_)) => StatusCode::NOT_FOUND,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
