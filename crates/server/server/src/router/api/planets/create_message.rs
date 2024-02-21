use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::request_body::PostPlanetMessage;
use crate::ws::Channel;
use crate::State;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use shared::{PlanetId, PlanetMessageId};
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn handler(
    Path(planet_id): Path<PlanetId>,
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<PostPlanetMessage>,
) -> Result<impl IntoResponse, Error> {
    let now = Utc::now().naive_utc();
    let new_id = PlanetMessageId::new_v4();
    let input = db::InputPlanetMessageEntity::new(
        new_id.clone(),
        planet_id.clone(),
        body.content,
        auth_user.id(),
        now,
    );
    let repo = db::PlanetMessageRepository::new(state.db());

    repo.create(input).await?;
    let message = repo.find_by_id(new_id).await?;

    // websocket で送る
    // TODO: この処理を共通化したい
    let rooms = state.chat_rooms();
    let chat_room = {
        let mut rooms = rooms.lock().await;
        match rooms.inner().get(&planet_id) {
            Some(v) => v.clone(),
            None => {
                let (tx, _) = broadcast::channel(10);
                let chat_room = Channel::new(tx);
                rooms.inner_mut().insert(planet_id, chat_room.clone());
                chat_room
            }
        }
    };

    chat_room.send(message.content.clone());

    Ok(Json(message))
}
