use rust_study_db_connector as db;

use crate::error::Error;
use crate::model::{AuthUser, User};
use crate::State;
use axum::{response::IntoResponse, Extension, Json};

use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/me",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "get me", body = User),
        (status = 401, description = "unauhtorization"),
        (status = 404, description = "not found")
    ),
    tag = "root",
)]
pub async fn handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let id = auth_user.id();

    let repo = db::UserRepository::new(state.db());
    let user = repo.find_by_id(id).await?;

    Ok(Json(User::from(user)))
}
