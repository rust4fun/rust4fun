use rust_study_shared as shared;

use crate::error::Error;
use crate::ChatRoomEntity;
use crate::DbConnector;
use crate::UserEntity;
use chrono::NaiveDateTime;
use derive_new::new;
use shared::{ChatRoomId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow)]
pub struct ChatMemberEntity {
    pub room_id: ChatRoomId,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputChatMemberEntity {
    pub room_id: ChatRoomId,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
}

pub struct ChatMemberRepository(Arc<DbConnector>);

impl ChatMemberRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: InputChatMemberEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO chat_members
                    (room_id, user_id, created_at)
                values
                    ($1::UUID, $2::UUID, $3)
            "#,
            input.room_id.id(),
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

    pub async fn list_users_by_room_id(
        &self,
        room_id: ChatRoomId,
    ) -> Result<Vec<ChatRoomEntity>, Error> {
        let pool = self.0.get_pool();

        let articles = sqlx::query_as!(
            ChatRoomEntity,
            r#"
                SELECT
                    chat_rooms.id
                    , chat_rooms.name
                    , chat_rooms.description
                    , chat_rooms.created_by
                    , chat_rooms.created_at
                    , chat_rooms.updated_at
                FROM chat_members
                    INNER JOIN chat_rooms ON chat_rooms.id = chat_members.room_id
                WHERE chat_members.room_id = $1::UUID
            "#,
            room_id.id()
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(articles)
    }

    pub async fn list_rooms_by_user_id(
        &self,
        user_id: ChatRoomId,
    ) -> Result<Vec<UserEntity>, Error> {
        let pool = self.0.get_pool();

        let articles = sqlx::query_as!(
            UserEntity,
            r#"
                SELECT
                    users.id
                    , users.name
                FROM chat_members
                    INNER JOIN users ON users.id = chat_members.user_id
                WHERE chat_members.user_id = $1::UUID
            "#,
            user_id.id()
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(articles)
    }
}
