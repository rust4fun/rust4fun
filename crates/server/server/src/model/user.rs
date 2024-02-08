use rust_study_db_connector as db;
use rust_study_shared as shared;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User(shared::User);

impl From<db::UserEntity> for User {
    fn from(value: db::UserEntity) -> Self {
        let user = shared::User::new(value.name);

        User(user)
    }
}
