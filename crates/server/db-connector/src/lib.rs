mod error;
mod repository;

pub use repository::articles::{ArticleEntity, ArticleRepository, InputArticleEntity};
pub use repository::planet_members::{
    InputPlanetMessageEntity, PlanetMessageEntity, PlanetMessageRepository,
};
pub use repository::planets::{InputPlanetEntity, PlanetEntity, PlanetRepository};
pub use repository::spheres::{InputSphereEntity, SphereEntity, SphereRepository};
pub use repository::users::{InputUserEntity, InputUserValidateEntity, UserEntity, UserRepository};
pub use repository::Pagination;

pub use error::Error;
use sqlx::PgPool;

pub struct DbConnector {
    pool: PgPool,
    secret: String,
}

impl DbConnector {
    pub fn new(pool: PgPool, secret: String) -> Self {
        Self { pool, secret }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub async fn migration(&self) -> Result<(), Error> {
        tracing::info!("migration starts!");

        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(Error::Migration)?;

        tracing::info!("migration ended!");

        Ok(())
    }
}

pub async fn init(pool: PgPool, secret: String) -> Result<DbConnector, Error> {
    let db = DbConnector::new(pool, secret);
    db.migration().await?;

    Ok(db)
}
