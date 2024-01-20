use rust_study_db_connector as db;
use rust_study_shared as shared;

pub mod router;

use db::DbConnector;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Article(shared::Article);

pub struct State {
    db: Arc<DbConnector>,
}

impl State {
    pub async fn init(pool: PgPool) -> Self {
        let db = Arc::new(DbConnector::new(pool));

        db.migration().await;

        Self { db }
    }

    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }
}
