use crate::{PlanetId, PlanetKind, PlanetMessageId, SphereId, UserId};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanetMessage {
    pub id: PlanetMessageId,
    pub planet_id: PlanetId,
    pub content: String,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Planet {
    pub id: PlanetId,
    pub sphere_id: SphereId,
    pub kind: PlanetKind,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostPlanetMessage {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostPlanet {
    pub sphere_id: SphereId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutPlanetRoom {
    pub name: String,
    pub description: Option<String>,
}
