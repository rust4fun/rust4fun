use rust_study_auth as auth;
use rust_study_db_connector as db;
use rust_study_shared as shared;

pub mod error;
pub mod middleware;
pub mod model;
pub mod query;
pub mod request_body;
pub mod response;
pub mod router;
pub mod ws;

use db::DbConnector;
use error::Error;
use serde::{Deserialize, Serialize};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use ws::ChatRooms;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Article(shared::Article);

pub struct State {
    db: Arc<DbConnector>,
    // TODO: process 中は、削除していないので、 TTL 的な概念を導入する
    chat_rooms: Arc<Mutex<ChatRooms>>,
}

impl State {
    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }

    pub fn chat_rooms(&self) -> Arc<Mutex<ChatRooms>> {
        self.chat_rooms.clone()
    }
}

pub async fn init(secret_store: SecretStore, pool: PgPool) -> Result<State, Error> {
    let auth_secret = secret_store
        .get("AUTH_SECRET")
        .ok_or(Error::NotFoundSecrets("AUTH_SECRET".into()))?;
    auth::init(auth_secret.as_bytes())?;

    let db_secret = secret_store
        .get("DB_SECRET")
        .ok_or(Error::NotFoundSecrets("DB_SECRET".into()))?;
    let db = Arc::new(db::init(pool, db_secret).await?);
    let chat_rooms = Arc::new(Mutex::new(ChatRooms::default()));

    Ok(State { db, chat_rooms })
}
