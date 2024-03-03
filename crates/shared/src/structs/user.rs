use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: client の自動生成でできる types とかぶるのでうまく共存させたい
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, new)]
pub struct Article {
    pub id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, new)]
pub struct User {
    pub name: Option<String>,
}
