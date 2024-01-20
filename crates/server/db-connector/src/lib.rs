mod error;
mod repository;
mod types;

pub use repository::UserRepository;
pub use repository::{ArticleEntity, ArticleRepository, InputArticleEntity};

pub use types::{ArticleId, UserId};

use sqlx::PgPool;

pub struct DbConnector {
    pool: PgPool,
}

impl DbConnector {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn migration(&self) {
        let res = sqlx::migrate!("./migrations").run(&self.pool).await;

        tracing::info!("{res:?}");
    }
}
