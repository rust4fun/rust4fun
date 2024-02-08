use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::request_body::PostChatMessage;
use crate::ws::ChatRoom;
use crate::State;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use shared::{ChatMessageId, ChatRoomId};
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn handler(
    Path(room_id): Path<ChatRoomId>,
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<PostChatMessage>,
) -> Result<impl IntoResponse, Error> {
    let now = Utc::now().naive_utc();
    let new_id = ChatMessageId::new_v4();
    let input = db::InputChatMessageEntity::new(room_id.clone(), body.content, auth_user.id(), now);
    let repo = db::ChatMessageRepository::new(state.db());

    repo.create(new_id.clone(), input).await?;
    let message = repo.find_by_id(new_id).await?;

    // websocket で送る
    // TODO: この処理を共通化したい
    let rooms = state.chat_rooms();
    let chat_room = {
        let mut rooms = rooms.lock().await;
        match rooms.inner().get(&room_id) {
            Some(v) => v.clone(),
            None => {
                let (tx, _) = broadcast::channel(10);
                let chat_room = ChatRoom::new(tx);
                rooms.inner_mut().insert(room_id, chat_room.clone());
                chat_room
            }
        }
    };

    chat_room.send(message.content.clone());

    Ok(Json(message))
}
