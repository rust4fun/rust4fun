use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::Serialize;
use shared::{SphereId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SphereEntity {
    pub id: SphereId,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputSphereEntity {
    pub id: SphereId,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
}

pub struct SphereRepository(Arc<DbConnector>);

impl SphereRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: InputSphereEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO spheres
                    (id, name, description, created_by, created_at)
                values
                    ($1::UUID, $2, $3, $4::UUID, $5)
            "#,
            input.id.id(),
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

    pub async fn find_by_id(&self, id: SphereId) -> Result<SphereEntity, Error> {
        let pool = self.0.get_pool();

        let article = sqlx::query_as!(
            SphereEntity,
            r#"
                SELECT *
                FROM spheres
                WHERE id = $1::UUID
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(article)
    }

    pub async fn list_by_created_by(&self, created_by: UserId) -> Result<Vec<SphereEntity>, Error> {
        let pool = self.0.get_pool();

        let articles = sqlx::query_as!(
            SphereEntity,
            r#"
                SELECT *
                FROM spheres
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repository::test_utils::test_db_connector;
    use crate::repository::users::test_utils::create_user;
    use sqlx::PgPool;
    use anyhow::Result;
    use chrono::Utc;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create(pool: PgPool) -> Result<()> {
        // ready
        let db = test_db_connector(pool.clone());
        let user_id = create_user(pool).await?;
        
        let repo = SphereRepository::new(Arc::new(db));

        let input = InputSphereEntity::new(
            SphereId::new_v4(), 
            "initila".into(), 
            Some("initila sphere".into()),
            user_id, 
            Utc::now().naive_utc()
        );

        // test
        let res = repo.create(input).await;
        assert!(res.is_ok());
        
        Ok(())
    }
}
