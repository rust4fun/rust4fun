// use rust_study_db_connector as db;
// use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::State;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn handler(
    _auth_user: Extension<AuthUser>,
    _state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    Ok(StatusCode::OK)
}
