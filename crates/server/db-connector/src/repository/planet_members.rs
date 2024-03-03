use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use crate::Pagination;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::Serialize;
use shared::{PlanetId, PlanetMessageId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PlanetMessageEntity {
    pub id: PlanetMessageId,
    pub planet_id: PlanetId,
    pub content: String,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputPlanetMessageEntity {
    pub id: PlanetMessageId,
    pub planet_id: PlanetId,
    pub content: String,
    pub user_id: UserId,
    pub created_at: NaiveDateTime,
}

pub struct PlanetMessageRepository(Arc<DbConnector>);

impl PlanetMessageRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: InputPlanetMessageEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO planet_messages
                    (id, planet_id, content, user_id, created_at)
                values
                    ($1::UUID, $2::UUID, $3, $4::UUID, $5)
            "#,
            input.id.id(),
            input.planet_id.id(),
            input.content,
            input.user_id.id(),
            input.created_at,
        )
        .execute(&pool)
        .await
        .map_err(Error::Database)?;

        if res.rows_affected() == 0 {
            return Err(Error::AlreadyExsited("planets_messagess".into()));
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: PlanetMessageId) -> Result<PlanetMessageEntity, Error> {
        let pool = self.0.get_pool();

        let msgs = sqlx::query_as!(
            PlanetMessageEntity,
            r#"
                SELECT
                    id
                    , planet_id
                    , content
                    , user_id
                    , created_at
                    , updated_at
                FROM
                    planet_messages
                WHERE 
                   id = $1::UUID
            "#,
            id.id(),
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(msgs)
    }

    pub async fn list_users_by_planet(
        &self,
        planet_id: PlanetId,
        pagination: Pagination,
    ) -> Result<Vec<PlanetMessageEntity>, Error> {
        let pool = self.0.get_pool();

        let msgs = sqlx::query_as!(
            PlanetMessageEntity,
            r#"
                SELECT
                    id
                    , planet_id
                    , content
                    , user_id
                    , created_at
                    , updated_at
                FROM
                    planet_messages
                WHERE 
                    planet_messages.planet_id = $1::UUID
                LIMIT $2
                OFFSET $3
            "#,
            planet_id.id(),
            pagination.limit,
            pagination.offset
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(msgs)
    }
}
