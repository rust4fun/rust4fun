use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::State;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use shared::{PlanetId, PlanetMessageId, PostPlanetMessage};
use std::sync::Arc;

#[allow(unused_imports)]
use shared::PlanetMessage;

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
    {
        tracing::debug!("send: websocket");
        let rooms = state.chat_rooms();
        let mut rooms = rooms.lock().await;
        rooms.send_or_pop(planet_id, message.content.clone())
    };

    Ok(Json(message))
}
