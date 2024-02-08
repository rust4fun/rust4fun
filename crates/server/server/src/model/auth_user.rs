use rust_study_auth as auth;
use rust_study_db_connector as db;
use rust_study_shared as shared;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: shared::UserId,
    pub name: Option<String>,
    pub claims: auth::Claims,
}

impl AuthUser {
    pub fn id(&self) -> shared::UserId {
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
