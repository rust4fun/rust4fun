use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::request_body::PostPlanet;
use crate::State;
use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use shared::PlanetId;
use shared::PlanetKind;
use std::sync::Arc;

pub async fn handler(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<PostPlanet>,
) -> Result<impl IntoResponse, Error> {
    let now = Utc::now().naive_utc();
    let new_id = PlanetId::new_v4();
    let input = db::InputPlanetEntity::new(
        new_id.clone(),
        body.sphere_id,
        PlanetKind::Text,
        body.name,
        body.description,
        auth_user.id(),
        now,
    );
    let repo = db::PlanetRepository::new(state.db());

    repo.create(input).await?;
    let room = repo.find_by_id(new_id).await?;

    Ok(Json(room))
}
