use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::Serialize;
use shared::{ChatMessageId, ChatRoomId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ChatMessageEntity {
    pub id: ChatMessageId,
    pub room_id: ChatRoomId,
    pub content: String,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputChatMessageEntity {
    pub room_id: ChatRoomId,
    pub content: String,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
}

pub struct ChatMessageRepository(Arc<DbConnector>);

impl ChatMessageRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(
        &self,
        id: ChatMessageId,
        input: InputChatMessageEntity,
    ) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO chat_messages
                    (id, room_id, content, user_id, created_at)
                values
                    ($1::UUID, $2::UUID, $3, $4::UUID, $5)
            "#,
            id.id(),
            input.room_id.id(),
            input.content,
            input.user_id.id(),
            input.created_at,
        )
        .execute(&pool)
        .await
        .map_err(Error::Database)?;

        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("chat_messagess".into()));
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: ChatMessageId) -> Result<ChatMessageEntity, Error> {
        let pool = self.0.get_pool();

        let article = sqlx::query_as!(
            ChatMessageEntity,
            r#"
                SELECT *
                FROM chat_messages
                WHERE id = $1::UUID
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(article)
    }

    pub async fn list_by_room_id(
        &self,
        room_id: ChatRoomId,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatMessageEntity>, Error> {
        let pool = self.0.get_pool();

        let articles = sqlx::query_as!(
            ChatMessageEntity,
            r#"
                SELECT *
                FROM chat_messages
                WHERE room_id = $1::UUID
                OFFSET $2
                LIMIT $3
            "#,
            room_id.id(),
            offset,
            limit
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(articles)
    }
}
