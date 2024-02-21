use rust_study_shared as shared;

use crate::error::Error;
use crate::DbConnector;
use chrono::NaiveDateTime;
use derive_new::new;
use serde::Serialize;
use shared::{PlanetId, PlanetKind, SphereId, UserId};
use std::sync::Arc;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PlanetEntity {
    pub id: PlanetId,
    pub sphere_id: SphereId,
    pub kind: PlanetKind,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, new)]
pub struct InputPlanetEntity {
    pub id: PlanetId,
    pub sphere_id: SphereId,
    pub kind: PlanetKind,
    pub name: String,
    pub description: Option<String>,
    pub created_by: UserId,
    pub created_at: NaiveDateTime,
}

pub struct PlanetRepository(Arc<DbConnector>);

impl PlanetRepository {
    pub fn new(db: Arc<DbConnector>) -> Self {
        Self(db)
    }

    pub async fn create(&self, input: InputPlanetEntity) -> Result<(), Error> {
        let pool = self.0.get_pool();

        let res = sqlx::query!(
            r#"
                INSERT INTO planets
                    (id, sphere_id, kind, name, description, created_by, created_at)
                values
                    ($1::UUID, $2::UUID, $3, $4, $5, $6::UUID, $7)
            "#,
            input.id.id(),
            input.sphere_id.id(),
            input.kind.to_string(),
            input.name,
            input.description,
            input.created_by.id(),
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

    pub async fn find_by_id(&self, id: PlanetId) -> Result<PlanetEntity, Error> {
        let pool = self.0.get_pool();

        let planet = sqlx::query_as!(
            PlanetEntity,
            r#"
                SELECT 
                    id
                    , sphere_id
                    , kind AS "kind: PlanetKind"
                    , name
                    , description
                    , created_by
                    , created_at
                    , updated_at
                FROM planets
                WHERE id = $1::UUID
            "#,
            id.id()
        )
        .fetch_one(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(planet)
    }

    pub async fn list_by_sphere_id(&self, sphere_id: SphereId) -> Result<Vec<PlanetEntity>, Error> {
        let pool = self.0.get_pool();

        let planets = sqlx::query_as!(
            PlanetEntity,
            r#"
                SELECT 
                    id
                    , sphere_id
                    , kind AS "kind: PlanetKind"
                    , name
                    , description
                    , created_by
                    , created_at
                    , updated_at
                FROM planets
                WHERE sphere_id = $1::UUID
            "#,
            sphere_id.id(),
        )
        .fetch_all(&pool)
        .await
        .map_err(Error::Database)?;

        Ok(planets)
    }
}
