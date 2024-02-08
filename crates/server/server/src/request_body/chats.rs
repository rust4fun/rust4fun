use rust_study_shared as shared;

use serde::{Deserialize, Serialize};
use shared::UserId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostChatMessage {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostChatRoom {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PutChatRoom {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostChatMember {
    pub user_id: UserId,
}
