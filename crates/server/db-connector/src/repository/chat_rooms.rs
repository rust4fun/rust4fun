use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::Serialize;
use shared::{ChatRoomId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ChatRoomEntity {
    pub id: ChatRoomId,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputChatRoomEntity {
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
}

pub struct ChatRoomRepository(Arc<DbConnector>);

impl ChatRoomRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, id: ChatRoomId, input: InputChatRoomEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO chat_rooms
                    (id, name, description, created_by, created_at)
                values
                    ($1::UUID, $2, $3, $4::UUID, $5)
            "#,
            id.id(),
            input.name,
            input.description,
            input.created_by.id(),
            input.created_at,
        )
        .execute(&pool)
        .await
        .map_err(Error::Database)?;

        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("chat_rooms".into()));
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: ChatRoomId) -> Result<ChatRoomEntity, Error> {
        let pool = self.0.get_pool();

        let article = sqlx::query_as!(
            ChatRoomEntity,
            r#"
                SELECT *
                FROM chat_rooms
                WHERE id = $1::UUID
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(article)
    }

    pub async fn list_by_created_by(
        &self,
        created_by: UserId,
    ) -> Result<Vec<ChatRoomEntity>, Error> {
        let pool = self.0.get_pool();

        let articles = sqlx::query_as!(
            ChatRoomEntity,
            r#"
                SELECT *
                FROM chat_rooms
                WHERE created_by = $1::UUID
            "#,
            created_by.id()
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(articles)
    }
}
