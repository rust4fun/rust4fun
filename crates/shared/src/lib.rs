use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// TODO: client の自動生成でできる types とかぶるのでうまく共存させたい
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
}

impl Article {
    pub fn new(title: String) -> Self {
        Article {
            id: Uuid::new_v4(),
            title,
        }
    }
}
