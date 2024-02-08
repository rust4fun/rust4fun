use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::query::Pagenations;
use crate::response::ListResponse;
use crate::State;
use axum::{extract::Path, extract::Query, response::IntoResponse, Extension, Json};
use shared::ChatRoomId;
use std::sync::Arc;

pub async fn handler(
    Path(room_id): Path<ChatRoomId>,
    Query(pagenations): Query<Pagenations>,
    _auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::ChatMessageRepository::new(state.db());

    let message = repo
        .list_by_room_id(room_id, pagenations.offset, pagenations.limit)
        .await?;
    let total = message.len();
    let response = ListResponse::new(message, pagenations.offset, pagenations.limit, total as u64);

    Ok(Json(response))
}
