use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::request_body::PostChatRoom;
use crate::State;
use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use shared::ChatRoomId;
use std::sync::Arc;

pub async fn handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<PostChatRoom>,
) -> Result<impl IntoResponse, Error> {
    let now = Utc::now().naive_utc();
    let new_id = ChatRoomId::new_v4();
    let input = db::InputChatRoomEntity::new(body.name, body.description, auth_user.id(), now);
    let repo = db::ChatRoomRepository::new(state.db());

    repo.create(new_id.clone(), input).await?;
    let room = repo.find_by_id(new_id).await?;

    Ok(Json(room))
}
