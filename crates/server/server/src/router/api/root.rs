use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::State;
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User(shared::User);

pub fn router() -> Router {
    Router::new().route("/", get(me))
}

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
pub async fn me(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let id = auth_user.id();

    let repo = db::UserRepository::new(state.db());
    let user = repo.find_by_id(id).await?;

    Ok(Json(User::from(user)))
}

impl From<db::UserEntity> for User {
    fn from(value: db::UserEntity) -> Self {
        let user = shared::User::new(value.name);

        User(user)
    }
}
