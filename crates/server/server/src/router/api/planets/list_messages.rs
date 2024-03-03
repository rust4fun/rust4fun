use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::query::Pagenations;
use crate::response::ListResponse;
use crate::State;
use axum::{extract::Path, extract::Query, response::IntoResponse, Extension, Json};
use shared::PlanetId;
use std::sync::Arc;

pub async fn handler(
    Path(planet_id): Path<PlanetId>,
    Query(pagenations): Query<Pagenations>,
    _auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::PlanetMessageRepository::new(state.db());

    let message = repo
        .list_users_by_planet(planet_id, pagenations.clone().into())
        .await?;
    let total = message.len();
    let response = ListResponse::new(message, pagenations.offset, pagenations.limit, total as u64);

    Ok(Json(response))
}
