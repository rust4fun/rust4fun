use crate::types::UserId;
use crate::DbConnector;
use std::sync::Arc;

pub struct UserEntity {
    pub id: UserId,
    pub name: String,
}

pub struct UserRepository(Arc<DbConnector>);

impl UserRepository {
    pub fn create() -> UserEntity {
        UserEntity {
            id: UserId::new_v4(),
            name: "aa".to_string(),
        }
    }

    pub fn create_or_get_if_existed() -> UserEntity {
        Self::create()
    }
}
