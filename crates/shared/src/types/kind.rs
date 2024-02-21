use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::string::ToString;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Hash, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum PlanetKind {
    Text,
    Voice,
}

impl ToString for PlanetKind {
    fn to_string(&self) -> String {
        match self {
            PlanetKind::Text => "text".to_string(),
            PlanetKind::Voice => "voice".to_string(),
        }
    }
}
