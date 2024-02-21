use rust_study_shared as shared;

use serde::{Deserialize, Serialize};
use shared::SphereId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostPlanetMessage {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostPlanet {
    pub sphere_id: SphereId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PutPlanetRoom {
    pub name: String,
    pub description: Option<String>,
}
