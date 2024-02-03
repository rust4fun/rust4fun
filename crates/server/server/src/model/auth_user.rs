use rust_study_auth as auth;
use rust_study_db_connector as db;

use db::UserId;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: UserId,
    pub name: Option<String>,
    pub claims: auth::Claims,
}

impl AuthUser {
    pub fn id(&self) -> UserId {
        self.id.clone()
    }
}

impl From<(db::UserEntity, auth::Claims)> for AuthUser {
    fn from((user, claims): (db::UserEntity, auth::Claims)) -> Self {
        Self {
            id: user.id,
            name: user.name,
            claims,
        }
    }
}
